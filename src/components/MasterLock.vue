<script setup lang="ts">
import { ref, onMounted } from "vue"
import { invoke } from "@tauri-apps/api/core"
import { useAuthStore } from "../stores/auth"

const emit = defineEmits<{ unlocked: [] }>()

const auth = useAuthStore()
const isSetup = ref(false)
const password = ref("")
const confirmPassword = ref("")
const error = ref("")
const loading = ref(false)

onMounted(async () => {
  await auth.checkStatus()
  isSetup.value = !auth.initialized
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
      emit("unlocked")
    } else {
      const ok = await invoke<boolean>("unlock", { password: password.value })
      if (!ok) {
        error.value = "密码错误"
        return
      }
      auth.setLocked(false)
      emit("unlocked")
    }
  } catch (e: unknown) {
    error.value = String(e)
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="flex items-center justify-center min-h-screen bg-zinc-900">
    <div class="w-full max-w-sm p-8">
      <div class="text-center mb-8">
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

      <div class="space-y-4">
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
    </div>
  </div>
</template>
