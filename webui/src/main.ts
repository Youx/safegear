import 'primeicons/primeicons.css'
import 'primeflex/primeflex.css'
import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { useAppInfoStore } from './stores'
import PrimeVue from 'primevue/config'
import Aura from '@primevue/themes/aura'
import Tooltip from 'primevue/tooltip';

import App from './App.vue'
import router from './router'


const app = createApp(App)
  .use(createPinia())
  .use(router)
  .use(PrimeVue, {
    theme: {
      preset: Aura,
      options: {
        darkModeSelector: '.my-app-dark'
      }
    }
  })
  .directive('tooltip', Tooltip);


const appInfoStore = useAppInfoStore()
async function initStores() {
  await appInfoStore.refresh()
}
initStores()

app.mount('#app')
