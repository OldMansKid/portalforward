<script setup lang="ts"></script>

<template>
  <v-container>
    <v-col cols="12" md="8">
      <v-list>
        <v-list-item>
          <v-list-item-content>
            <v-list-item-title> sample title </v-list-item-title>
            <v-list-item-subtitle> sample subtitle </v-list-item-subtitle>
          </v-list-item-content>
        </v-list-item>
      </v-list>
    </v-col>
    <v-col>
      <v-text-field label="Search" outlined dense></v-text-field>
    </v-col>
  </v-container>
</template>

<style scoped>
.v-container {
  display: flex;
  flex-direction: row;
  gap: 16px;
  padding: 16px;
}
</style>
