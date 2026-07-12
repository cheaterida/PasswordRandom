<script setup lang="ts">
import { ref, onMounted } from "vue"
import { invoke } from "@tauri-apps/api/core"
import { useTemplateStore } from "../stores/template"

const emit = defineEmits<{ created: [] }>()

const templateStore = useTemplateStore()
const expanded = ref(false)

const newName = ref("")
const newPattern = ref("")
const error = ref("")
const saving = ref(false)

const examples = [
  "[word]-[digit:4]",
  "[upper:4]-[lower:3]-[digit:4]",
  "[alpha:6][symbol:2][digit:2]",
  "my_[word]_[digit:3]",
  "prefix_[upper:2][digit:4]_suffix",
]

onMounted(() => {
  templateStore.loadTemplates()
})

async function saveTemplate() {
  error.value = ""
  if (!newName.value.trim()) {
    error.value = "请输入模板名称"
    return
  }
  if (!newPattern.value.trim()) {
    error.value = "请输入模板表达式"
    return
  }

  saving.value = true
  try {
    await invoke("validate_template", { pattern: newPattern.value })
    await templateStore.saveTemplate(newName.value.trim(), newPattern.value.trim())
    newName.value = ""
    newPattern.value = ""
    emit("created")
  } catch (e: unknown) {
    error.value = String(e)
  } finally {
    saving.value = false
  }
}

async function removeTemplate(id: number) {
  await templateStore.deleteTemplate(id)
  emit("created")
}
</script>

<template>
  <div class="border-t border-zinc-700 pt-3">
    <button
      class="flex items-center gap-1 text-sm text-zinc-400 hover:text-zinc-200 cursor-pointer"
      @click="expanded = !expanded"
    >
      <span>{{ expanded ? "▾" : "▸" }}</span>
      管理模板 ({{ templateStore.templates.length }})
    </button>

    <div v-if="expanded" class="mt-3 space-y-3">
      <!-- Existing Templates -->
      <div v-if="templateStore.templates.length > 0" class="space-y-1">
        <div
          v-for="tpl in templateStore.templates"
          :key="tpl.id"
          class="flex items-center justify-between px-3 py-2 bg-zinc-800 rounded-lg"
        >
          <div>
            <div class="text-zinc-200 text-sm">{{ tpl.name }}</div>
            <div class="text-zinc-500 text-xs font-mono">{{ tpl.pattern }}</div>
          </div>
          <button
            class="text-red-400 hover:text-red-300 text-xs cursor-pointer"
            @click="removeTemplate(tpl.id)"
          >
            删除
          </button>
        </div>
      </div>

      <!-- Create New -->
      <div class="space-y-2">
        <input
          v-model="newName"
          placeholder="模板名称"
          class="w-full px-3 py-2 bg-zinc-800 border border-zinc-700 rounded-lg text-zinc-200 placeholder-zinc-500 focus:outline-none focus:border-emerald-500 text-sm"
        />
        <input
          v-model="newPattern"
          placeholder="模板表达式，如: [word]-[digit:4]"
          class="w-full px-3 py-2 bg-zinc-800 border border-zinc-700 rounded-lg text-zinc-200 placeholder-zinc-500 focus:outline-none focus:border-emerald-500 text-sm font-mono"
          @keyup.enter="saveTemplate"
        />
        <div class="flex flex-wrap gap-1">
          <span class="text-zinc-500 text-xs mr-1">令牌:</span>
          <span class="text-zinc-500 text-xs bg-zinc-800 px-1 rounded">[word]</span>
          <span class="text-zinc-500 text-xs bg-zinc-800 px-1 rounded">[upper:N]</span>
          <span class="text-zinc-500 text-xs bg-zinc-800 px-1 rounded">[lower:N]</span>
          <span class="text-zinc-500 text-xs bg-zinc-800 px-1 rounded">[digit:N]</span>
          <span class="text-zinc-500 text-xs bg-zinc-800 px-1 rounded">[symbol:N]</span>
          <span class="text-zinc-500 text-xs bg-zinc-800 px-1 rounded">[alpha:N]</span>
          <span class="text-zinc-500 text-xs bg-zinc-800 px-1 rounded">[any:N]</span>
        </div>
        <div class="flex flex-wrap gap-1">
          <span
            v-for="ex in examples"
            :key="ex"
            class="text-xs text-zinc-500 bg-zinc-800 px-1.5 py-0.5 rounded cursor-pointer hover:bg-zinc-700"
            @click="newPattern = ex"
          >
            {{ ex }}
          </span>
        </div>
        <div v-if="error" class="text-red-400 text-xs">{{ error }}</div>
        <button
          :disabled="saving"
          class="w-full py-1.5 bg-zinc-700 hover:bg-zinc-600 disabled:bg-zinc-800 text-zinc-300 text-sm rounded-lg transition-colors cursor-pointer"
          @click="saveTemplate"
        >
          {{ saving ? "保存中..." : "保存模板" }}
        </button>
      </div>
    </div>
  </div>
</template>
