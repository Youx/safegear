<script setup lang="ts">
import type { ItemDetails } from '@/bindings/ItemDetails'
import { useItemsStore, useTagsStore } from '@/stores'

import { ref, watch } from 'vue'
import { useRoute } from 'vue-router'
import ProgressSpinner from 'primevue/progressspinner'
import Message from 'primevue/message'
import Tag from 'primevue/tag'
import Panel from 'primevue/panel'
import Timeline from 'primevue/timeline';
import type { ItemEvent } from '@/bindings/ItemEvent'
import EventTimelineItem from '@/components/EventTimelineItem.vue'

const route = useRoute()

const loading = ref(false)
const item = ref(null as ItemDetails | null)
const error = ref(null as string | null)

const itemsStore = useItemsStore()
const tagsStore = useTagsStore()

// watch the params of the route to fetch the data again
watch(() => route.params.id, fetchItemDetails as any, { immediate: true })

async function fetchItemDetails(itemId: bigint) {
  error.value = item.value = null
  loading.value = true

  try {
    item.value = await itemsStore.details(itemId)
  } catch (err: any) {
    error.value = err.toString()
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <ProgressSpinner v-if="loading" />
  <Message v-if="error" severity="error">{{ error }}</Message>
  <div v-if="item" class="flex justify-content-center">
    <dl class="grid col-10 flex">
      <h2 class="col-12">Item details</h2>
      <dt class="col-2">Id :</dt>
      <dd class="col-8">#{{ item.id }}</dd>
      <dt class="col-2">Name :</dt>
      <dd class="col-8">{{ item.name }}</dd>
      <dt class="col-2">Serial number :</dt>
      <dd class="col-8">{{ item.serial_number }}</dd>
      <dt class="col-2">Days between inspection :</dt>
      <dd class="col-8">{{ item.inspection_period_days }}</dd>
      <dt class="col-2">Tags :</dt>
      <dd class="col-8">
        <Tag v-for="tagId in item.tags" :key="tagId" :value="tagsStore.getTag(tagId).name" />
      </dd>
      <Panel class="col-10" header="Events:">
        <Timeline :value="item.events">
          <template #content="slotProps">
            <EventTimelineItem :event="slotProps.item" />
          </template>
        </Timeline>
      </Panel>
    </dl>
  </div>
</template>
