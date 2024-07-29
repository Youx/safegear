import { createRouter, createWebHashHistory } from 'vue-router'
import ItemCreate from '../views/ItemCreate.vue'
import ItemDetails from '../views/ItemDetails.vue'
import ItemList from '../views/ItemList.vue'
import TagList from '../views/TagList.vue'
import HomeView from '../views/HomeView.vue'
import LoginView from '../views/LoginView.vue'


const router = createRouter({
  history: createWebHashHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      component: HomeView
    },
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
