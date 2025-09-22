<script setup lang="ts">
import { RouterView } from 'vue-router'
import { ref } from 'vue'
const showMenu = ref(false)
</script>

<template>
  <v-app>
    <v-main id="main-content">
      <v-app-bar :elevation="2" density="compact">
        <template v-slot:prepend>
          <v-app-bar-nav-icon icon="mdi-menu" @click="showMenu = !showMenu"></v-app-bar-nav-icon>
        </template>
      </v-app-bar>
      <v-navigation-drawer id="my-drawer" v-model="showMenu">
        <v-list>
          <v-list-item link to="/" @click="showMenu = false">
            <v-list-item-title>Home</v-list-item-title>
          </v-list-item>
          <v-list-item link to="/about" @click="showMenu = false">
            <v-list-item-title>About</v-list-item-title>
          </v-list-item>
          <v-list-item link to="/games" @click="showMenu = false">
            <v-list-item-title>Games</v-list-item-title>
          </v-list-item>
        </v-list>
      </v-navigation-drawer>
      <v-container>
        <RouterView />
      </v-container>
    </v-main>
  </v-app>
</template>

<style scoped>
header {
  line-height: 1.5;
}

.v-app {
  height: 100%;
}

.logo {
  display: block;
  margin: 0 auto 2rem;
}
.v-list {
  display: flex;
  flex-direction: column;
  padding: 0;
  margin: 0;
}

nav {
  width: 100%;
  font-size: 12px;
  text-align: center;
  margin-top: 2rem;
}

nav a.router-link-exact-active {
  color: var(--color-text);
}

nav a.router-link-exact-active:hover {
  background-color: transparent;
}

nav a {
  display: inline-block;
  padding: 0 1rem;
  border-left: 1px solid var(--color-border);
}

nav a:first-of-type {
  border: 0;
}

#main-content {
  display: flex;
  flex-direction: column;
  width: 100%;
  background-color: aqua;
}

@media (min-width: 1024px) {
  header {
    display: flex;
    place-items: center;
    padding-right: calc(var(--section-gap) / 2);
  }

  .logo {
    margin: 0 2rem 0 0;
  }

  header .wrapper {
    display: flex;
    place-items: flex-start;
    flex-wrap: wrap;
  }

  nav {
    text-align: left;
    margin-left: -1rem;
    font-size: 1rem;

    padding: 1rem 0;
    margin-top: 1rem;
  }
}
</style>
