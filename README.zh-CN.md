# 🔐 PasswordRandom

> 私人密码随机生成器 · 安全 · 离线 · 跨平台

[![MIT License](https://img.shields.io/badge/license-MIT-green)](LICENSE)
[![Release](https://img.shields.io/badge/release-v0.1.1-blue)](https://github.com/cheaterida/PasswordRandom/releases)
[![Tauri](https://img.shields.io/badge/Tauri-2.x-orange)](https://tauri.app)
[![Vue](https://img.shields.io/badge/Vue-3.5-brightgreen)](https://vuejs.org)
[![Rust](https://img.shields.io/badge/Rust-1.97-red)](https://rust-lang.org)

[English](README.md)

---

## ✨ 功能

- **4 种密码生成模式** — 随机密码 / PIN 数字 / 密语短语 / 模板定制
- **主密码保护** — Argon2id 密钥派生，无主密码无法解密数据
- **AES-256-GCM 加密** — 所有存储密码经军事级加密
- **生物识别解锁** (Android) — 指纹 + 人脸，快速安全
- **密码历史管理** — 分类标注、搜索、查看/隐藏/复制
- **手动添加** — 支持录入自有密码统一管理
- **完全离线** — 数据仅存储在本地，不上传任何服务器
- **跨平台** — Windows 桌面 + Android 移动端

## 📥 下载

前往 [Releases](https://github.com/cheaterida/PasswordRandom/releases) 下载最新版本：

| 平台 | 文件 |
|---|---|
| Windows 64位 | `PasswordRandom-v0.1.1-windows-x64.exe` |
| Android | `PasswordRandom-v0.1.1-android.apk` |

> Windows 版直接运行，无需安装任何运行时依赖。

## 📖 使用说明

### 首次使用
1. 启动后设置一个**主密码**（至少 6 位）
2. 选择是否启用生物解锁（仅 Android）
3. 进入主界面

### 生成密码
- **随机** — 选择长度和字符集（大小写/数字/符号），可排除易混淆字符
- **PIN** — 纯数字密码，支持禁止连续/序列数字
- **密语** — 基于单词组合，易记且安全，如 `correct-horse-battery-staple`
- **模板** — 自定义格式，如 `[word]-[digit:4]` → `dawn-4721`

### 管理密码
- 生成的密码可**保存到加密历史**，标注用途和分类
- 支持**手动添加**自有密码统一管理
- 按分类筛选、关键词搜索
- 5 分钟无操作自动锁定

## 🏗 技术架构

```
┌──────────────────────────────────────────┐
│              前端 (Vue 3 + TS)            │
│    MasterLock  │  Generator  │  History   │
│    (解锁)       │  (4种模式)   │  (加密库)  │
├──────────────────────────────────────────┤
│             Tauri v2 (IPC 桥)             │
├──────────────────────────────────────────┤
│              后端 (Rust)                  │
│  crypto    │   db     │  generator       │
│  Argon2id  │  SQLite  │  rand + 模板引擎  │
│  AES-GCM   │  加密CRUD │  内置词库 500+  │
├──────────────────────────────────────────┤
│            原生接口                       │
│   Biometric (生物)  │  Clipboard (剪贴板) │
└──────────────────────────────────────────┘
```

### 数据安全

```
主密码 ─→ Argon2id ─→ 派生密钥
                         │
             ┌───────────┴───────────┐
             ▼                       ▼
        验证登录              AES-256-GCM 加解密
                            所有历史密码密文入库
```

- 主密码**永不存储**，仅存 Argon2id 验证哈希
- 加密密钥仅解密后驻留内存，锁定/退出即销毁
- 生物识别通过系统级 BiometricPrompt 验证

## 📂 项目结构

```
├── src/                        # Vue 3 前端
│   ├── components/
│   │   ├── MasterLock.vue      # 主密码解锁 + 生物识别
│   │   ├── GeneratorPanel.vue  # 密码生成面板
│   │   ├── PasswordHistory.vue # 历史记录
│   │   ├── TemplateEditor.vue  # 模板编辑器
│   │   ├── CategoryManager.vue # 分类管理
│   │   └── Settings.vue        # 设置页
│   ├── stores/                 # Pinia 状态管理
│   └── types/                  # TypeScript 类型
├── src-tauri/                  # Rust 后端
│   └── src/
│       ├── main.rs / lib.rs    # 入口 & 配置
│       ├── commands.rs         # Tauri IPC 命令
│       ├── crypto.rs           # 加密模块
│       ├── db.rs               # SQLite 数据库
│       ├── generator.rs        # 密码生成引擎
│       └── biometric.rs        # 生物识别密钥存储
└── src-tauri/gen/android/      # Android 工程
```

## 🔧 从源码构建

### 环境需求

- **Rust** 1.70+ （`rustup`）
- **Node.js** 18+（`node`）
- **VS Build Tools**（Windows MSVC 工具链）
- **Android** — JDK 17 + Android SDK + NDK 27

### 构建步骤

```bash
# 克隆仓库
git clone git@github.com:cheaterida/PasswordRandom.git
cd PasswordRandom

# 安装前端依赖
npm install

# Windows 桌面构建
npx tauri build

# Android APK 构建
# 需先配置 ANDROID_HOME / NDK_HOME / JAVA_HOME
npx tauri android build --apk --target aarch64
```

## 📄 许可证

[MIT](LICENSE) © 2026 cheater

---

<p align="center">愿你的每一个密码都安全无忧 🙏</p>
