<script setup lang="ts">
import { onMounted, ref } from "vue"
import { invoke } from "@tauri-apps/api/core"
import { checkStatus, authenticate } from "@tauri-apps/plugin-biometric"
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

const reAuthVerified = ref(false)
const showReAuth = ref(false)
const reAuthPassword = ref("")
const reAuthError = ref("")
const reAuthLoading = ref(false)
const bioAvailable = ref(false)
let pendingAction: (() => void) | null = null

onMounted(async () => {
  historyStore.loadPasswords()
  historyStore.loadCategories()
  try {
    const status = await checkStatus()
    bioAvailable.value = status.isAvailable
  } catch {
    bioAvailable.value = false
  }
})

function requireReAuth(action: () => void) {
  if (reAuthVerified.value) {
    action()
    return
  }
  pendingAction = action
  reAuthPassword.value = ""
  reAuthError.value = ""
  showReAuth.value = true
}

async function handleReAuthPassword() {
  reAuthError.value = ""
  if (!reAuthPassword.value) {
    reAuthError.value = "请输入主密码"
    return
  }
  reAuthLoading.value = true
  try {
    const ok = await invoke<boolean>("unlock", { password: reAuthPassword.value })
    if (!ok) {
      reAuthError.value = "密码错误"
      return
    }
    onReAuthSuccess()
  } catch (e: unknown) {
    reAuthError.value = String(e)
  } finally {
    reAuthLoading.value = false
  }
}

async function handleReAuthBio() {
  reAuthError.value = ""
  reAuthLoading.value = true
  try {
    await authenticate("验证身份以查看密码")
    await invoke("biometric_unlock")
    onReAuthSuccess()
  } catch {
    reAuthError.value = "生物验证失败"
  } finally {
    reAuthLoading.value = false
  }
}

function onReAuthSuccess() {
  showReAuth.value = false
  reAuthVerified.value = true
  reAuthPassword.value = ""
  if (pendingAction) {
    const action = pendingAction
    pendingAction = null
    action()
  }
}

function cancelReAuth() {
  showReAuth.value = false
  reAuthPassword.value = ""
  reAuthError.value = ""
  pendingAction = null
}

function toggleView(id: number) {
  requireReAuth(() => {
    viewPassword.value[id] = !viewPassword.value[id]
  })
}

async function copyPassword(password: string) {
  requireReAuth(async () => {
    const { writeText } = await import("@tauri-apps/plugin-clipboard-manager")
    await writeText(password)
  })
}

function doDelete(id: number, label: string) {
  requireReAuth(() => {
    if (window.confirm(`确定删除 "${label}"?`)) {
      historyStore.deletePassword(id)
    }
  })
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

    <!-- Re-authentication Overlay -->
    <div
      v-if="showReAuth"
      class="fixed inset-0 bg-black/60 flex items-center justify-center z-50"
      @click.self="cancelReAuth"
    >
      <div class="bg-zinc-800 border border-zinc-700 rounded-xl p-6 w-80 space-y-4 mx-4">
        <h3 class="text-lg font-semibold text-zinc-100 text-center">验证身份</h3>
        <p class="text-zinc-400 text-sm text-center">查看密码前请验证您的身份</p>

        <div v-if="bioAvailable" >
          <button
            :disabled="reAuthLoading"
            class="w-full py-2.5 bg-zinc-700 hover:bg-zinc-600 disabled:bg-zinc-800 text-zinc-200 rounded-lg transition-colors cursor-pointer flex items-center justify-center gap-2 text-sm"
            @click="handleReAuthBio"
          >
            <span class="text-lg">👆</span>
            <span>{{ reAuthLoading ? "验证中..." : "生物识别验证" }}</span>
          </button>
          <div class="flex items-center gap-3 my-3">
            <div class="flex-1 h-px bg-zinc-700"></div>
            <span class="text-zinc-600 text-xs">或输入密码</span>
            <div class="flex-1 h-px bg-zinc-700"></div>
          </div>
        </div>

        <input
          v-model="reAuthPassword"
          type="password"
          placeholder="主密码"
          class="w-full px-3 py-2 bg-zinc-900 border border-zinc-700 rounded-lg text-zinc-100 placeholder-zinc-500 focus:outline-none focus:border-emerald-500 text-sm"
          @keyup.enter="handleReAuthPassword"
        />
        <div v-if="reAuthError" class="text-red-400 text-xs text-center bg-red-900/30 py-1.5 rounded-lg">
          {{ reAuthError }}
        </div>
        <div class="flex gap-2">
          <button
            class="flex-1 py-2 bg-zinc-700 hover:bg-zinc-600 text-zinc-300 rounded-lg transition-colors cursor-pointer text-sm"
            @click="cancelReAuth"
          >
            取消
          </button>
          <button
            :disabled="reAuthLoading"
            class="flex-1 py-2 bg-emerald-600 hover:bg-emerald-500 disabled:bg-emerald-800 text-white rounded-lg transition-colors cursor-pointer text-sm"
            @click="handleReAuthPassword"
          >
            {{ reAuthLoading ? "验证中..." : "验证" }}
          </button>
        </div>
      </div>
    </div>

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
