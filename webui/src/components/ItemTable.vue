<script setup lang="ts">
import { useItemSearchStore, useItemsStore, useTagsStore } from '@/stores'
import DataTable, { type DataTableFilterMetaData } from 'primevue/datatable'
import Column from 'primevue/column'
import Tag from 'primevue/tag'
import InputIcon from 'primevue/inputicon'
import InputText from 'primevue/inputtext'
import IconField from 'primevue/iconfield'
import Button from 'primevue/button'

const itemStore = useItemsStore()
const tagStore = useTagsStore()
const filters = useItemSearchStore()
const selectedItems = defineModel('selectedItems')
</script>
<template>
  <DataTable
    v-model:selection="selectedItems"
    :value="itemStore.items"
    :paginator="true"
    :rows="20"
    tableStyle="min-width: 50rem"
    v-model:filters="filters.data"
    :globalFilterFields="['name']"
  >
    <template #header>
      <div class="flex overflow-hidden gap-3">
        <h2 class="flex-grow-1 flex align-items-left justify-content-left">Items</h2>
        <div class="flex-none flex align-items-center justify-content-center">
          <IconField>
            <InputIcon>
              <i class="pi pi-search" />
            </InputIcon>
            <InputText
              v-model="(filters.data['global'] as DataTableFilterMetaData).value"
              placeholder="Keyword Search"
            />
          </IconField>
        </div>
        <div class="flex-none flex align-items-center justify-content-center">
          <Button icon="pi pi-refresh" rounded raised @click="itemStore.refresh()" />
        </div>
      </div>
    </template>
    <Column selectionMode="multiple"></Column>
    <Column field="id" header="#"></Column>
    <Column field="name" header="Name"></Column>
    <Column header="Tags">
      <template #body="slotProps">
        <Tag
          v-for="tag in slotProps.data.tags"
          v-bind:key="tag.id"
          :value="tagStore.getTag(tag).name"
        />
      </template>
    </Column>
    <Column header="Actions">
      <template #body="">
        <div class="flex gap-1">
          <Button
            icon="pi pi-info"
            rounded
            outlined
            severity="secondary"
            size="small"
            aria-label="Details"
          />
          <Button
            icon="pi pi-trash"
            rounded
            outlined
            severity="danger"
            size="small"
            aria-label="Delete"
          />
        </div>
      </template>
    </Column>
  </DataTable>
</template>
