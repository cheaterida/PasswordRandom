export interface GenConfig {
  length: number
  upper: boolean
  lower: boolean
  digits: boolean
  symbols: boolean
  exclude_ambiguous: boolean
}

export interface PinConfig {
  length: number
  no_consecutive: boolean
  no_sequential: boolean
}

export interface PassphraseConfig {
  word_count: number
  separator: string
  capitalize: boolean
}

export interface Category {
  id: number
  name: string
  icon: string
  created_at: string
}

export interface PasswordDisplay {
  id: number
  category_id: number | null
  category_name: string | null
  label: string
  username: string
  password: string
  mode: string
  template_id: number | null
  created_at: string
}

export interface Template {
  id: number
  name: string
  pattern: string
  created_at: string
}

export type PasswordMode = "random" | "pin" | "passphrase" | "template"

export const MODE_LABELS: Record<PasswordMode, string> = {
  random: "随机",
  pin: "PIN",
  passphrase: "密语",
  template: "模板",
}

export const MODE_ICONS: Record<PasswordMode, string> = {
  random: "🔀",
  pin: "🔢",
  passphrase: "📝",
  template: "⚙️",
}
