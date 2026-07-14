# 🔐 PasswordRandom

> Private password generator & manager · Secure · Offline · Cross-platform

[![MIT License](https://img.shields.io/badge/license-MIT-green)](LICENSE)
[![Release](https://img.shields.io/badge/release-vx.x.x-blue)](https://github.com/cheaterida/PasswordRandom/releases)
[![Tauri](https://img.shields.io/badge/Tauri-2.x-orange)](https://tauri.app)
[![Vue](https://img.shields.io/badge/Vue-3.5-brightgreen)](https://vuejs.org)
[![Rust](https://img.shields.io/badge/Rust-1.97-red)](https://rust-lang.org)

[中文文档](README.zh-CN.md)

---

## ✨ Features

- **4 generation modes** — Random / PIN / Passphrase / Custom Template
- **Master password** — Argon2id key derivation; data locked without it
- **AES-256-GCM encryption** — Military-grade encryption for all stored passwords
- **Biometric unlock** (Android) — Fingerprint & face recognition
- **Password history** — Categories, search, show/hide, copy
- **Manual entry** — Add your own existing passwords
- **Fully offline** — Data stored locally, never uploaded
- **Cross-platform** — Windows desktop + Android

## 📥 Download

Get the latest from [Releases](https://github.com/cheaterida/PasswordRandom/releases):

| Platform | File |
|---|---|
| Windows 64-bit | `PasswordRandom-vx.x.x-windows-x64.exe` |
| Android | `PasswordRandom-vx.x.x-android.apk` |

> Windows: run the .exe directly — no runtime dependencies required.

## 📖 Usage

### First time
1. Set a **master password** (6+ characters)
2. Optionally enable biometric unlock (Android)
3. Enter the main interface

### Generator modes
- **Random** — length + character sets (upper/lower/digits/symbols), exclude ambiguous chars
- **PIN** — numeric-only, optional consecutive/sequential restrictions
- **Passphrase** — word-based, easy to remember, e.g. `correct-horse-battery-staple`
- **Template** — custom format, e.g. `[word]-[digit:4]` → `dawn-4721`

### Password management
- Save generated passwords to encrypted history with labels & categories
- Manually add your own passwords
- Filter by category, search by keyword
- Auto-lock after 5 minutes of inactivity

## 🏗 Architecture

```
┌──────────────────────────────────────────┐
│            Frontend (Vue 3 + TS)         │
│   MasterLock  │  Generator  │  History   │
│    (unlock)   │  (4 modes)  │  (vault)   │
├──────────────────────────────────────────┤
│            Tauri v2 (IPC bridge)         │
├──────────────────────────────────────────┤
│              Backend (Rust)              │
│  crypto    │   db     │  generator       │
│  Argon2id  │  SQLite  │  rand + template │
│  AES-GCM   │  CRUD    │  500+ wordlist   │
├──────────────────────────────────────────┤
│             Native interfaces            │
│          Biometric  │  Clipboard         │
└──────────────────────────────────────────┘
```

### Security

```
Master password ─→ Argon2id ─→ Derived key
                                   │
                    ┌──────────────┴──────────────┐
                    ▼                             ▼
              Login verification        AES-256-GCM encrypt/decrypt
                                        all history stored as ciphertext
```

- Master password **never stored** — only Argon2id verification hash
- Encryption key exists in memory only when unlocked — destroyed on lock/exit
- Biometric auth verified through system-level BiometricPrompt (Android)

## 📂 Project structure

```
├── src/                        # Vue 3 frontend
│   ├── components/
│   │   ├── MasterLock.vue      # Unlock + biometric
│   │   ├── GeneratorPanel.vue  # Password generator
│   │   ├── PasswordHistory.vue # History & vault
│   │   ├── TemplateEditor.vue  # Template manager
│   │   ├── CategoryManager.vue # Category manager
│   │   └── Settings.vue        # App settings
│   ├── stores/                 # Pinia state
│   └── types/                  # TypeScript types
├── src-tauri/                  # Rust backend
│   └── src/
│       ├── main.rs / lib.rs    # Entry & config
│       ├── commands.rs         # Tauri IPC commands
│       ├── crypto.rs           # Encryption module
│       ├── db.rs               # SQLite database
│       ├── generator.rs        # Password engine
│       └── biometric.rs        # Biometric key storage
└── src-tauri/gen/android/      # Android project
```

## 🔧 Building from source

### Prerequisites

- **Rust** 1.70+ (`rustup`)
- **Node.js** 18+ (`node`)
- **VS Build Tools** (Windows MSVC)
- **Android** — JDK 17 + Android SDK + NDK 27

### Build steps

```bash
# Clone
git clone git@github.com:cheaterida/PasswordRandom.git
cd PasswordRandom

# Install frontend deps
npm install

# Windows
npx tauri build

# Android (requires ANDROID_HOME / NDK_HOME / JAVA_HOME)
npx tauri android build --apk --target aarch64
```

## 📄 License

[MIT](LICENSE) © 2026 cheater

---

<p align="center">May every password of yours be safe and secure 🙏</p>
