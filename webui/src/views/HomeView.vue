<script setup lang="ts">
import Changelog from '../../public/CHANGELOG.md'
import Panel from 'primevue/panel';
import ScrollPanel from 'primevue/scrollpanel';
</script>
<template>
    <div class="grid flex flex-column col-12">
        <div class="flex flex-none col-12">
            <h2>Home</h2>
        </div>
        <div class="flex flex-grow-0 col-6">
            <Panel header="What's New?" class="col-12">
                <ScrollPanel class="col-12" style="height: 50rem">
                    <Changelog />
                </ScrollPanel>
            </Panel>
        </div>
    </div>
</template>