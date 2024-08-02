import { defineStore } from 'pinia'
import type { Item } from './bindings/Item'
import type { CreateItem } from './bindings/CreateItem'
import type { Tag } from './bindings/Tag'
import type { ItemDetails } from './bindings/ItemDetails'
import type { DataTableFilterMeta } from 'primevue/datatable'

const URL = '/api'

class Requester {
  static async request<T>(url: string, method: 'POST' | 'GET' | 'DELETE' | 'PUT', data: T | undefined): Promise<Response> {
    const appSettings = useAppSettingsStore();
    const serializedData = data !== undefined ? JSON.stringify(data) : undefined;
    const headers: {[id: string]: string} = {};
    headers['Content-Type'] = 'application/json';
    if (appSettings.jwtToken) {
      headers['Authorization'] = 'Bearer ' + appSettings.jwtToken;
    }
    return await fetch(URL + url, {
      method,
      headers,
      body: serializedData,
    })
  }
  static async post<T, Y>(url: string, data: T): Promise<Y> {
    const res: Y = JSON.parse(await (await Requester.request(url, 'POST', data)).text());
    return res
  }
  static async get<Y>(url: string): Promise<Y> {
    const res: Y = JSON.parse(await (await Requester.request(url, 'GET', undefined)).text())
    return res
  }
  static async delete(url: string) {
    await Requester.request(url, 'DELETE', undefined)
  }

}

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
      return await Requester.get('/items' + item_id)
    },

    async refresh() {
      this.items = await Requester.get('/items')
    },

    async create(create_item: CreateItem): Promise<Item> {
      const item: Item = await Requester.post('/items', create_item);
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
      const tags: Tag[] = await Requester.get('/tags');
      this.tags = tags
      this.byId = tags.reduce(
        function (map, obj: Tag) {
          map[obj.id as unknown as string] = obj
          return map
        },
        {} as { [key: string]: Tag }
      )
    },
    async delete(tag_id: bigint[] | bigint) {
      if (Array.isArray(tag_id)) {
        await Promise.allSettled(tag_id.map((value) => Requester.delete('/tags/' + value )))
      } else {
        await Requester.delete('/tags/' + tag_id)
      }
      await this.refresh()
      const itemStore = useItemsStore()
      await itemStore.refresh()
    }
  }
})

import { FilterMatchMode } from '@primevue/core/api'
import { $dt } from '@primevue/themes'
import type { UserToken } from './bindings/UserToken'
import type { LoginUser } from './bindings/LoginUser'

export const useItemSearchStore = defineStore('itemSearch', {
  state: () => ({
    data: {
      global: { value: null, matchMode: FilterMatchMode.CONTAINS },
      name: { value: null, matchMode: FilterMatchMode.STARTS_WITH }
    } as DataTableFilterMeta
  })
})
export const useTagSearchStore = defineStore('tagSearch', {
  state: () => ({
    data: {
      global: { value: null, matchMode: FilterMatchMode.CONTAINS }
    } as DataTableFilterMeta
  })
})
import { jwtDecode } from 'jwt-decode'
import type { UserWithPermissions } from './bindings/UserWithPermissions'
export const useAppSettingsStore = defineStore('appSettings', {
  state: () => ({
    badLogin: false,
    jwtToken: null as string | null,
    darkMode: document.querySelector('html')?.classList.contains('my-app-dark')
  }),
  getters: {
    textColor(): string {
      return $dt('text.color').value[this.darkMode ? 'dark' : 'light'].value
    },
    canManageTags(): boolean {
      if (!this.jwtToken) {
        return false
      }
      const payload = jwtDecode(this.jwtToken)
      return (payload as any).perm_tags;
    },
    canManageItems(): boolean {
      if (!this.jwtToken) {
        return false
      }
      const payload = jwtDecode(this.jwtToken)
      return (payload as any).perm_items;
    },
    canManageUsers(): boolean {
      if (!this.jwtToken) {
        return false
      }
      const payload = jwtDecode(this.jwtToken)
      return (payload as any).perm_users;
    }
  },
  actions: {
    logout() {
      this.jwtToken = null
    },
    async login(login: string, password: string) {
      this.badLogin = false;
      const query: LoginUser = {
        login,
        password,
      };
      if (this.jwtToken) {
        throw new Error("JWT Token is already set, this should not happen")
      }
      const response = await fetch(URL + '/users/login', {
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(query),
        method: 'POST'
      });
      if (response.status !== 200) {
        this.badLogin = true;
      } else {
        const body: UserToken = await response.json()
        this.jwtToken = body.jwt_token;
      }
    },
    toggle() {
      this.darkMode = !this.darkMode
      document.querySelector('html')?.classList.toggle('my-app-dark')
    }
  }
})

export const useAppInfoStore = defineStore('appInfo', {
  state: () => ({
    version: "",
  }),
  actions: {
    async refresh() {
      const version = await fetch("/version.txt");
      this.version = await version.text();
    }
  }
})

export const useUsersStore = defineStore('users', {
  state: () => ({
    users: [] as UserWithPermissions[],
    filters: {
      global: { value: null, matchMode: FilterMatchMode.CONTAINS }
    } as DataTableFilterMeta
  }),
  actions: {
    async refresh() {
      this.users = await Requester.get('/users')
    },
    async delete(tag_id: bigint[] | bigint) {
      console.log("TODO: delete users")
    }
  }
})