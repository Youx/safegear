<script setup lang="ts">
import { useAppSettingsStore, useItemSearchStore, useItemsStore, useTagsStore } from '@/stores'
import DataTable, { type DataTableFilterMetaData } from 'primevue/datatable'
import Column from 'primevue/column'
import Tag from 'primevue/tag'
import InputIcon from 'primevue/inputicon'
import InputText from 'primevue/inputtext'
import IconField from 'primevue/iconfield'
import Button from 'primevue/button'
import router from '@/router'

const itemStore = useItemsStore()
const tagStore = useTagsStore()
const filters = useItemSearchStore()
const selectedItems = defineModel('selectedItems')
const appSettings = useAppSettingsStore()

async function refresh() {
  await itemStore.refresh()
  await tagStore.refresh()
}
// trigger a refresh on page load
refresh()

</script>
<template>
  <DataTable v-model:selection="selectedItems" :value="itemStore.items" :paginator="true" :rows="20"
    tableStyle="min-width: 50rem" v-model:filters="filters.data" :globalFilterFields="['name', 'serial_number']">
    <template #header>
      <div class="flex overflow-hidden gap-3">
        <h2 class="flex-none flex align-items-left justify-content-left">Items</h2>
        <div class="flex-grow-1 align-items-center flex justify-content-left">
          <Button v-if="appSettings.canManageItems" icon="pi pi-plus" rounded raised
            @click="router.push('/items/new')"></Button>
        </div>
        <div class="flex-none flex align-items-center justify-content-center">
          <IconField>
            <InputIcon>
              <i class="pi pi-search" />
            </InputIcon>
            <InputText v-model="(filters.data['global'] as DataTableFilterMetaData).value"
              placeholder="Keyword Search" />
          </IconField>
        </div>
        <div class="flex-none flex align-items-center justify-content-center">
          <Button icon="pi pi-refresh" rounded raised @click="refresh()" />
        </div>
      </div>
    </template>
    <Column selectionMode="multiple"></Column>
    <Column field="id" header="#"></Column>
    <Column field="name" header="Name"></Column>
    <Column field="serial_number" header="Serial number"></Column>
    <Column header="Tags">
      <template #body="slotProps">
        <Tag v-for="tag in slotProps.data.tags" v-bind:key="tag.id" :value="tagStore.getTag(tag).name" />
      </template>
    </Column>
    <Column header="Actions">
      <template #body="slotProps">
        <div class="flex gap-1">
          <Button icon="pi pi-info" rounded outlined severity="secondary" size="small" aria-label="Details"
            @click="router.push('/items/details/' + slotProps.data.id)" />
          <Button v-if="appSettings.canManageItems" icon="pi pi-trash" rounded outlined severity="danger" size="small"
            aria-label="Delete" />
        </div>
      </template>
    </Column>
  </DataTable>
</template>
