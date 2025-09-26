<script lang="ts" setup>
import axios from "axios";
import { ref } from "vue";
import { useRouter } from "vue-router";

const username = ref("");
const password = ref("");
const errorMessage = ref("");
const router = useRouter();
async function handleLogin() {
  try {
    const response = await axios.post("http://localhost:8001/login", {
      username: username.value,
      password: password.value,
    });
    const token = response.data.token;
    localStorage.setItem("jwt_token", token);
    router.push("/games");
  } catch (error) {
    console.error("Login error:", error);
    errorMessage.value = "Login failed. Please check your credentials.";
  }

}
</script>

<template>
  <div class="login-view">
    <label>Login View</label>
    <form @submit.prevent="handleLogin">
      <div>
        <label for="username">Username:</label>
        <input type="text" id="username" name="username" v-model="username" />
      </div>
      <div>
        <label for="password">Password:</label>
        <input type="password" id="password" name="password" v-model="password" />
      </div>
      <button type="submit">Login</button>
    </form>
  </div>
</template>
