<script setup lang="ts">
import { ref, computed } from "vue"
import { invoke } from "@tauri-apps/api/core"
import { writeText } from "@tauri-apps/plugin-clipboard-manager"
import type { PasswordMode, GenConfig, PinConfig, PassphraseConfig } from "../types"
import { MODE_LABELS, MODE_ICONS } from "../types"
import TemplateEditor from "./TemplateEditor.vue"
import { useHistoryStore } from "../stores/history"
import { useTemplateStore } from "../stores/template"

const historyStore = useHistoryStore()
const templateStore = useTemplateStore()

const modes: PasswordMode[] = ["random", "pin", "passphrase", "template"]
const currentMode = ref<PasswordMode>("random")

const generated = ref("")
const copied = ref(false)
const generating = ref(false)
const saved = ref(false)

const randomConfig = ref<GenConfig>({
  length: 20,
  upper: true,
  lower: true,
  digits: true,
  symbols: true,
  exclude_ambiguous: false,
})

const pinConfig = ref<PinConfig>({
  length: 6,
  no_consecutive: false,
  no_sequential: false,
})

const passphraseConfig = ref<PassphraseConfig>({
  word_count: 4,
  separator: "-",
  capitalize: false,
})

const selectedTemplateId = ref<number | null>(null)

const saveLabel = ref("")
const saveUsername = ref("")
const saveCategoryId = ref<number | null>(null)
const showSavePanel = ref(false)
const saveSuccess = ref(false)

const separators = [
  { label: "-", value: "-" },
  { label: ".", value: "." },
  { label: "_", value: "_" },
  { label: " ", value: " " },
  { label: "#", value: "#" },
]

async function doGenerate() {
  generating.value = true
  copied.value = false
  saved.value = false
  try {
    switch (currentMode.value) {
      case "random":
        generated.value = await invoke("generate_password", {
          config: randomConfig.value,
        })
        break
      case "pin":
        generated.value = await invoke("generate_pin", {
          config: pinConfig.value,
        })
        break
      case "passphrase":
        generated.value = await invoke("generate_passphrase", {
          config: passphraseConfig.value,
        })
        break
      case "template": {
        if (!selectedTemplateId.value) break
        const tpl = templateStore.templates.find(
          (t) => t.id === selectedTemplateId.value
        )
        if (tpl) {
          generated.value = await invoke("generate_from_template", {
            pattern: tpl.pattern,
          })
        }
        break
      }
    }
  } catch (e: unknown) {
    generated.value = ""
  } finally {
    generating.value = false
  }
}

async function copyToClipboard() {
  if (!generated.value) return
  await writeText(generated.value)
  copied.value = true
  setTimeout(() => {
    copied.value = false
  }, 2000)
}

async function showSave() {
  if (!generated.value || saved.value) return
  await historyStore.loadCategories()
  showSavePanel.value = true
}

async function savePassword() {
  if (!generated.value) return
  await historyStore.savePassword(
    saveLabel.value || "未标注",
    saveUsername.value,
    generated.value,
    currentMode.value,
    saveCategoryId.value,
    selectedTemplateId.value,
  )
  saved.value = true
  saveSuccess.value = true
  setTimeout(() => {
    showSavePanel.value = false
    saveSuccess.value = false
    saveLabel.value = ""
    saveUsername.value = ""
    saveCategoryId.value = null
  }, 1500)
}

function cancelSave() {
  showSavePanel.value = false
}

const strengthLabel = computed(() => {
  const pw = generated.value
  if (!pw) return { text: "", color: "" }
  const len = pw.length
  const hasUpper = /[A-Z]/.test(pw)
  const hasLower = /[a-z]/.test(pw)
  const hasDigit = /[0-9]/.test(pw)
  const hasSymbol = /[^A-Za-z0-9]/.test(pw)
  const variety = [hasUpper, hasLower, hasDigit, hasSymbol].filter(Boolean).length

  if (len >= 16 && variety >= 3) return { text: "很强", color: "text-emerald-400" }
  if (len >= 12 && variety >= 2) return { text: "较强", color: "text-green-400" }
  if (len >= 8 && variety >= 2) return { text: "中等", color: "text-yellow-400" }
  return { text: "弱", color: "text-red-400" }
})

function onTemplateCreated() {
  templateStore.loadTemplates()
}
</script>

<template>
  <div class="flex flex-col h-full">
    <div class="flex gap-1 mb-4 bg-zinc-800 rounded-lg p-1">
      <button
        v-for="mode in modes"
        :key="mode"
        :class="[
          'flex-1 py-2 rounded-md text-xs font-medium transition-colors cursor-pointer whitespace-nowrap',
          currentMode === mode
            ? 'bg-emerald-600 text-white'
            : 'text-zinc-400 hover:text-zinc-200',
        ]"
        @click="currentMode = mode"
      >
        {{ MODE_ICONS[mode] }} {{ MODE_LABELS[mode] }}
      </button>
    </div>

    <div class="flex-1 space-y-4">
      <!-- Random Config -->
      <div v-if="currentMode === 'random'" class="space-y-3">
        <div class="flex items-center justify-between">
          <label class="text-zinc-400 text-sm">长度</label>
          <div class="flex items-center gap-2">
            <input
              v-model.number="randomConfig.length"
              type="range"
              min="8"
              max="128"
              class="w-28 accent-emerald-500"
            />
            <span class="text-zinc-200 text-sm font-mono w-8 text-right">{{
              randomConfig.length
            }}</span>
          </div>
        </div>
        <div class="flex flex-wrap gap-2">
          <label
            v-for="opt in [
              { key: 'upper' as const, label: 'A-Z' },
              { key: 'lower' as const, label: 'a-z' },
              { key: 'digits' as const, label: '0-9' },
              { key: 'symbols' as const, label: '符号' },
            ]"
            :key="opt.key"
            :class="[
              'flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-sm border cursor-pointer select-none transition-colors',
              randomConfig[opt.key]
                ? 'bg-emerald-900/30 border-emerald-700 text-emerald-400'
                : 'bg-zinc-800 border-zinc-700 text-zinc-500',
            ]"
            @click="
              randomConfig[opt.key] = !randomConfig[opt.key]
            "
          >
            {{ opt.label }}
          </label>
        </div>
        <label class="flex items-center gap-2 text-zinc-400 text-sm cursor-pointer">
          <input
            v-model="randomConfig.exclude_ambiguous"
            type="checkbox"
            class="accent-emerald-500"
          />
          排除易混淆字符 (I l 1 O 0)
        </label>
      </div>

      <!-- PIN Config -->
      <div v-if="currentMode === 'pin'" class="space-y-3">
        <div class="flex items-center justify-between">
          <label class="text-zinc-400 text-sm">长度</label>
          <div class="flex items-center gap-2">
            <input
              v-model.number="pinConfig.length"
              type="range"
              min="4"
              max="12"
              class="w-28 accent-emerald-500"
            />
            <span class="text-zinc-200 text-sm font-mono w-8 text-right">{{
              pinConfig.length
            }}</span>
          </div>
        </div>
        <label class="flex items-center gap-2 text-zinc-400 text-sm cursor-pointer">
          <input
            v-model="pinConfig.no_consecutive"
            type="checkbox"
            class="accent-emerald-500"
          />
          禁止连续相同数字
        </label>
        <label class="flex items-center gap-2 text-zinc-400 text-sm cursor-pointer">
          <input
            v-model="pinConfig.no_sequential"
            type="checkbox"
            class="accent-emerald-500"
          />
          禁止递增/递减序列
        </label>
      </div>

      <!-- Passphrase Config -->
      <div v-if="currentMode === 'passphrase'" class="space-y-3">
        <div class="flex items-center justify-between">
          <label class="text-zinc-400 text-sm">单词数</label>
          <div class="flex items-center gap-2">
            <input
              v-model.number="passphraseConfig.word_count"
              type="range"
              min="3"
              max="12"
              class="w-28 accent-emerald-500"
            />
            <span class="text-zinc-200 text-sm font-mono w-8 text-right">{{
              passphraseConfig.word_count
            }}</span>
          </div>
        </div>
        <div class="flex items-center justify-between">
          <label class="text-zinc-400 text-sm">分隔符</label>
          <div class="flex gap-1">
            <button
              v-for="s in separators"
              :key="s.value"
              :class="[
                'px-3 py-1 text-sm rounded cursor-pointer transition-colors',
                passphraseConfig.separator === s.value
                  ? 'bg-emerald-600 text-white'
                  : 'bg-zinc-800 text-zinc-400 hover:bg-zinc-700',
              ]"
              @click="passphraseConfig.separator = s.value"
            >
              {{ s.label === " " ? "空格" : s.label }}
            </button>
          </div>
        </div>
        <label class="flex items-center gap-2 text-zinc-400 text-sm cursor-pointer">
          <input
            v-model="passphraseConfig.capitalize"
            type="checkbox"
            class="accent-emerald-500"
          />
          首字母大写
        </label>
      </div>

      <!-- Template Mode -->
      <div v-if="currentMode === 'template'" class="space-y-3">
        <div v-if="templateStore.templates.length === 0" class="text-zinc-500 text-sm text-center py-4">
          暂无模板，请在下方创建
        </div>
        <div v-else class="space-y-2">
          <label class="text-zinc-400 text-sm">选择模板</label>
          <div class="flex flex-wrap gap-2">
            <button
              v-for="tpl in templateStore.templates"
              :key="tpl.id"
              :class="[
                'px-3 py-1.5 text-sm rounded-lg border cursor-pointer transition-colors',
                selectedTemplateId === tpl.id
                  ? 'bg-emerald-900/30 border-emerald-700 text-emerald-400'
                  : 'bg-zinc-800 border-zinc-700 text-zinc-400 hover:border-zinc-600',
              ]"
              @click="selectedTemplateId = tpl.id"
            >
              {{ tpl.name }}
            </button>
          </div>
        </div>
        <TemplateEditor @created="onTemplateCreated" />
      </div>

      <!-- Generate Button -->
      <button
        :disabled="generating || (currentMode === 'template' && !selectedTemplateId)"
        class="w-full py-2.5 bg-emerald-600 hover:bg-emerald-500 disabled:bg-zinc-700 disabled:text-zinc-500 text-white font-medium rounded-lg transition-colors cursor-pointer"
        @click="doGenerate"
      >
        {{ generating ? "生成中..." : "生成密码" }}
      </button>

      <!-- Generated Output -->
      <div
        v-if="generated"
        class="p-4 bg-zinc-800 rounded-xl border border-zinc-700 relative"
      >
        <div class="flex items-center justify-between mb-2">
          <span :class="['text-xs font-medium', strengthLabel.color]">
            强度: {{ strengthLabel.text }}
          </span>
          <div class="flex gap-2">
            <button
              :class="[
                'text-xs px-2 py-1 rounded transition-colors cursor-pointer',
                saved
                  ? 'bg-emerald-800 text-emerald-300 cursor-default'
                  : 'bg-zinc-700 hover:bg-zinc-600 text-zinc-300',
              ]"
              :disabled="saved"
              @click="!saved && showSave()"
            >
              {{ saved ? "已保存" : "保存" }}
            </button>
            <button
              :class="[
                'text-xs px-2 py-1 rounded transition-colors cursor-pointer',
                copied
                  ? 'bg-emerald-800 text-emerald-300'
                  : 'bg-zinc-700 hover:bg-zinc-600 text-zinc-300',
              ]"
              @click="copyToClipboard"
            >
              {{ copied ? "已复制" : "复制" }}
            </button>
          </div>
        </div>
        <div
          class="font-mono text-lg text-emerald-400 break-all select-all cursor-pointer"
          @click="copyToClipboard"
        >
          {{ generated }}
        </div>
      </div>
    </div>

    <!-- Save Panel Overlay -->
    <div
      v-if="showSavePanel"
      class="fixed inset-0 bg-black/60 flex items-center justify-center z-50"
      @click.self="cancelSave"
    >
      <div class="bg-zinc-800 border border-zinc-700 rounded-xl p-6 w-96 space-y-4">
        <h3 class="text-lg font-semibold text-zinc-100">保存密码</h3>
        <input
          v-model="saveLabel"
          placeholder="用途标注 (如: Gmail)"
          class="w-full px-3 py-2 bg-zinc-900 border border-zinc-700 rounded-lg text-zinc-100 placeholder-zinc-500 focus:outline-none focus:border-emerald-500"
        />
        <input
          v-model="saveUsername"
          placeholder="用户名 (可选)"
          class="w-full px-3 py-2 bg-zinc-900 border border-zinc-700 rounded-lg text-zinc-100 placeholder-zinc-500 focus:outline-none focus:border-emerald-500"
        />
        <select
          v-model="saveCategoryId"
          class="w-full px-3 py-2 bg-zinc-900 border border-zinc-700 rounded-lg text-zinc-100 focus:outline-none focus:border-emerald-500"
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
        <div class="flex gap-2">
          <button
            class="flex-1 py-2 bg-zinc-700 hover:bg-zinc-600 text-zinc-300 rounded-lg transition-colors cursor-pointer"
            @click="cancelSave"
          >
            取消
          </button>
          <button
            class="flex-1 py-2 bg-emerald-600 hover:bg-emerald-500 text-white rounded-lg transition-colors cursor-pointer"
            @click="savePassword"
          >
            {{ saveSuccess ? "已保存" : "保存" }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
