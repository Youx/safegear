<script setup lang="ts">
import { useUsersStore } from '@/stores'
import DataTable, { type DataTableFilterMetaData } from 'primevue/datatable'
import { type Tag } from '@/bindings/Tag'
import Column from 'primevue/column'
import Button from 'primevue/button'
import IconField from 'primevue/iconfield'
import InputIcon from 'primevue/inputicon'
import InputText from 'primevue/inputtext'
import router from '@/router'
import type { UserWithPermissions } from '@/bindings/UserWithPermissions'

const store = useUsersStore()
const selectedTags = defineModel('selectedUsers', { default: (): Tag[] => [] })
store.refresh()

</script>

<template>
    <div class="w-screen">
        <DataTable :value="store.users" v-model:filters="store.filters" :globalFilterFields="['login']"
            v-model:selection="selectedTags" :paginator="true" :rows="20">
            <template #header>
                <div class="flex overflow-hidden gap-3">
                    <h2 class="flex-none flex align-items-left justify-content-left">Users</h2>
                    <div class="flex-grow-1 gap-1 align-items-center flex justify-content-left">
                        <Button icon="pi pi-plus" rounded raised @click="router.push('/users/new')"></Button>
                        <Button icon="pi pi-trash" rounded outlined :disabled="selectedTags.length == 0"
                            severity="danger" size="small" aria-label="Delete"
                            @click="store.delete(selectedTags.map((t) => t.id))" />
                    </div>
                    <div class="flex-none flex align-items-center justify-content-center">
                        <IconField>
                            <InputIcon>
                                <i class="pi pi-search" />
                            </InputIcon>
                            <InputText v-model="(store.filters['global'] as DataTableFilterMetaData).value"
                                placeholder="Keyword Search" />
                        </IconField>
                    </div>
                    <div class="flex-none flex align-items-center justify-content-center">
                        <Button icon="pi pi-refresh" rounded raised @click="store.refresh()" />
                    </div>
                </div>
            </template>
            <Column selectionMode="multiple"></Column>
            <Column field="id" header="#" />
            <Column field="login" header="Login" />
            <Column header="Permissions">
                <template #body="slotProps">
                    <div class="flex gap-2">
                        <span class="pi pi-database" v-tooltip="'Can manage items'"
                            v-if="(slotProps.data as UserWithPermissions).perm_items" />
                        <span class="pi pi-tags" v-tooltip="'Can manage tags'"
                            v-if="(slotProps.data as UserWithPermissions).perm_tags" />
                        <span class="pi pi-users" v-tooltip="'Can manage users'"
                            v-if="(slotProps.data as UserWithPermissions).perm_users" />
                        <span class="pi pi-list-check" v-tooltip="'Can inspect items'"
                            v-if="(slotProps.data as UserWithPermissions).perm_action_inspect" />
                        <span class="pi pi-chevron-circle-right" v-tooltip="'Can lend items'"
                            v-if="(slotProps.data as UserWithPermissions).perm_action_lend" />
                    </div>
                </template>
            </Column>
            <Column header="Actions">
                <template #body="slotProps">
                    <div class="flex gap-1">
                        <Button icon="pi pi-key" severity="secondary" rounded outlined size="small"
                            aria-label="Change password" v-tooltip="'Change password'" />
                        <Button icon="pi pi-trash" rounded outlined severity="danger" size="small" aria-label="Delete"
                            v-tooltip="'Delete'" @click="store.delete(slotProps.data.id)" />
                    </div>
                </template>
            </Column>
        </DataTable>
    </div>
</template>
