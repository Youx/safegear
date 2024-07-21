<script setup lang="ts">
import { RouterView } from 'vue-router'
import NavBar from './components/NavBar.vue'
</script>

<template>
  <div class="grid">
    <div class="col-12">
      <NavBar></NavBar>
    </div>
    <div class="col-12">
      <RouterView />
    </div>
  </div>
</template>
