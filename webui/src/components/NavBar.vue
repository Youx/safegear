<script setup lang="ts">
import type { MenuItem } from 'primevue/menuitem'
import Menubar from 'primevue/menubar'

const items: MenuItem[] = [
  {
    label: 'Items',
    url: '/items'
  }
]
</script>
<template>
  <Menubar :model="items" />
</template>
