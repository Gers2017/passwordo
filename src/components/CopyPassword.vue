<script setup lang="ts">
import { sendNotification } from "@tauri-apps/api/notification";
import { writeClipboard } from "../tauri"

const { password } = defineProps<{ password: string }>();

async function handleCopy() {
    await writeClipboard(password)
    sendNotification("Copied to clipboard");
}

</script>

<template>
    <span>{{password}}</span>
    <button class="copy" @click="handleCopy">
        <img src="../assets/clipboard-copy.svg" alt="copy">
    </button>
</template>

<style scoped>
span {
    font-weight: 600;
}

.copy {
    color: white;
    padding: 0.4rem;
}
</style>