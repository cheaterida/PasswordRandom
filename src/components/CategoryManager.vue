<script setup lang="ts">
import { ref } from "vue"
import { useHistoryStore } from "../stores/history"

const historyStore = useHistoryStore()
const expanded = ref(false)

const newName = ref("")
const newIcon = ref("")
const error = ref("")

const icons = ["🔑", "📧", "🏦", "🎮", "🛒", "💬", "🌐", "💼", "🎵", "📱"]

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

async function removeCategory(id: number) {
  await historyStore.deleteCategory(id)
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
  </div>
</template>
