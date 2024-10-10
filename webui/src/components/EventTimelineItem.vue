<script setup lang="ts">
import type { EventData } from '@/bindings/EventData';
import type { ItemEvent } from '@/bindings/ItemEvent';
import { DateTime } from 'luxon';
defineProps<{
    event: ItemEvent
}>()

function eventDescription(ts: string, event_data: EventData): string {
    const event_date = DateTime.fromISO(ts);

    const printDay = event_date.toFormat("dd/MM/yyyy")
    const printMonth = event_date.toFormat("MM/yyyy")

    switch (event_data.kind) {
        case "Manufactured":
            return `Manufactured on ${printMonth}`
        case "PutIntoService":
            return `Put into service on ${printDay}`
        case "Inspected":
            switch (event_data.result) {
                case 'Good':
                    return `Inspected by ${event_data.inspector} on ${printDay}. Item is in good condition.`
                case 'NormalWear':
                    return `Inspected by ${event_data.inspector} on ${printDay}. Item is in normal wear condition and can still be used.`
                case 'Warning':
                    return `Inspected by ${event_data.inspector} on ${printDay}}. Item is heavily worn off, and will have to be replaced soon.`
                case 'Danger':
                    return `Inspected by ${event_data.inspector} on ${printDay}. Item is dangerous and should be retired immediately.`
            }
            break;
        case "Borrowed":
            return `Borrowed by ${event_data.borrower} (validation: ${event_data.validator}) on ${printDay}`
        case "Returned":
            return `Borrow returned (validation: ${event_data.validator}) on ${printDay}`
        case "Lost":
            return `Declared lost on ${printDay}`
        case "Retired":
            return `Retired on ${printDay}`
    }
}
</script>
<template>
    <div>
        {{ eventDescription(event.ts, event.data) }}
    </div>
</template>