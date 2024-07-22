import { defineStore } from 'pinia'
import type { Item } from './bindings/Item'
import type { CreateItem } from './bindings/CreateItem'
import type { Tag } from './bindings/Tag'
import type { ItemDetails } from './bindings/ItemDetails'
import type { DataTableFilterMeta } from 'primevue/datatable'

const URL = '/api'

export const useItemsStore = defineStore('items', {
  state: () => ({
    items: [] as Item[]
  }),
  getters: {
    // Unique names of all items
    names(): string[] {
      return [...new Set(this.items.map((item) => item.name))].sort()
    }
  },
  actions: {
    async details(item_id: bigint): Promise<ItemDetails> {
      const response = await fetch(URL + '/items/' + item_id, { method: 'GET' })
      if (response.status !== 200) {
        throw new Error('Failed to fetch item details: ' + response.statusText)
      }
      const item: ItemDetails = JSON.parse(await response.text())
      return item
    },

    async refresh() {
      const response = await fetch(URL + '/items', { method: 'GET' })
      const items: [Item] = JSON.parse(await response.text())
      this.items = items
      return items
    },

    async create(create_item: CreateItem) {
      const response = await fetch(URL + '/items', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(create_item)
      })
      const item: Item = JSON.parse(await response.text())
      this.refresh()
      return item
    }
  }
})

export const useTagsStore = defineStore('tags', {
  state: () => ({ tags: [] as Tag[], byId: {} as { [key: string]: Tag } }),
  getters: {},
  actions: {
    getTag(tagId: bigint): Tag {
      return (
        this.byId[tagId as unknown as string] ?? { id: -1, color: '#ffffff', name: 'undefined' }
      )
    },
    async refresh() {
      const response = await fetch(URL + '/tags', { method: 'GET' })
      const tags: [Tag] = JSON.parse(await response.text())
      this.tags = tags
      this.byId = tags.reduce(
        function (map, obj: Tag) {
          map[obj.id as unknown as string] = obj
          return map
        },
        {} as { [key: string]: Tag }
      )
      return tags
    }
  }
})

import { FilterMatchMode } from '@primevue/core/api'
import { $dt } from '@primevue/themes'
export const useItemSearchStore = defineStore('itemSearch', {
  state: () => ({
    data: {
      global: { value: null, matchMode: FilterMatchMode.CONTAINS },
      name: { value: null, matchMode: FilterMatchMode.STARTS_WITH }
    } as DataTableFilterMeta
  })
})

export const useAppSettingsStore = defineStore('appSettings', {
  state: () => ({
    darkMode: document.querySelector('html')?.classList.contains('my-app-dark')
  }),
  getters: {
    textColor(): string {
      return $dt('text.color').value[this.darkMode ? 'dark' : 'light'].value
    }
  },
  actions: {
    toggle() {
      this.darkMode = !this.darkMode
      document.querySelector('html')?.classList.toggle('my-app-dark')
    }
  }
})
