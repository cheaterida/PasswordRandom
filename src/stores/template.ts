import { defineStore } from "pinia"
import { invoke } from "@tauri-apps/api/core"
import type { Template } from "../types"

export const useTemplateStore = defineStore("template", {
  state: () => ({
    templates: [] as Template[],
  }),
  actions: {
    async loadTemplates() {
      this.templates = await invoke("get_templates")
    },
    async saveTemplate(name: string, pattern: string) {
      await invoke("save_template", { name, pattern })
      await this.loadTemplates()
    },
    async deleteTemplate(id: number) {
      await invoke("delete_template", { id })
      this.templates = this.templates.filter((t) => t.id !== id)
    },
  },
})
