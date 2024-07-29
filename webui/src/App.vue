<script setup lang="ts">
import { RouterView } from 'vue-router'
import NavBar from './components/NavBar.vue'
import { useAppInfoStore, useAppSettingsStore } from './stores';
import LoginView from './views/LoginView.vue';

const appInfoStore = useAppInfoStore()
const appSettings = useAppSettingsStore()
</script>
<template>
  <div class="flex flex-column min-h-screen p-2">
    <div class="flex flex-none">
      <NavBar></NavBar>
    </div>
    <div class="flex flex-grow-1 justify-content-center align-content-center">
      <LoginView v-if="!appSettings.jwtToken" />
      <RouterView v-else />
    </div>
    <div class="flex flex-none justify-content-center text-xs"><a href="https://github.com/Youx/safegear">SafeGear</a><span> - {{ appInfoStore.version }} - (c) 2024 Hugo Camboulive</span></div>
  </div>
</template>
