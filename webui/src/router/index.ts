import { createRouter, createWebHistory } from 'vue-router'
import ItemList from '../views/ItemList.vue'
import ItemCreate from '../views/ItemCreate.vue'
import ItemDetails from '../views/ItemDetails.vue'
import TagList from '../views/TagList.vue'

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
    },
    {
      path: '/items/details/:id',
      component: ItemDetails
    },
    {
      path: '/tags',
      component: TagList
    }
  ]
})

export default router