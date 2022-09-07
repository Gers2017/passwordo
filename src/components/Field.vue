<script setup lang="ts">
const { forId, labelText } = defineProps<{ forId: string, labelText: string }>();
</script>

<template>
    <fieldset>
        <label :for="forId">{{labelText}}</label>
        <slot></slot>
    </fieldset>
</template>