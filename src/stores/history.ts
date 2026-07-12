import { defineStore } from "pinia"
import { invoke } from "@tauri-apps/api/core"
import type { PasswordDisplay, Category } from "../types"

export const useHistoryStore = defineStore("history", {
  state: () => ({
    passwords: [] as PasswordDisplay[],
    categories: [] as Category[],
    search: "",
    selectedCategory: null as number | null,
    loading: false,
  }),
  actions: {
    async loadPasswords() {
      this.loading = true
      try {
        this.passwords = await invoke("get_passwords", {
          categoryId: this.selectedCategory,
          search: this.search || null,
        })
      } finally {
        this.loading = false
      }
    },
    async loadCategories() {
      this.categories = await invoke("get_categories")
    },
    async savePassword(
      label: string,
      username: string,
      password: string,
      mode: string,
      categoryId: number | null,
      templateId: number | null,
    ) {
      await invoke("save_password", {
        label,
        username,
        password,
        mode,
        categoryId,
        templateId,
      })
      await this.loadPasswords()
    },
    async deletePassword(id: number) {
      await invoke("delete_password", { id })
      this.passwords = this.passwords.filter((p) => p.id !== id)
    },
    async createCategory(name: string, icon: string) {
      await invoke("create_category", { name, icon })
      await this.loadCategories()
    },
    async deleteCategory(id: number) {
      await invoke("delete_category", { id })
      this.selectedCategory = null
      await this.loadCategories()
      await this.loadPasswords()
    },
  },
})
