<script setup lang="ts">
import { ref, onMounted } from "vue"
import { invoke } from "@tauri-apps/api/core"
import { useTemplateStore } from "../stores/template"

const emit = defineEmits<{ created: [] }>()

const templateStore = useTemplateStore()
const expanded = ref(false)

const editId = ref<number | null>(null)
const editName = ref("")
const editPattern = ref("")
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

function startEdit(tpl: { id: number; name: string; pattern: string }) {
  editId.value = tpl.id
  editName.value = tpl.name
  editPattern.value = tpl.pattern
  error.value = ""
}

function cancelEdit() {
  editId.value = null
  editName.value = ""
  editPattern.value = ""
  error.value = ""
}

async function doSave() {
  error.value = ""
  if (!editName.value.trim()) {
    error.value = "请输入模板名称"
    return
  }
  if (!editPattern.value.trim()) {
    error.value = "请输入模板表达式"
    return
  }

  saving.value = true
  try {
    await invoke("validate_template", { pattern: editPattern.value })
    if (editId.value) {
      await templateStore.updateTemplate(editId.value, editName.value.trim(), editPattern.value.trim())
    } else {
      await templateStore.saveTemplate(editName.value.trim(), editPattern.value.trim())
    }
    cancelEdit()
    emit("created")
  } catch (e: unknown) {
    error.value = String(e)
  } finally {
    saving.value = false
  }
}

async function removeTemplate(id: number) {
  await templateStore.deleteTemplate(id)
  if (editId.value === id) cancelEdit()
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
          <div class="flex items-center gap-1.5">
            <button
              class="text-emerald-400 hover:text-emerald-300 text-xs cursor-pointer"
              @click="startEdit(tpl)"
            >
              编辑
            </button>
            <button
              class="text-red-400 hover:text-red-300 text-xs cursor-pointer"
              @click="removeTemplate(tpl.id)"
            >
              删除
            </button>
          </div>
        </div>
      </div>

      <!-- Edit / Create Form -->
      <div class="space-y-2">
        <div class="text-zinc-400 text-xs">
          {{ editId ? `编辑模板` : "新建模板" }}
        </div>
        <input
          v-model="editName"
          placeholder="模板名称"
          class="w-full px-3 py-2 bg-zinc-800 border border-zinc-700 rounded-lg text-zinc-200 placeholder-zinc-500 focus:outline-none focus:border-emerald-500 text-sm"
        />
        <input
          v-model="editPattern"
          placeholder="模板表达式，如: [word]-[digit:4]"
          class="w-full px-3 py-2 bg-zinc-800 border border-zinc-700 rounded-lg text-zinc-200 placeholder-zinc-500 focus:outline-none focus:border-emerald-500 text-sm font-mono"
          @keyup.enter="doSave"
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
            @click="editPattern = ex"
          >
            {{ ex }}
          </span>
        </div>
        <div v-if="error" class="text-red-400 text-xs">{{ error }}</div>
        <div class="flex gap-2">
          <button
            v-if="editId"
            class="flex-1 py-1.5 bg-zinc-700 hover:bg-zinc-600 text-zinc-300 text-sm rounded-lg transition-colors cursor-pointer"
            @click="cancelEdit"
          >
            取消
          </button>
          <button
            :disabled="saving"
            class="flex-1 py-1.5 bg-emerald-600 hover:bg-emerald-500 disabled:bg-emerald-800 text-white text-sm rounded-lg transition-colors cursor-pointer"
            @click="doSave"
          >
            {{ saving ? "保存中..." : editId ? "更新" : "保存" }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
