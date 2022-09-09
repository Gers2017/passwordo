<script setup lang="ts">
import EntropyMeter from "./EntropyMeter.vue";
import { sendNotification } from "@tauri-apps/api/notification";
import { writeClipboard, PasswordInfo } from "../tauri"

const { passwordInfo } = defineProps<{ passwordInfo: PasswordInfo }>();
const { text, entropy } = passwordInfo;
async function handleCopy() {
    await writeClipboard(text);
    sendNotification("Copied to clipboard");
}

</script>

<template>
    <li>
        <div class="row">
            <span class="entropy">Entropy: {{entropy}} bits</span>
            <EntropyMeter :entropy="entropy" />
        </div>
        <div class="content">
            <span>{{text}}</span>
            <button class="copy" @click="handleCopy" title="Click to copy the password">
                <img src="../assets/clipboard-copy.svg" alt="copy to clipboard">
            </button>
        </div>
    </li>
</template>

<style scoped>
li {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
}

button.copy {
    color: white;
    padding: 0.4rem;
}

span {
    font-weight: 600;
}

.content {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border: 1px solid white;
    padding: 0.4rem;
    border-radius: 8px;
    background-color: gainsboro;
}

.entropy {
    font-size: 0.8rem;
}
</style>