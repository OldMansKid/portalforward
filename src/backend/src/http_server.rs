use std::sync::Arc;

use argon2::{PasswordHash, PasswordVerifier};
use http_body_util::BodyExt;
use hyper_util::rt::TokioIo;
use rand::RngCore;

pub async fn start_server() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8001").await;
    if let Err(e) = listener {
        log::error!("Failed to bind to address: {}", e);
        std::process::exit(1);
    }
    let listener = listener.unwrap();
    let communicator = SqliteCommunicator::new();
    let mut secret = [0u8; 32];
    rand::rng().fill_bytes(&mut secret);
    let secret = Arc::new(secret);
    loop {
        let incoming = listener.accept().await;
        if let Err(e) = incoming {
            log::error!("Failed to accept connection: {}", e);
            continue;
        }
        let (stream, _) = incoming.unwrap();
        let communicator = communicator.clone();
        tokio::spawn(handle_connection(stream, communicator, secret.clone()));
    }
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    communicator: SqliteCommunicator,
    secret: Arc<[u8; 32]>,
) {
    let io = TokioIo::new(stream);
    let service = AuthService {
        communicator,
        secret,
    };
    let http = hyper::server::conn::http1::Builder::new();
    if let Err(e) = http.serve_connection(io, service).await {
        log::error!("Error serving connection: {}", e);
    }
}

struct AuthService {
    communicator: SqliteCommunicator,
    secret: Arc<[u8; 32]>,
}

impl hyper::service::Service<hyper::Request<hyper::body::Incoming>> for AuthService {
    type Response = hyper::Response<AuthBody>;

    type Error = AuthError;

    type Future = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn call(&self, req: hyper::Request<hyper::body::Incoming>) -> Self::Future {
        match (req.method(), req.uri().path()) {
            (&hyper::Method::POST, "/login") => {
                let communicator = self.communicator.clone();
                Box::pin(login(req, communicator, self.secret.clone()))
            }
            _ => {
                if let Err(_) = check_request_token(&req, self.secret.as_ref()) {
                    let response_body = AuthBody::new_constant(b"Unauthorized");
                    let response = hyper::Response::builder()
                        .status(hyper::StatusCode::UNAUTHORIZED)
                        .body(response_body)
                        .unwrap();
                    Box::pin(async move { Ok(response) })
                } else {
                    let response_body = AuthBody::new_constant(b"Hello, authenticated user!");
                    let response = hyper::Response::builder()
                        .status(hyper::StatusCode::OK)
                        .body(response_body)
                        .unwrap();
                    Box::pin(async move { Ok(response) })
                }
            }
        }
    }
}

enum AuthBody {
    Constant(ConstantBody),
    StringBody(Option<String>),
}

impl hyper::body::Body for AuthBody {
    type Data = hyper::body::Bytes;

    type Error = AuthError;

    fn poll_frame(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Result<hyper::body::Frame<Self::Data>, Self::Error>>> {
        match *self {
            AuthBody::Constant(ref mut items) => {
                if items.done {
                    std::task::Poll::Ready(None)
                } else {
                    let chunk = hyper::body::Bytes::from_static(items.data);
                    items.done = true;
                    std::task::Poll::Ready(Some(Ok(hyper::body::Frame::data(chunk))))
                }
            }
            AuthBody::StringBody(ref mut items) => {
                if items.is_none() {
                    std::task::Poll::Ready(None)
                } else {
                    let chunk = hyper::body::Bytes::from(items.take().unwrap());
                    std::task::Poll::Ready(Some(Ok(hyper::body::Frame::data(chunk))))
                }
            }
        }
    }
    fn is_end_stream(&self) -> bool {
        match *self {
            AuthBody::Constant(ref items) => items.done,
            AuthBody::StringBody(ref items) => items.is_none(),
        }
    }

    fn size_hint(&self) -> hyper::body::SizeHint {
        match *self {
            AuthBody::Constant(ref items) => {
                if items.done {
                    hyper::body::SizeHint::with_exact(0)
                } else {
                    hyper::body::SizeHint::with_exact(items.data.len() as u64)
                }
            }
            AuthBody::StringBody(ref items) => {
                if let Some(s) = items {
                    hyper::body::SizeHint::with_exact(s.len() as u64)
                } else {
                    hyper::body::SizeHint::with_exact(0)
                }
            }
        }
    }
}

impl AuthBody {
    fn new_string(s: String) -> Self {
        AuthBody::StringBody(Some(s))
    }
    fn new_constant(data: &'static [u8]) -> Self {
        AuthBody::Constant(ConstantBody::new(data))
    }
}

struct ConstantBody {
    data: &'static [u8],
    done: bool,
}

impl ConstantBody {
    fn new(data: &'static [u8]) -> Self {
        Self { data, done: false }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum AuthError {
    IOError,
    SqlError,
    AuthError,
    Expired,
}
impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::IOError => write!(f, "I/O Error"),
            AuthError::SqlError => write!(f, "SQL Error"),
            AuthError::AuthError => write!(f, "Authentication Error"),
            AuthError::Expired => write!(f, "Token Expired"),
        }
    }
}
impl std::error::Error for AuthError {}

async fn login(
    request: hyper::Request<hyper::body::Incoming>,
    communicator: SqliteCommunicator,
    secret: Arc<[u8; 32]>,
) -> Result<hyper::Response<AuthBody>, AuthError> {
    let mut buffer: [u8; 4096] = [0; 4096];
    let (parts, body) = request.into_parts();
    let progress = load_body(body, &mut buffer).await?;
    if progress == 0 {
        log::info!("Empty request body");
        return Err(AuthError::AuthError);
    }
    let request_body: LoginRequest = match serde_json::from_slice(&buffer[..progress]) {
        Ok(body) => body,
        Err(e) => {
            log::info!("Failed to parse JSON body: {}", e);
            return Err(AuthError::IOError);
        }
    };

    let result = communicator
        .login_user(&request_body.username, request_body.password)
        .await;
    if let Err(e) = result {
        log::info!("Login failed: {}", e);
        let response_body = AuthBody::new_constant(b"Unauthorized");
        let response = hyper::Response::builder()
            .status(hyper::StatusCode::UNAUTHORIZED)
            .body(response_body)
            .unwrap();
        return Ok(response);
    }
    let token_result = generate_jwt(&request_body.username, secret.as_ref());
    if let Err(e) = token_result {
        log::error!("Failed to generate JWT: {}", e);
        return Err(AuthError::IOError);
    }
    let token = token_result.unwrap();
    let response_body = AuthBody::new_string(token);
    let response = hyper::Response::builder()
        .status(hyper::StatusCode::OK)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .body(response_body)
        .unwrap();
    Ok(response)
}

async fn load_body(mut body: hyper::body::Incoming, buffer: &mut [u8]) -> Result<usize, AuthError> {
    let mut progress = 0;
    while let Some(frame) = body.frame().await {
        if let Err(e) = frame {
            log::info!("Error reading body frame: {}", e);
            return Err(AuthError::IOError);
        }
        let frame = frame.unwrap();
        if frame.is_data() {
            let data = frame.into_data().unwrap();
            let data_len = data.len();
            if progress + data_len > buffer.len() {
                log::info!("Request body too large");
                return Err(AuthError::IOError);
            }
            buffer[progress..progress + data_len].copy_from_slice(&data);
            progress += data_len;
        }
    }
    Ok(progress)
}

fn get_password_hash(conn: &rusqlite::Connection, username: &str) -> Result<String, AuthError> {
    let mut stmt = match conn.prepare("SELECT password_hash FROM users WHERE username = ?1") {
        Ok(s) => s,
        Err(e) => {
            log::error!("Failed to prepare SQL statement: {}", e);
            return Err(AuthError::SqlError);
        }
    };
    let mut rows = match stmt.query([username]) {
        Ok(r) => r,
        Err(e) => {
            log::error!("Failed to execute SQL query: {}", e);
            return Err(AuthError::SqlError);
        }
    };
    if let Some(row) = match rows.next() {
        Ok(r) => r,
        Err(e) => {
            log::error!("Failed to fetch SQL row: {}", e);
            return Err(AuthError::SqlError);
        }
    } {
        let hash: String = match row.get(0) {
            Ok(h) => h,
            Err(e) => {
                log::error!("Failed to get password hash from row: {}", e);
                return Err(AuthError::SqlError);
            }
        };
        Ok(hash)
    } else {
        log::info!("Username not found: {}", username);
        Err(AuthError::AuthError)
    }
}

#[derive(Debug)]
enum SqliteRequest {
    LoginUser {
        username: String,
        password: String,
        responder: tokio::sync::oneshot::Sender<Result<(), AuthError>>,
    },
}

#[derive(Clone)]
struct SqliteCommunicator {
    sender: tokio::sync::mpsc::Sender<SqliteRequest>,
}

impl SqliteCommunicator {
    fn new() -> SqliteCommunicator {
        let (sender, receiver) = tokio::sync::mpsc::channel(16);
        tokio::spawn(sqlite_worker(receiver));
        Self { sender }
    }

    async fn login_user(&self, username: &str, password: String) -> Result<(), AuthError> {
        let (responder, response_receiver) = tokio::sync::oneshot::channel();
        let request = SqliteRequest::LoginUser {
            username: username.to_string(),
            password,
            responder,
        };
        if let Err(e) = self.sender.send(request).await {
            log::error!("Failed to send SQLite request: {}", e);
            return Err(AuthError::IOError);
        }
        match response_receiver.await {
            Ok(result) => result,
            Err(e) => {
                log::error!("Failed to receive SQLite response: {}", e);
                Err(AuthError::IOError)
            }
        }
    }
}

async fn sqlite_worker(mut receiver: tokio::sync::mpsc::Receiver<SqliteRequest>) {
    let connection = match rusqlite::Connection::open("auth.db") {
        Ok(conn) => conn,
        Err(e) => {
            log::error!("Failed to open SQLite database: {}", e);
            return;
        }
    };
    while let Some(request) = receiver.recv().await {
        match request {
            SqliteRequest::LoginUser {
                username,
                password,
                responder,
            } => {
                let result = handle_login(&connection, &username, &password);
                let _ = responder.send(result);
            }
        }
    }
}

fn handle_login(
    conn: &rusqlite::Connection,
    username: &str,
    password: &str,
) -> Result<(), AuthError> {
    let stored_hash = get_password_hash(conn, username)?;
    let parsed_hash = PasswordHash::new(&stored_hash);
    if let Err(e) = parsed_hash {
        log::error!("error parsing stored hash for {}: {}", username, e);
        return Err(AuthError::SqlError);
    }
    let parsed_hash = parsed_hash.unwrap();
    let argon = argon2::Argon2::default();
    if let Err(e) = argon.verify_password(password.as_bytes(), &parsed_hash) {
        log::info!(
            "failed to verify user {} entered password {}: {}",
            username,
            password,
            e
        );
        return Err(AuthError::AuthError);
    }
    return Ok(());
}

#[derive(serde::Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

impl Claims {
    fn is_expired(&self) -> bool {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        self.exp < current_time
    }
}

fn generate_jwt(username: &str, secret: &[u8]) -> Result<String, AuthError> {
    let expiration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 3600; // 1 hour expiration
    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
    };
    let header = jsonwebtoken::Header::default();
    match jsonwebtoken::encode(
        &header,
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret),
    ) {
        Ok(token) => Ok(token),
        Err(e) => {
            log::error!("Failed to generate JWT: {}", e);
            Err(AuthError::IOError)
        }
    }
}

fn check_request_token(
    req: &hyper::Request<hyper::body::Incoming>,
    secret: &[u8],
) -> Result<Claims, AuthError> {
    let auth_header = match req.headers().get(hyper::header::AUTHORIZATION) {
        Some(h) => h.to_str().map_err(|_| AuthError::AuthError)?,
        None => return Err(AuthError::AuthError),
    };
    if !auth_header.starts_with("Bearer ") {
        return Err(AuthError::AuthError);
    }
    let token = &auth_header[7..];
    verify_jwt(token, secret)
}

fn verify_jwt(token: &str, secret: &[u8]) -> Result<Claims, AuthError> {
    let token_data = jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret),
        &jsonwebtoken::Validation::default(),
    )
    .map_err(|e| {
        log::info!("Failed to decode JWT: {}", e);
        AuthError::AuthError
    })?;
    if token_data.claims.is_expired() {
        log::info!("JWT is expired");
        return Err(AuthError::AuthError);
    }
    Ok(token_data.claims)
}
