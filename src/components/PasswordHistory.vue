<script setup lang="ts">
import { onMounted, ref } from "vue"
import { useHistoryStore } from "../stores/history"
import CategoryManager from "./CategoryManager.vue"

const historyStore = useHistoryStore()
const viewPassword = ref<Record<number, boolean>>({})

const showAddPanel = ref(false)
const addLabel = ref("")
const addUsername = ref("")
const addPassword = ref("")
const addCategoryId = ref<number | null>(null)
const addError = ref("")
const addSuccess = ref(false)

onMounted(() => {
  historyStore.loadPasswords()
  historyStore.loadCategories()
})

function toggleView(id: number) {
  viewPassword.value[id] = !viewPassword.value[id]
}

async function copyPassword(password: string) {
  const { writeText } = await import("@tauri-apps/plugin-clipboard-manager")
  await writeText(password)
}

async function doDelete(id: number, label: string) {
  if (window.confirm(`确定删除 "${label}"?`)) {
    await historyStore.deletePassword(id)
  }
}

async function handleAdd() {
  addError.value = ""
  if (!addLabel.value.trim()) {
    addError.value = "请输入用途标注"
    return
  }
  if (!addPassword.value) {
    addError.value = "请输入密码"
    return
  }
  try {
    await historyStore.savePassword(
      addLabel.value.trim(),
      addUsername.value.trim(),
      addPassword.value,
      "manual",
      addCategoryId.value,
      null,
    )
    addSuccess.value = true
    setTimeout(() => {
      showAddPanel.value = false
      addSuccess.value = false
      addLabel.value = ""
      addUsername.value = ""
      addPassword.value = ""
      addCategoryId.value = null
    }, 1200)
  } catch (e: unknown) {
    addError.value = String(e)
  }
}

function formatTime(ts: string) {
  if (!ts) return ""
  const d = new Date(ts)
  return d.toLocaleString("zh-CN", {
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  })
}
</script>

<template>
  <div class="flex flex-col h-full">
    <!-- Search & Filter -->
    <div class="flex flex-col gap-2 mb-3">
      <input
        v-model="historyStore.search"
        placeholder="搜索..."
        class="w-full px-3 py-2 bg-zinc-800 border border-zinc-700 rounded-lg text-zinc-200 placeholder-zinc-500 focus:outline-none focus:border-emerald-500 text-sm text-center"
        @input="historyStore.loadPasswords()"
      />
      <div class="flex gap-2 justify-center">
        <select
          v-model="historyStore.selectedCategory"
          class="px-3 py-2 bg-zinc-800 border border-zinc-700 rounded-lg text-zinc-200 text-sm focus:outline-none focus:border-emerald-500"
          @change="historyStore.loadPasswords()"
        >
          <option :value="null">全部分类</option>
          <option
            v-for="cat in historyStore.categories"
            :key="cat.id"
            :value="cat.id"
          >
            {{ cat.icon }} {{ cat.name }}
          </option>
        </select>
        <button
          class="px-3 py-2 bg-emerald-600 hover:bg-emerald-500 text-white text-sm font-medium rounded-lg shrink-0 cursor-pointer transition-colors"
          @click="showAddPanel = true; historyStore.loadCategories()"
        >
          + 添加
        </button>
      </div>
    </div>

    <!-- Passwords List -->
    <div v-if="historyStore.loading" class="text-center text-zinc-500 py-8">
      ...
    </div>
    <div
      v-else-if="historyStore.passwords.length === 0"
      class="text-center text-zinc-500 py-8 text-sm"
    >
      暂无历史记录
    </div>
    <div v-else class="flex-1 overflow-y-auto space-y-2">
      <div
        v-for="pw in historyStore.passwords"
        :key="pw.id"
        class="bg-zinc-800 rounded-lg p-3 border border-zinc-700/50 hover:border-zinc-600 transition-colors"
      >
        <div class="flex items-start justify-between">
          <div class="flex-1 min-w-0">
            <div class="flex items-center gap-2">
              <span class="text-zinc-200 font-medium text-sm truncate">
                {{ pw.label }}
              </span>
              <span
                v-if="pw.category_name"
                class="text-xs px-1.5 py-0.5 rounded bg-zinc-700 text-zinc-400"
              >
                {{ pw.category_name }}
              </span>
              <span
                v-if="pw.mode === 'manual'"
                class="text-xs px-1.5 py-0.5 rounded bg-amber-900/40 text-amber-400"
              >
                手动
              </span>
            </div>
            <div v-if="pw.username" class="text-zinc-500 text-xs mt-0.5">
              {{ pw.username }}
            </div>
            <div
              :class="[
                'font-mono text-sm mt-1.5 break-all',
                viewPassword[pw.id]
                  ? 'text-emerald-400'
                  : 'text-zinc-600',
              ]"
            >
              {{ viewPassword[pw.id] ? pw.password : "•".repeat(Math.min(pw.password.length, 24)) }}
            </div>
          </div>

          <div class="flex items-center gap-1 ml-2 shrink-0">
            <button
              class="text-xs px-1.5 py-1 rounded bg-zinc-700 hover:bg-zinc-600 text-zinc-400 hover:text-zinc-200 cursor-pointer transition-colors"
              @click="toggleView(pw.id)"
            >
              {{ viewPassword[pw.id] ? "隐藏" : "查看" }}
            </button>
            <button
              class="text-xs px-1.5 py-1 rounded bg-zinc-700 hover:bg-zinc-600 text-zinc-400 hover:text-zinc-200 cursor-pointer transition-colors"
              @click="copyPassword(pw.password)"
            >
              复制
            </button>
            <button
              class="text-xs px-1.5 py-1 rounded bg-zinc-700 hover:bg-red-900 text-zinc-400 hover:text-red-400 cursor-pointer transition-colors"
              @click="doDelete(pw.id, pw.label)"
            >
              删
            </button>
          </div>
        </div>
        <div class="text-zinc-600 text-xs mt-2">
          {{ formatTime(pw.created_at) }}
          <span class="ml-2 text-zinc-700">
            {{ pw.mode === "random" ? "随机" : pw.mode === "pin" ? "PIN" : pw.mode === "passphrase" ? "密语" : pw.mode === "template" ? "模板" : "手动" }}
          </span>
        </div>
      </div>
    </div>

    <!-- Category Manager -->
    <CategoryManager />

    <!-- Add Password Overlay -->
    <div
      v-if="showAddPanel"
      class="fixed inset-0 bg-black/60 flex items-center justify-center z-50"
      @click.self="showAddPanel = false"
    >
      <div class="bg-zinc-800 border border-zinc-700 rounded-xl p-6 w-80 space-y-4 mx-4">
        <h3 class="text-lg font-semibold text-zinc-100">添加密码记录</h3>
        <input
          v-model="addLabel"
          placeholder="用途标注 (如: Gmail)"
          class="w-full px-3 py-2 bg-zinc-900 border border-zinc-700 rounded-lg text-zinc-100 placeholder-zinc-500 focus:outline-none focus:border-emerald-500 text-sm"
        />
        <input
          v-model="addUsername"
          placeholder="用户名 (可选)"
          class="w-full px-3 py-2 bg-zinc-900 border border-zinc-700 rounded-lg text-zinc-100 placeholder-zinc-500 focus:outline-none focus:border-emerald-500 text-sm"
        />
        <div class="relative">
          <input
            v-model="addPassword"
            type="password"
            placeholder="密码"
            class="w-full px-3 py-2 bg-zinc-900 border border-zinc-700 rounded-lg text-zinc-100 placeholder-zinc-500 focus:outline-none focus:border-emerald-500 text-sm"
          />
        </div>
        <select
          v-model="addCategoryId"
          class="w-full px-3 py-2 bg-zinc-900 border border-zinc-700 rounded-lg text-zinc-100 text-sm focus:outline-none focus:border-emerald-500"
        >
          <option :value="null">无分类</option>
          <option
            v-for="cat in historyStore.categories"
            :key="cat.id"
            :value="cat.id"
          >
            {{ cat.icon }} {{ cat.name }}
          </option>
        </select>
        <div v-if="addError" class="text-red-400 text-xs">{{ addError }}</div>
        <div class="flex gap-2">
          <button
            class="flex-1 py-2 bg-zinc-700 hover:bg-zinc-600 text-zinc-300 rounded-lg transition-colors cursor-pointer text-sm"
            @click="showAddPanel = false"
          >
            取消
          </button>
          <button
            class="flex-1 py-2 bg-emerald-600 hover:bg-emerald-500 text-white rounded-lg transition-colors cursor-pointer text-sm"
            @click="handleAdd"
          >
            {{ addSuccess ? "已添加" : "保存" }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
