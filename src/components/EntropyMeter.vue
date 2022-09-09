<script setup lang="ts">
// https://keepass.info/help/kb/pw_quality_est.html
import { computed } from "vue";
interface Props {
    entropy: number
}

const { entropy } = defineProps<Props>();

const strength = computed(() => {
    if (entropy < 64) return "Very weak";
    else if (entropy < 80) return "Weak";
    else if (entropy < 112) return "Good";
    else if (entropy < 128) return "Strong";
    else if (entropy >= 128) return "Very strong";
});

const color = computed(() => {
    if (entropy < 64) return "red";
    else if (entropy < 80) return "orange";
    else if (entropy < 112) return "lime";
    else if (entropy < 128) return "green";
    else if (entropy >= 128) return "blue";
});

</script>

<template>
    <span :class="color">{{strength}}</span>
</template>

<style scoped>
span {
    color: white;
    font-weight: bold;
    font-size: 0.8rem;
    border-radius: 8px;
    padding: 2px 4px;
}

span.red {
    background-color: #e2002d;
}

span.orange {
    background-color: #ee9b00;
}

span.lime {
    background-color: #00b609;
}

span.green {
    background-color: #00b9a1;
}

span.blue {
    background-color: #1c85fd;
}
</style>