import 'primeicons/primeicons.css'
import 'primeflex/primeflex.css'
import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { useItemsStore, useTagsStore } from './stores'
import PrimeVue from 'primevue/config'
import Aura from '@primevue/themes/aura'

import App from './App.vue'
import router from './router'

const app = createApp(App)
  .use(createPinia())
  .use(router)
  .use(PrimeVue, {
    theme: {
      preset: Aura,
      options: {
        darkModeSelector: 'system'
      }
    }
  })

const itemStore = useItemsStore()
const tagsStore = useTagsStore()
async function initStores() {
  await itemStore.refresh()
  await tagsStore.refresh()
}
initStores()

app.mount('#app')
