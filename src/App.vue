<script setup lang="ts">
import { reactive, ref, computed } from "vue";
import CopyPassword from "./components/CopyPassword.vue";
import Field from "./components/Field.vue";
import { writeClipboard, GenerationMethod, gen_schemed_passwords, gen_nonsense_passwords, gen_rand_bytes_passwords, PasswordInfo } from "./tauri"
import { sendNotification } from "@tauri-apps/api/notification";

const methods: GenerationMethod[] = ["schema", "nonsense", "random-bytes"]
const listOfWords = reactive<string[]>([]);
const passwords = reactive<PasswordInfo[]>([]);

const method = ref(methods[0]);
const useSpecial = ref(true);
const passwordsToGenerate = ref(1);

// schema method
const hashLength = ref(8);
const invert = ref(false);
const wordsToUse = ref("");

// nonsense method
const passwordLength = ref(12);

const isSchemaMethod = computed(() => method.value === "schema")
const isNonsenseMethod = computed(() => method.value === "nonsense")
const isRandBytesMethod = computed(() => method.value === "random-bytes")

function clamp(amount: number, min = 1) {
  return Math.max(amount, min);
}

async function onSubmit(_e: Event) {
  let generated: PasswordInfo[] = [];
  const amount = clamp(passwordsToGenerate.value);

  try {
    switch (method.value) {
      case "schema":
        // empty listOfWord
        listOfWords.splice(0, listOfWords.length)
        // fill with new words
        listOfWords.push(...extractWords(wordsToUse.value));

        generated = await gen_schemed_passwords(
          amount,
          clamp(hashLength.value),
          invert.value,
          listOfWords,
          useSpecial.value
        );
        break;
      case "nonsense":

        generated = await gen_nonsense_passwords(amount, clamp(passwordLength.value), useSpecial.value);

        break;
      case "random-bytes":

        generated = await gen_rand_bytes_passwords(amount, clamp(passwordLength.value));

        break;
      default:
        break;
    }

    passwords.push(...generated);
    sendNotification("Generating passwords");

  } catch (e) {
    console.error(e);
  }
}

async function copyAll() {
  await writeClipboard(passwords.join("\n"));
  sendNotification("All passwords were copied to the clipboard");
}

function clearOutput() {
  passwords.splice(0, passwords.length);
}

function extractWords(text: string): string[] {
  const words: string[] =
    text.trim().split("\n").map(it => it.split(",").map(it => it.trim())).flat().filter(it => it.length > 0)
  return words;
}

</script>

<template>
  <div class="content app-view">
    <form class="password-form list" @submit.prevent="onSubmit">

      <Field for-id="gen-method" label-text="Generation Method">
        <select id="gen-method" v-model="method">
          <option v-for="item in methods" :value="item">
            {{item}}
          </option>
        </select>
      </Field>

      <Field for-id="amount" label-text="Passwords to generate">
        <input name="amount" type="number" placeholder="6" v-model.number="passwordsToGenerate" min="1">
      </Field>

      <Field for-id="special" label-text="Include special characters" v-if="isSchemaMethod || isNonsenseMethod">
        <input type="checkbox" name="special" id="special" v-model="useSpecial">
      </Field>

      <!-- Schema method -->

      <div v-if="isSchemaMethod">
        <Field for-id="hash-length" label-text="Hash length">
          <input name="hash-length" type="number" placeholder="12" v-model.number="hashLength" min="1" max="100">
        </Field>

        <Field for-id="inverted" label-text="Invert schema">
          <input type="checkbox" name="inverted" id="inverted" v-model="invert">
        </Field>

        <Field for-id="list-of-words" label-text="Words to use">
          <textarea minlength="" name="list-of-words" id="list-of-words" cols="36" rows="8"
            placeholder="A list of words like: potato, spoon or each word in it's own line. The longer the word, the better the result"
            v-model="wordsToUse"></textarea>
        </Field>
      </div>

      <!-- Nonsense method -->

      <div v-if="isNonsenseMethod || isRandBytesMethod">
        <Field for-id="password-length" label-text="Password length">
          <input name="password-length" type="number" placeholder="12" v-model.number="passwordLength" min="1" max="60">
        </Field>
      </div>

      <button class="action">Generate</button>

    </form>
    <div class="list">
      <div class="row">
        <button @click="copyAll" title="Click to copy all passwords">Copy all</button>
        <button @click="clearOutput" title="Click to clear all passwords">Clear</button>
      </div>
      <ul class="list">
        <CopyPassword :password-info="p" v-for="p in passwords" />
      </ul>
    </div>
  </div>
</template>

<style scoped>
ul {
  list-style: none;
  padding: 0;
  margin: 0;
  margin-block: 0;
}
</style>
