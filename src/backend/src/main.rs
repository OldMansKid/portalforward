use crate::http_server::start_server;

mod http_server;
#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    start_server().await
}
