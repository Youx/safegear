<script setup lang="ts">
import { useAppSettingsStore } from '@/stores'
import Button from 'primevue/button'
const appSettings = useAppSettingsStore()
</script>
<template>
  <Button v-if="appSettings.darkMode" icon="pi pi-sun" @click="appSettings.toggle()" />
  <Button v-else icon="pi pi-moon" @click="appSettings.toggle()" />
</template>
