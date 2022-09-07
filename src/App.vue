<script setup lang="ts">
import { reactive, ref, computed } from "vue";
import CopyPassword from "./components/CopyPassword.vue";
import Field from "./components/Field.vue";
import { writeClipboard, GenerationMethod, gen_schemed_passwords, gen_nonsense_passwords } from "./tauri"
import { sendNotification } from "@tauri-apps/api/notification";

const methods: GenerationMethod[] = ["schema", "nonsense"]
const passwords = reactive<string[]>([])

const method = ref(methods[0]);
const useSpecial = ref(true);
const passwordsToGenerate = ref(1);

// schema method
const hashLength = ref(6);
const invert = ref(false);
const listOfWords = reactive<string[]>([]);

// nonsense method
const passwordLength = ref(12);
const useRandomBytes = ref(false);

const isSchemaMethod = computed(() => {
  return method.value === "schema";
})

async function onSubmit(_e: Event) {
  let generated: string[] = [];
  const amount = Math.max(passwordsToGenerate.value, 1)

  console.log({
    method: method.value,
    len: passwordLength.value,
    amount,
    randBytes: useRandomBytes.value,
    invert: invert.value,
    hashLength: hashLength.value,
    useSpecial: useSpecial.value,
  });

  console.log(listOfWords);

  if (isSchemaMethod.value) {
    const hLength = Math.max(hashLength.value, 1);
    generated = await gen_schemed_passwords(
      hLength,
      amount,
      invert.value,
      listOfWords,
      useSpecial.value
    );
  } else {
    const length = Math.max(passwordLength.value, 1);
    generated = await gen_nonsense_passwords(length, amount, useRandomBytes.value, useSpecial.value);
  }

  passwords.push(...generated);
  sendNotification("Generating passwords");
}

async function copyAll() {
  await writeClipboard(passwords.join("\n"));
  sendNotification("All passwords copied to clipboard");
}

function clearOutput() {
  passwords.splice(0, passwords.length);
}

function handleWords(e: Event) {
  let target = e.target as HTMLTextAreaElement;
  const _words: string[] =
    target.value.trim().split("\n").map(it => it.split(",").map(it => it.trim())).flat().filter(it => it.length > 0)

  listOfWords.splice(0, listOfWords.length)
  listOfWords.push(..._words);
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

      <Field for-id="special" label-text="Include special characters">
        <input type="checkbox" name="special" id="special" v-model="useSpecial">
      </Field>

      <Field for-id="hash-length" label-text="Hash length" v-if="isSchemaMethod">
        <input name="hash-length" type="number" placeholder="12" v-model.number="hashLength" min="1" max="100">
      </Field>

      <Field for-id="password-length" label-text="Password length" v-if="!isSchemaMethod">
        <input name="password-length" type="number" placeholder="12" v-model.number="passwordLength" min="1" max="60">
      </Field>

      <Field for-id="inverted" label-text="Invert schema" v-if="isSchemaMethod">
        <input type="checkbox" name="inverted" id="inverted" v-model="invert">
      </Field>


      <Field for-id="list-of-words" label-text="Words to use" v-if="isSchemaMethod">
        <textarea name="list-of-words" id="list-of-words" cols="30" rows="8" placeholder="potato, spoon"
          @input="handleWords"></textarea>
      </Field>

      <Field for-id="random-bytes" label-text="Use random bytes" v-if="!isSchemaMethod">
        <input type="checkbox" name="random-bytes" id="random-bytes" v-model="useRandomBytes">
      </Field>

      <button class="action">Generate</button>
    </form>
    <div class="list">
      <div class="row">
        <button @click="copyAll">Copy all</button>
        <button @click="clearOutput">Clear</button>
      </div>
      <li v-for="p in passwords">
        <CopyPassword :password="p" />
      </li>
    </div>
  </div>
</template>

<style scoped>
li {
  display: flex;
  justify-content: space-between;
  align-items: center;
  border: 1px solid white;
  padding: 0.5rem;
  border-radius: 8px;
  background-color: gainsboro;
}
</style>
