# Key Switch Development Guide

[简体中文](./development.md) · **English** · [繁體中文](./development.zh-TW.md)

## 1. Prerequisites

- Node.js 20 LTS or later
- npm
- The Rust stable toolchain
- Platform dependencies required by Tauri 2

Windows also requires Microsoft C++ Build Tools and WebView2 Runtime.

## 2. Run the project

From the project root:

```bash
npm install
npm run tauri:dev
```

To work on the frontend only:

```bash
npm run dev
```

Browser mode does not provide Tauri capabilities such as the system keychain, clipboard, or the local data directory.

## 3. Common checks

```bash
# Type-check and build the frontend
npm run build

# Format, compile, and test Rust
cd src-tauri
cargo fmt --all -- --check
cargo check --locked
cargo test --locked
```

## 4. Directory responsibilities

| Directory | Responsibility |
| --- | --- |
| `src/views/` | Dashboard, providers, and settings pages |
| `src/components/` | Reusable Vue components and dialogs |
| `src/api/` | Frontend wrappers for Tauri commands |
| `src/stores/` | Pinia state and business operations |
| `src-tauri/src/` | Rust commands, local data, and system capabilities |
| `src-tauri/capabilities/` | Tauri permission declarations |
| `docs/` | Project documentation and README assets |

Keep the Vue `<script setup lang="ts">` style when adding frontend features. For native capabilities, define a Rust `#[tauri::command]` and register it in `src-tauri/src/lib.rs`.

## 5. Security rules

- Full API Keys must be read and handled on the Rust side; do not put them in logs, URLs, persistent DOM state, or frontend storage.
- List APIs should return masked keys only; sensitive values are stored in the system keychain.
- External checks must only access user-configured endpoints and should use timeouts and clear error categories.
- Never commit real API Keys in source code, test data, Issues, pull requests, or screenshots.
