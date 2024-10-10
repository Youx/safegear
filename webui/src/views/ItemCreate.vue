<script setup lang="ts">
import { useItemsStore, useTagsStore } from '@/stores'
import { useRouter } from 'vue-router'
import AutoComplete, { type AutoCompleteCompleteEvent } from 'primevue/autocomplete'
import InputNumber from 'primevue/inputnumber'
import InputText from 'primevue/inputtext'
import MultiSelect from 'primevue/multiselect'
import Button from 'primevue/button'
import DatePicker from 'primevue/datepicker'
import { ref } from 'vue'

let itemStore = useItemsStore()
let tagStore = useTagsStore()
let router = useRouter()

let formName = defineModel('name', { default: '' })
let formTags = defineModel('tags', { default: () => [] })
let formInspection = defineModel('formInspection', { default: null })
let formSerial = defineModel('formSerial', { default: null })
let formManufacturedOn = defineModel('formManufacturedOn', { default: null })
let formPutIntoServiceOn = defineModel('formPutIntoServiceOn', { default: null })
// name autocompletion
const names = ref(itemStore.names)
function search(event: AutoCompleteCompleteEvent) {
  names.value = itemStore.names.filter((name) => name.includes(event.query))
}

async function submit() {
  await itemStore.create({
    name: formName.value,
    inspection_period_days: formInspection.value,
    serial_number: formSerial.value,
    tags: formTags.value,
    manufactured_on: formManufacturedOn.value,
    put_into_service_on: formPutIntoServiceOn.value,
  })
  router.push('/items')
}
</script>
<template>
  <div class="grid justify-content-center">
    <div class="col-9">
      <h2>New item</h2>
    </div>
    <div class="field grid justify-content-end col-9">
      <label class="col-4 mb-0" for="name">Name</label>
      <AutoComplete
        class="col-6 h-3rem"
        :dropdown="true"
        :suggestions="names"
        v-model="formName"
        @complete="search($event)"
      />
      <div class="col-2"></div>
    </div>
    <div class="field grid justify-content-end col-9">
      <label class="col-4 mb-0" for="daysBetweenChecks">Days between checks</label>
      <div class="col-8">
        <InputNumber class="h-3rem" :showButtons="true" v-model="formInspection" />
      </div>
    </div>
    <div class="field grid justify-content-end col-9">
      <label class="col-4 mb-0" for="daysBetweenChecks">Serial number</label>
      <div class="col-8">
        <InputText class="h-3rem" v-model="formSerial" />
      </div>
    </div>
    <div class="field grid justify-content-end col-9">
      <label class="col-4 mb-0">Manufactured on</label>
      <div class="col-8">
        <DatePicker v-model="formManufacturedOn" dateFormat="dd/mm/yy" />
      </div>
    </div>
    <div class="field grid justify-content-end col-9">
      <label class="col-4 mb-0">Put into service on</label>
      <div class="col-8">
        <DatePicker v-model="formPutIntoServiceOn" dateFormat="dd/mm/yy" />
      </div>
    </div>
    <div class="field grid col-9">
      <label class="col-4 mb-0" for="tags">Tags</label>
      <div class="col-8">
        <MultiSelect
          v-model="formTags"
          display="chip"
          :options="tagStore.tags"
          optionLabel="name"
          optionValue="id"
          filter
          placeholder="Select Tags"
          :maxSelectedLabels="3"
          class="col-6 h-3rem"
        />
      </div>
    </div>
    <div class="flex justify-content-start col-9">
      <div class="col-4 flex justify-content-start">
        <Button severity="secondary" @click="router.back" label="Cancel" />
      </div>
      <div class="col-4 flex justify-content-start">
        <Button @click="submit()" label="Submit" />
      </div>
    </div>
  </div>
</template>
<style lang="postcss">
.p-multiselect-label-container {
  align-content: center !important;
}
</style>
