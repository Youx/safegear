<script setup lang="ts">
import { useTagSearchStore, useTagsStore } from '@/stores'
import DataTable, { type DataTableFilterMetaData } from 'primevue/datatable'
import { type Tag } from '@/bindings/Tag'
import Column from 'primevue/column'
import Button from 'primevue/button'
import IconField from 'primevue/iconfield'
import InputIcon from 'primevue/inputicon'
import InputText from 'primevue/inputtext'
import router from '@/router'

let store = useTagsStore()
const filters = useTagSearchStore()
const selectedTags = defineModel('selectedTags', { default: (): Tag[] => []})

</script>

<template>
  <div>
    <DataTable :value="store.tags" v-model:filters="filters.data" :globalFilterFields="['name']" v-model:selection="selectedTags"
    >
      <template #header>
        <div class="flex overflow-hidden gap-3">
          <h2 class="flex-none flex align-items-left justify-content-left">Tags</h2>
          <div class="flex-grow-1 gap-1 align-items-center flex justify-content-left">
            <Button icon="pi pi-plus" rounded raised @click="router.push('/items/new')"></Button>
            <Button
              icon="pi pi-trash"
              rounded
              outlined
              :disabled="selectedTags.length == 0"
              severity="danger"
              size="small"
              aria-label="Delete"
              @click="store.delete(selectedTags.map((t) => t.id))"
            />
          </div>
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
            <Button icon="pi pi-refresh" rounded raised @click="store.refresh()" />
          </div>
        </div>
      </template>
      <Column selectionMode="multiple"></Column>
      <Column field="id" header="#" />
      <Column field="name" header="Name" />
      <Column field="color" header="Color" />
      <Column header="Actions">
        <template #body="slotProps">
          <div class="flex gap-1">
            <Button
              icon="pi pi-trash"
              rounded
              outlined
              severity="danger"
              size="small"
              aria-label="Delete"
              @click="store.delete(slotProps.data.id)"
            />
          </div>
        </template>
      </Column>
    </DataTable>
  </div>
</template>

<style scoped></style>
