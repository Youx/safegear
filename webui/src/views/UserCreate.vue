<script setup lang="ts">
import { useUsersStore } from '@/stores'
import { useRouter } from 'vue-router'
import InputText from 'primevue/inputtext'
import Button from 'primevue/button'
import Checkbox from 'primevue/checkbox'

let store = useUsersStore()
let router = useRouter()

let formLogin = defineModel('login', { default: '' })

let formPassword = defineModel('password', { default: '' })
let formPassword2 = defineModel('password2', { default: '' })
let formPermItems = defineModel('perm_items', { default: false })
let formPermTags = defineModel('perm_tags', { default: false })
let formPermUsers = defineModel('perm_users', { default: false })
let formPermActionInspect = defineModel('perm_action_inspect', { default: false })
let formPermActionLend = defineModel('perm_action_lend', { default: false })

async function submit() {
    store.create({
        login: formLogin.value,
        password: formPassword.value,
        is_active: true,
        perm_items: formPermItems.value,
        perm_tags: formPermTags.value,
        perm_users: formPermUsers.value,
        perm_action_inspect: formPermActionInspect.value,
        perm_action_lend: formPermActionLend.value,
    })
    router.push('/users')
}
function validate(): boolean {
    if (formLogin.value === '') {
        return false;
    }
    if (formPassword.value === '') {
        return false
    }
    if (formPassword.value !== formPassword2.value) {
        return false
    }
    return true
}
</script>
<template>
    <div class="grid justify-content-center">
        <div class="col-9">
            <h2>New user</h2>
        </div>
        <div class="field grid justify-content-end col-9">
            <label class="col-4 mb-0" for="name">Login</label>
            <InputText class="col-6 h-3rem" :dropdown="true" v-model="formLogin" />
            <div class=" col-2">
            </div>
        </div>

        <div class="field grid justify-content-end col-9">
            <div class="col-4">
                <label for="perm-items"><span class="pi pi-database mr-3"></span>
                    <span>Manage items</span>
                </label>
            </div>
            <div class="col-8">
                <Checkbox id="perm-items" v-model="formPermItems" :binary="true" />
            </div>
        </div>
        <div class="field grid justify-content-end col-9">
            <div class="col-4">
                <label for="perm-tags"><span class="pi pi-tags mr-3"></span>
                    <span>Manage tags</span>
                </label>
            </div>
            <div class="col-8">
                <Checkbox id="perm-tags" v-model="formPermTags" :binary="true" />
            </div>
        </div>
        <div class="field grid justify-content-end col-9">
            <div class="col-4">
                <label for="perm-users"><span class="pi pi-users mr-3"></span>
                    <span>Manage users</span>
                </label>
            </div>
            <div class="col-8">
                <Checkbox id="perm-users" v-model="formPermUsers" :binary="true" />
            </div>
        </div>
        <div class="field grid justify-content-end col-9">
            <div class="col-4">
                <label for="perm-action-inspect"><span class="pi pi-list-check mr-3"></span>
                    <span>Inspect gear</span>
                </label>
            </div>
            <div class="col-8">
                <Checkbox id="perm-action-inspect" v-model="formPermActionInspect" :binary="true" />
            </div>
        </div>
        <div class="field grid justify-content-end col-9">
            <div class="col-4">
                <label for="perm-action-lend"><span class="pi pi-chevron-circle-right mr-3"></span>
                    <span>Lend gear</span>
                </label>
            </div>
            <div class="col-8">
                <Checkbox id="perm-action-lend" v-model="formPermActionLend" :binary="true" />
            </div>
        </div>

        <div class="field grid justify-content-end col-9">
            <label class="col-4 mb-0" for="formPassword">Password</label>
            <div class="col-8">
                <InputText type="password" class="h-3rem" v-model="formPassword" />
            </div>
        </div>
        <div class="field grid justify-content-end col-9">
            <label class="col-4 mb-0" for="formPassword">Password verification</label>
            <div class="col-8">
                <InputText type="password" class="h-3rem" v-model="formPassword2" />
            </div>
        </div>
        <div class="flex justify-content-start col-9">
            <div class="col-4 flex justify-content-start">
                <Button severity="secondary" @click="router.back" label="Cancel" />
            </div>
            <div class="col-4 flex justify-content-start">
                <Button :disabled="!validate()" @click="submit()" label="Submit" />
            </div>
        </div>
    </div>
</template>
