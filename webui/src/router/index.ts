import { createRouter, createWebHistory } from 'vue-router'
import ItemList from '../views/ItemList.vue'
import ItemCreate from '../views/ItemCreate.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/items',
      component: ItemList
    },
    {
      path: '/items/new',
      component: ItemCreate
    }
  ]
})

export default router
