<script setup lang="ts">
import Panel from 'primevue/panel';
import Button from 'primevue/button';
import InputText from 'primevue/inputtext';
import Message from 'primevue/message';
import FloatLabel from 'primevue/floatlabel';

import { useAppSettingsStore } from '@/stores';
import { ref } from 'vue';

const appSettings = useAppSettingsStore();
const login = ref();
const password = ref();

async function submit() {
    await appSettings.login(login.value, password.value)
}
</script>
<template>
    <div class="flex p5 align-items-center">
        <Panel class="flex flex-column">
            <Message class="mb-5" severity="error" v-if="appSettings.badLogin">Invalid login</Message>

            <FloatLabel class="col-12 mb-4">
                <InputText id="login" v-model="login" type="text" />
                <label for="login">Login</label>
            </FloatLabel>

            <FloatLabel class="col-12 mb-4">
                <InputText id="password" v-model="password" type="password" />
                <label for="password">Password</label>
            </FloatLabel>

            <Button class="col-12 p-2" severity="primary" @click="submit()" label="Sign in" />
        </Panel>
    </div>
</template>