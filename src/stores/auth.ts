import { defineStore } from "pinia"
import { invoke } from "@tauri-apps/api/core"

export const useAuthStore = defineStore("auth", {
  state: () => ({
    initialized: false,
    locked: true,
  }),
  actions: {
    async checkStatus() {
      this.initialized = await invoke("is_initialized")
    },
    setLocked(val: boolean) {
      this.locked = val
    },
  },
})
