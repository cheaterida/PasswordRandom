<script setup lang="ts">
import { ref, onMounted } from "vue"
import { invoke } from "@tauri-apps/api/core"
import { checkStatus, authenticate } from "@tauri-apps/plugin-biometric"
import { useAuthStore } from "../stores/auth"

const emit = defineEmits<{ unlocked: [] }>()

const auth = useAuthStore()
const isSetup = ref(false)
const password = ref("")
const confirmPassword = ref("")
const error = ref("")
const loading = ref(false)

const bioAvailable = ref(false)
const bioEnabled = ref(false)
const bioLoading = ref(false)
const showBioPrompt = ref(false)

onMounted(async () => {
  await auth.checkStatus()
  isSetup.value = !auth.initialized

  try {
    const status = await checkStatus()
    bioAvailable.value = status.isAvailable
  } catch {
    bioAvailable.value = false
  }

  if (!isSetup.value && bioAvailable.value) {
    try {
      bioEnabled.value = await invoke<boolean>("biometric_is_enabled")
    } catch {
      bioEnabled.value = false
    }
  }
})

async function handleSubmit() {
  error.value = ""
  if (!password.value) {
    error.value = "请输入密码"
    return
  }

  if (isSetup.value) {
    if (password.value.length < 6) {
      error.value = "主密码至少 6 位"
      return
    }
    if (password.value !== confirmPassword.value) {
      error.value = "两次输入密码不一致"
      return
    }
  }

  loading.value = true
  try {
    if (isSetup.value) {
      await invoke("init_master_key", { password: password.value })
      auth.setLocked(false)
      if (bioAvailable.value) {
        showBioPrompt.value = true
      } else {
        emit("unlocked")
      }
    } else {
      const ok = await invoke<boolean>("unlock", { password: password.value })
      if (!ok) {
        error.value = "密码错误"
        return
      }
      auth.setLocked(false)
      if (bioAvailable.value && !bioEnabled.value) {
        showBioPrompt.value = true
      } else {
        emit("unlocked")
      }
    }
  } catch (e: unknown) {
    error.value = String(e)
  } finally {
    loading.value = false
  }
}

async function handleBioAuth() {
  bioLoading.value = true
  error.value = ""
  try {
    await authenticate("验证生物以解锁")
    await invoke("biometric_unlock")
    auth.setLocked(false)
    emit("unlocked")
  } catch (e: unknown) {
    error.value = "生物验证失败，请使用密码"
  } finally {
    bioLoading.value = false
  }
}

async function enableBio() {
  try {
    await invoke("biometric_enable")
    bioEnabled.value = true
    showBioPrompt.value = false
    emit("unlocked")
  } catch (e: unknown) {
    error.value = "启用失败: " + String(e)
    showBioPrompt.value = false
    emit("unlocked")
  }
}

function skipBio() {
  showBioPrompt.value = false
  emit("unlocked")
}
</script>

<template>
  <div class="flex items-center justify-center min-h-screen bg-zinc-900">
    <div class="w-full max-w-sm p-8">
      <!-- Setup/Unlock Screen -->
      <div v-if="!showBioPrompt" class="text-center mb-8">
        <div class="text-5xl mb-4">🔐</div>
        <h1 class="text-2xl font-bold text-zinc-100">
          {{ isSetup ? "设置主密码" : "解锁密码库" }}
        </h1>
        <p class="text-zinc-400 text-sm mt-2">
          {{
            isSetup
              ? "请设置一个强密码来保护你的密码数据"
              : "请输入你的主密码以继续"
          }}
        </p>
      </div>

      <!-- Bio Prompt -->
      <div v-if="showBioPrompt" class="text-center mb-8">
        <div class="text-6xl mb-4">👆</div>
        <h1 class="text-xl font-bold text-zinc-100">启用生物解锁</h1>
        <p class="text-zinc-400 text-sm mt-2">
          下次你可以使用生物快速解锁，无需输入密码
        </p>
      </div>

      <div v-if="!showBioPrompt" class="space-y-4">
        <!-- Biometric Button -->
        <button
          v-if="!isSetup && bioAvailable && bioEnabled"
          :disabled="bioLoading"
          class="w-full py-3 bg-zinc-700 hover:bg-zinc-600 disabled:bg-zinc-800 text-zinc-200 font-medium rounded-lg transition-colors cursor-pointer flex items-center justify-center gap-2"
          @click="handleBioAuth"
        >
          <span class="text-xl">👆</span>
          <span>{{ bioLoading ? "验证中..." : "生物解锁" }}</span>
        </button>

        <div
          v-if="!isSetup && bioAvailable && bioEnabled"
          class="flex items-center gap-3 my-3"
        >
          <div class="flex-1 h-px bg-zinc-700"></div>
          <span class="text-zinc-600 text-xs">或使用密码</span>
          <div class="flex-1 h-px bg-zinc-700"></div>
        </div>

        <div>
          <input
            v-model="password"
            type="password"
            :placeholder="isSetup ? '主密码 (至少 6 位)' : '主密码'"
            class="w-full px-4 py-3 bg-zinc-800 border border-zinc-700 rounded-lg text-zinc-100 placeholder-zinc-500 focus:outline-none focus:border-emerald-500 transition-colors"
            @keyup.enter="handleSubmit"
          />
        </div>

        <div v-if="isSetup">
          <input
            v-model="confirmPassword"
            type="password"
            placeholder="确认主密码"
            class="w-full px-4 py-3 bg-zinc-800 border border-zinc-700 rounded-lg text-zinc-100 placeholder-zinc-500 focus:outline-none focus:border-emerald-500 transition-colors"
            @keyup.enter="handleSubmit"
          />
        </div>

        <div v-if="error" class="text-red-400 text-sm text-center bg-red-900/30 py-2 rounded-lg">
          {{ error }}
        </div>

        <button
          :disabled="loading"
          class="w-full py-3 bg-emerald-600 hover:bg-emerald-500 disabled:bg-emerald-800 text-white font-medium rounded-lg transition-colors cursor-pointer"
          @click="handleSubmit"
        >
          {{ loading ? "处理中..." : isSetup ? "设置并进入" : "解锁" }}
        </button>
      </div>

      <!-- Bio Prompt Actions -->
      <div v-if="showBioPrompt" class="space-y-3">
        <button
          class="w-full py-3 bg-emerald-600 hover:bg-emerald-500 text-white font-medium rounded-lg transition-colors cursor-pointer"
          @click="enableBio"
        >
          启用生物解锁
        </button>
        <button
          class="w-full py-3 bg-zinc-700 hover:bg-zinc-600 text-zinc-400 rounded-lg transition-colors cursor-pointer"
          @click="skipBio"
        >
          暂不启用
        </button>
      </div>
    </div>
  </div>
</template>
