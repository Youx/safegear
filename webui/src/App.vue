<script setup lang="ts">
import { RouterView } from 'vue-router'
import NavBar from './components/NavBar.vue'
import { useAppInfoStore } from './stores';

const appInfoStore = useAppInfoStore()
</script>
<template>
  <div class="grid">
    <div class="col-12">
      <NavBar></NavBar>
    </div>
    <div class="col-12">
      <RouterView />
    </div>
    <div class="col-12 flex justify-content-center text-xs"><a href="https://github.com/Youx/safegear">SafeGear</a><span> - {{ appInfoStore.version }} - (c) 2024 Hugo Camboulive</span></div>
  </div>
</template>
