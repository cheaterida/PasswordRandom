<script setup lang="ts">
import { ref } from "vue"
import { invoke } from "@tauri-apps/api/core"
import { checkStatus, authenticate } from "@tauri-apps/plugin-biometric"
import { useHistoryStore } from "../stores/history"

const historyStore = useHistoryStore()
const expanded = ref(false)

const newName = ref("")
const newIcon = ref("")
const error = ref("")

const showReAuth = ref(false)
const reAuthPassword = ref("")
const reAuthError = ref("")
const reAuthLoading = ref(false)
const bioAvailable = ref(false)
let pendingDeleteId: number | null = null

const icons = ["🔑", "📧", "🏦", "🎮", "🛒", "💬", "🌐", "💼", "🎵", "📱"]

async function checkBioAvailable() {
  try {
    const s = await checkStatus()
    bioAvailable.value = s.isAvailable
  } catch { bioAvailable.value = false }
}

async function createCategory() {
  error.value = ""
  if (!newName.value.trim()) {
    error.value = "请输入分类名称"
    return
  }
  try {
    await historyStore.createCategory(
      newName.value.trim(),
      newIcon.value || "📁",
    )
    newName.value = ""
    newIcon.value = ""
  } catch (e: unknown) {
    error.value = String(e)
  }
}

function removeCategory(id: number) {
  pendingDeleteId = id
  reAuthPassword.value = ""
  reAuthError.value = ""
  showReAuth.value = true
  checkBioAvailable()
}

async function handleReAuthPassword() {
  reAuthError.value = ""
  reAuthLoading.value = true
  try {
    const ok = await invoke<boolean>("unlock", { password: reAuthPassword.value })
    if (!ok) { reAuthError.value = "密码错误"; return }
    doDelete()
  } catch (e: unknown) { reAuthError.value = String(e) } finally {
    reAuthLoading.value = false
  }
}

async function handleReAuthBio() {
  reAuthError.value = ""
  reAuthLoading.value = true
  try {
    await authenticate("验证身份以删除分类")
    await invoke("biometric_unlock")
    doDelete()
  } catch { reAuthError.value = "验证失败" } finally {
    reAuthLoading.value = false
  }
}

async function doDelete() {
  showReAuth.value = false
  if (pendingDeleteId != null) {
    await historyStore.deleteCategory(pendingDeleteId)
    pendingDeleteId = null
  }
}

function cancelReAuth() {
  showReAuth.value = false
  pendingDeleteId = null
}
</script>

<template>
  <div class="border-t border-zinc-700 pt-3 mt-3">
    <button
      class="flex items-center gap-1 text-sm text-zinc-400 hover:text-zinc-200 cursor-pointer"
      @click="expanded = !expanded"
    >
      <span>{{ expanded ? "▾" : "▸" }}</span>
      管理分类 ({{ historyStore.categories.length }})
    </button>

    <div v-if="expanded" class="mt-3 space-y-3">
      <div v-if="historyStore.categories.length > 0" class="flex flex-wrap gap-2">
        <div
          v-for="cat in historyStore.categories"
          :key="cat.id"
          class="flex items-center gap-1.5 px-3 py-1.5 bg-zinc-800 rounded-lg text-sm"
        >
          <span>{{ cat.icon }}</span>
          <span class="text-zinc-300">{{ cat.name }}</span>
          <button
            class="text-red-400 hover:text-red-300 text-xs ml-1 cursor-pointer"
            @click="removeCategory(cat.id)"
          >
            ✕
          </button>
        </div>
      </div>

      <div class="flex gap-2">
        <input
          v-model="newName"
          placeholder="分类名称"
          class="flex-1 px-3 py-2 bg-zinc-800 border border-zinc-700 rounded-lg text-zinc-200 placeholder-zinc-500 focus:outline-none focus:border-emerald-500 text-sm"
          @keyup.enter="createCategory"
        />
        <button
          class="px-4 py-2 bg-zinc-700 hover:bg-zinc-600 text-zinc-300 rounded-lg text-sm transition-colors cursor-pointer"
          @click="createCategory"
        >
          添加
        </button>
      </div>

      <div class="flex flex-wrap gap-1.5">
        <span class="text-zinc-500 text-xs">图标:</span>
        <button
          v-for="icon in icons"
          :key="icon"
          :class="[
            'px-2 py-0.5 text-sm rounded cursor-pointer transition-colors',
            newIcon === icon
              ? 'bg-emerald-900/40 border border-emerald-700'
              : 'bg-zinc-800 border border-zinc-800 hover:border-zinc-600',
          ]"
          @click="newIcon = icon"
        >
          {{ icon }}
        </button>
      </div>

      <div v-if="error" class="text-red-400 text-xs">{{ error }}</div>
    </div>

    <!-- Re-auth Overlay -->
    <div
      v-if="showReAuth"
      class="fixed inset-0 bg-black/60 flex items-center justify-center z-50"
      @click.self="cancelReAuth"
    >
      <div class="bg-zinc-800 border border-zinc-700 rounded-xl p-6 w-80 space-y-4 mx-4">
        <h3 class="text-lg font-semibold text-zinc-100 text-center">验证身份</h3>
        <p class="text-zinc-400 text-sm text-center">删除分类前请验证您的身份</p>

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
            <span class="text-zinc-600 text-xs">或</span>
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
  </div>
</template>
