<script setup lang="ts">
import { ref, onMounted } from "vue"
import { invoke } from "@tauri-apps/api/core"
import { writeText } from "@tauri-apps/plugin-clipboard-manager"
import { checkStatus, authenticate } from "@tauri-apps/plugin-biometric"

const emit = defineEmits<{ showAbout: [] }>()

const bioAvailable = ref(false)
const bioEnabled = ref(false)
const bioLoading = ref(false)

const isLight = ref(false)
const themeLoading = ref(false)

const dbPath = ref("")
const dbPathRevealed = ref(false)
const dbPathLoading = ref(false)

const exportData = ref("")
const exportRevealed = ref(false)
const exportLoading = ref(false)

const showChangePw = ref(false)
const oldPassword = ref("")
const newPassword = ref("")
const confirmNewPassword = ref("")
const changePwError = ref("")
const changePwSuccess = ref("")
const changePwLoading = ref(false)

const showReAuth = ref(false)
const reAuthPassword = ref("")
const reAuthError = ref("")
const reAuthLoading = ref(false)
let reAuthResolve: (() => void) | null = null

onMounted(async () => {
  isLight.value = localStorage.getItem("theme") === "light"
  applyTheme()
  try {
    const status = await checkStatus()
    bioAvailable.value = status.isAvailable
    if (bioAvailable.value) {
      bioEnabled.value = await invoke<boolean>("biometric_is_enabled")
    }
  } catch {
    bioAvailable.value = false
  }
})

function runWithReAuth(action: () => void) {
  reAuthPassword.value = ""
  reAuthError.value = ""
  showReAuth.value = true
  reAuthResolve = action
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
    await authenticate("验证身份以继续")
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
  reAuthPassword.value = ""
  if (reAuthResolve) {
    const fn = reAuthResolve
    reAuthResolve = null
    fn()
  }
}

function cancelReAuth() {
  showReAuth.value = false
  reAuthError.value = ""
  reAuthResolve = null
}

async function toggleBio() {
  bioLoading.value = true
  try {
    if (bioEnabled.value) {
      await invoke("biometric_disable")
      bioEnabled.value = false
    } else {
      runWithReAuth(async () => {
        await invoke("biometric_enable")
        bioEnabled.value = true
        bioLoading.value = false
      })
      return
    }
  } catch (e: unknown) {
    // ignore
  } finally {
    bioLoading.value = false
  }
}

async function handleChangePw() {
  changePwError.value = ""
  changePwSuccess.value = ""
  if (!oldPassword.value) {
    changePwError.value = "请输入旧密码"
    return
  }
  if (!newPassword.value || newPassword.value.length < 6) {
    changePwError.value = "新密码至少 6 位"
    return
  }
  if (newPassword.value !== confirmNewPassword.value) {
    changePwError.value = "两次输入密码不一致"
    return
  }
  changePwLoading.value = true
  try {
    await invoke("change_master_password", {
      oldPassword: oldPassword.value,
      newPassword: newPassword.value,
    })
    changePwSuccess.value = "密码修改成功"
    oldPassword.value = ""
    newPassword.value = ""
    confirmNewPassword.value = ""
    setTimeout(() => {
      showChangePw.value = false
      changePwSuccess.value = ""
    }, 1500)
  } catch (e: unknown) {
    changePwError.value = String(e)
  } finally {
    changePwLoading.value = false
  }
}

async function revealDbPath() {
  dbPathRevealed.value = false
  dbPathLoading.value = true
  try {
    dbPath.value = await invoke<string>("get_db_path")
    dbPathRevealed.value = true
  } catch (e: unknown) {
    dbPath.value = "获取失败"
  } finally {
    dbPathLoading.value = false
  }
}

async function copyDbPath() {
  if (dbPath.value) {
    await writeText(dbPath.value)
  }
}

function toggleDbPath() {
  if (!dbPathRevealed.value) {
    runWithReAuth(revealDbPath)
  } else {
    dbPathRevealed.value = false
    dbPath.value = ""
  }
}

async function doExport() {
  exportLoading.value = true
  try {
    const data = await invoke<string>("export_data")
    exportData.value = data
    exportRevealed.value = true
  } catch (e: unknown) {
    exportData.value = ""
  } finally {
    exportLoading.value = false
  }
}

function triggerExport() {
  exportRevealed.value = false
  exportData.value = ""
  runWithReAuth(doExport)
}

async function copyExport() {
  if (exportData.value) {
    await writeText(exportData.value)
  }
}

function hideExport() {
  exportRevealed.value = false
  exportData.value = ""
}

function toggleExport() {
  if (exportRevealed.value) {
    hideExport()
  } else {
    triggerExport()
  }
}

function applyTheme() {
  if (isLight.value) {
    document.documentElement.classList.add("light")
  } else {
    document.documentElement.classList.remove("light")
  }
}

async function toggleTheme() {
  themeLoading.value = true
  isLight.value = !isLight.value
  applyTheme()
  localStorage.setItem("theme", isLight.value ? "light" : "dark")
  themeLoading.value = false
}
</script>

<template>
  <div class="flex flex-col h-full space-y-4">
    <!-- Biometric -->
    <div
      v-if="bioAvailable"
      class="bg-zinc-800 rounded-xl p-4 border border-zinc-700"
    >
      <div class="flex items-center justify-between">
        <div>
          <div class="text-zinc-200 text-sm font-medium">生物识别解锁</div>
          <div class="text-zinc-500 text-xs mt-0.5">
            {{ bioEnabled ? "已启用" : "已禁用" }}
          </div>
        </div>
        <button
          :disabled="bioLoading"
          :class="[
            'w-12 h-7 rounded-full transition-colors cursor-pointer shrink-0 relative',
            bioEnabled ? 'bg-emerald-600' : 'bg-zinc-600',
          ]"
          @click="toggleBio"
        >
          <div
            :class="[
              'absolute top-0.5 w-6 h-6 rounded-full bg-white transition-transform',
              bioEnabled ? 'translate-x-5' : 'translate-x-0.5',
            ]"
          />
        </button>
      </div>
    </div>

    <!-- Change Master Password -->
    <div class="bg-zinc-800 rounded-xl p-4 border border-zinc-700">
      <div
        class="flex items-center justify-between cursor-pointer"
        @click="showChangePw = !showChangePw"
      >
        <div>
          <div class="text-zinc-200 text-sm font-medium">修改主密码</div>
          <div class="text-zinc-500 text-xs mt-0.5">更改您的解锁密码</div>
        </div>
        <span class="text-zinc-500 text-sm">{{ showChangePw ? "▾" : "▸" }}</span>
      </div>

      <div v-if="showChangePw" class="mt-3 space-y-2">
        <input
          v-model="oldPassword"
          type="password"
          placeholder="当前主密码"
          class="w-full px-3 py-2 bg-zinc-900 border border-zinc-700 rounded-lg text-zinc-100 placeholder-zinc-500 focus:outline-none focus:border-emerald-500 text-sm"
        />
        <input
          v-model="newPassword"
          type="password"
          placeholder="新主密码 (至少 6 位)"
          class="w-full px-3 py-2 bg-zinc-900 border border-zinc-700 rounded-lg text-zinc-100 placeholder-zinc-500 focus:outline-none focus:border-emerald-500 text-sm"
        />
        <input
          v-model="confirmNewPassword"
          type="password"
          placeholder="确认新主密码"
          class="w-full px-3 py-2 bg-zinc-900 border border-zinc-700 rounded-lg text-zinc-100 placeholder-zinc-500 focus:outline-none focus:border-emerald-500 text-sm"
          @keyup.enter="handleChangePw"
        />
        <div v-if="changePwError" class="text-red-400 text-xs">{{ changePwError }}</div>
        <div v-if="changePwSuccess" class="text-emerald-400 text-xs">{{ changePwSuccess }}</div>
        <button
          :disabled="changePwLoading"
          class="w-full py-2 bg-emerald-600 hover:bg-emerald-500 disabled:bg-emerald-800 text-white text-sm rounded-lg transition-colors cursor-pointer"
          @click="handleChangePw"
        >
          {{ changePwLoading ? "处理中..." : "修改密码" }}
        </button>
      </div>
    </div>

    <!-- Database Path -->
    <div class="bg-zinc-800 rounded-xl p-4 border border-zinc-700">
      <div
        class="flex items-center justify-between cursor-pointer"
        @click="toggleDbPath"
      >
        <div>
          <div class="text-zinc-200 text-sm font-medium">密码库位置</div>
          <div class="text-zinc-500 text-xs mt-0.5">
            {{ dbPathRevealed ? "点击隐藏" : dbPathLoading ? "加载中..." : "需验证身份后查看" }}
          </div>
        </div>
        <span class="text-zinc-500 text-sm">{{ dbPathRevealed ? "▴" : "▸" }}</span>
      </div>

      <div
        v-if="dbPathRevealed"
        class="mt-3 p-2 bg-zinc-900 rounded-lg border border-zinc-700"
      >
        <div class="flex items-center justify-between gap-2">
          <div class="text-zinc-300 text-xs font-mono break-all select-all">
            {{ dbPath }}
          </div>
          <button
            class="text-xs px-2 py-1 rounded bg-zinc-700 hover:bg-zinc-600 text-zinc-400 hover:text-zinc-200 shrink-0 cursor-pointer transition-colors"
            @click="copyDbPath"
          >
            复制
          </button>
        </div>
      </div>
    </div>

    <!-- Export -->
    <div class="bg-zinc-800 rounded-xl p-4 border border-zinc-700">
      <div
        class="flex items-center justify-between cursor-pointer"
        @click="toggleExport"
      >
        <div>
          <div class="text-zinc-200 text-sm font-medium">加密导出</div>
          <div class="text-zinc-500 text-xs mt-0.5">
            {{ exportRevealed ? "点击隐藏" : exportLoading ? "加密中..." : "需验证身份后导出" }}
          </div>
        </div>
        <span class="text-zinc-500 text-sm">{{ exportRevealed ? "▴" : "▸" }}</span>
      </div>

      <div v-if="exportRevealed" class="mt-3 space-y-2">
        <div class="text-zinc-400 text-xs">导出的加密数据 (Base64)，可复制保存到文件：</div>
        <div class="relative">
          <div class="text-zinc-300 text-xs font-mono break-all bg-zinc-900 rounded-lg p-2 border border-zinc-700 max-h-32 overflow-y-auto select-all">
            {{ exportData }}
          </div>
        </div>
        <button
          class="w-full py-1.5 bg-emerald-600 hover:bg-emerald-500 text-white text-sm rounded-lg transition-colors cursor-pointer"
          @click="copyExport"
        >
          复制导出数据
        </button>
      </div>
    </div>

    <!-- Theme -->
    <div class="bg-zinc-800 rounded-xl p-4 border border-zinc-700">
      <div class="flex items-center justify-between">
        <div>
          <div class="text-zinc-200 text-sm font-medium">
            {{ isLight ? "浅色主题" : "深色主题" }}
          </div>
          <div class="text-zinc-500 text-xs mt-0.5">切换界面明亮度</div>
        </div>
        <button
          :disabled="themeLoading"
          :class="[
            'w-12 h-7 rounded-full transition-colors cursor-pointer shrink-0 relative',
            isLight ? 'bg-amber-500' : 'bg-zinc-600',
          ]"
          @click="toggleTheme"
        >
          <div
            :class="[
              'absolute top-0.5 w-6 h-6 rounded-full bg-white transition-transform flex items-center justify-center text-xs',
              isLight ? 'translate-x-5' : 'translate-x-0.5',
            ]"
          >
            {{ isLight ? "☀️" : "🌙" }}
          </div>
        </button>
      </div>
    </div>

    <!-- About -->
    <button
      class="w-full py-3 bg-zinc-800 hover:bg-zinc-700 border border-zinc-700 rounded-xl text-zinc-400 hover:text-zinc-200 text-sm transition-colors cursor-pointer"
      @click="emit('showAbout')"
    >
      关于应用
    </button>

    <!-- Re-auth Overlay -->
    <div
      v-if="showReAuth"
      class="fixed inset-0 bg-black/60 flex items-center justify-center z-50"
      @click.self="cancelReAuth"
    >
      <div class="bg-zinc-800 border border-zinc-700 rounded-xl p-6 w-80 space-y-4 mx-4">
        <h3 class="text-lg font-semibold text-zinc-100 text-center">验证身份</h3>
        <p class="text-zinc-400 text-sm text-center">输入主密码或使用生物识别</p>

        <div v-if="bioAvailable && bioEnabled">
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
