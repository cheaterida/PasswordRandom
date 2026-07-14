<script setup lang="ts">
import { ref } from "vue"
import { invoke } from "@tauri-apps/api/core"
import MasterLock from "./components/MasterLock.vue"
import GeneratorPanel from "./components/GeneratorPanel.vue"
import PasswordHistory from "./components/PasswordHistory.vue"
import Settings from "./components/Settings.vue"

const locked = ref(true)
const activeTab = ref<"generate" | "history" | "settings">("generate")
const showAbout = ref(false)

async function onUnlocked() {
  locked.value = false
}

async function onLock() {
  try {
    await invoke("lock")
  } catch {
    // ignore
  }
  locked.value = true
}
</script>

<template>
  <div class="flex flex-col h-screen bg-zinc-900">
    <MasterLock v-if="locked" @unlocked="onUnlocked" />
    <template v-else>
      <!-- Safe area spacer for status bar -->
      <div class="shrink-0 bg-zinc-900" style="height: env(safe-area-inset-top, 0px)"></div>

      <!-- Header -->
      <header class="flex items-center justify-between px-4 py-2.5 bg-zinc-800 border-b border-zinc-700 shrink-0">
        <h1
          class="text-base font-bold text-zinc-100 cursor-pointer hover:text-emerald-400 transition-colors"
          @click="showAbout = true"
        >
          🔐 密码随机生成器
        </h1>
        <button
          class="text-sm text-zinc-400 hover:text-zinc-200 cursor-pointer transition-colors"
          @click="onLock"
        >
          🔒 锁定
        </button>
      </header>

      <!-- Tab Bar -->
      <nav class="flex bg-zinc-800 border-b border-zinc-700 shrink-0">
        <button
          :class="[
            'flex-1 flex items-center justify-center py-2.5 text-sm font-medium cursor-pointer transition-colors',
            activeTab === 'generate'
              ? 'text-emerald-400 border-b-2 border-emerald-400'
              : 'text-zinc-500 hover:text-zinc-300',
          ]"
          @click="activeTab = 'generate'"
        >
          生成
        </button>
        <button
          :class="[
            'flex-1 flex items-center justify-center py-2.5 text-sm font-medium cursor-pointer transition-colors',
            activeTab === 'history'
              ? 'text-emerald-400 border-b-2 border-emerald-400'
              : 'text-zinc-500 hover:text-zinc-300',
          ]"
          @click="activeTab = 'history'"
        >
          历史
        </button>
        <button
          :class="[
            'flex-1 flex items-center justify-center py-2.5 text-sm font-medium cursor-pointer transition-colors',
            activeTab === 'settings'
              ? 'text-emerald-400 border-b-2 border-emerald-400'
              : 'text-zinc-500 hover:text-zinc-300',
          ]"
          @click="activeTab = 'settings'"
        >
          设置
        </button>
      </nav>

      <!-- Main Content -->
      <main class="flex-1 overflow-y-auto px-6 py-4">
        <GeneratorPanel v-if="activeTab === 'generate'" />
        <PasswordHistory v-else-if="activeTab === 'history'" />
        <Settings v-else />
      </main>

      <!-- About Modal -->
      <div
        v-if="showAbout"
        class="fixed inset-0 bg-black/60 flex items-center justify-center z-50"
        @click.self="showAbout = false"
      >
        <div class="bg-zinc-800 border border-zinc-700 rounded-xl p-6 w-80 mx-4 max-h-[80vh] overflow-y-auto">
          <div class="text-center mb-4">
            <div class="text-4xl mb-2">🔐</div>
            <h2 class="text-xl font-bold text-zinc-100">密码随机生成器</h2>
            <p class="text-zinc-400 text-sm mt-1">PasswordRandom</p>
          </div>

          <div class="space-y-3 text-sm text-zinc-300">
            <div class="bg-zinc-900 rounded-lg p-3 space-y-2">
              <div class="flex justify-between">
                <span class="text-zinc-500">版本</span>
                <span class="text-zinc-200 font-mono">v0.1.0</span>
              </div>
              <div class="flex justify-between">
                <span class="text-zinc-500">开发者</span>
                <span class="text-zinc-200">cheater</span>
              </div>
            </div>

            <div class="bg-zinc-900 rounded-lg p-3">
              <p class="text-zinc-500 mb-2 text-xs">项目仓库</p>
              <p class="text-emerald-400 font-mono text-xs break-all">
                github.com/cheaterida/PasswordRandom
              </p>
            </div>

            <div class="bg-zinc-900 rounded-lg p-3">
              <p class="text-zinc-500 mb-2 text-xs">使用介绍</p>
              <ul class="space-y-1 text-xs text-zinc-400">
                <li>• 支持随机密码、PIN 数字、密语和模板四种生成模式</li>
                <li>• 生成后可一键复制或加密保存到本地</li>
                <li>• 支持密码分类管理、搜索和历史记录</li>
                <li>• 所有数据经 AES-256-GCM 加密存储</li>
                <li>• 支持手动添加自有密码记录</li>
              </ul>
            </div>

            <div class="text-center text-zinc-500 text-xs py-1">
              <p>愿你的每一个密码都安全无忧 🙏</p>
              <p class="mt-1">Happy coding & stay secure!</p>
            </div>
          </div>

          <button
            class="w-full mt-4 py-2 bg-zinc-700 hover:bg-zinc-600 text-zinc-300 rounded-lg cursor-pointer transition-colors text-sm"
            @click="showAbout = false"
          >
            关闭
          </button>
        </div>
      </div>
    </template>
  </div>
</template>
