# Key Switch 二次開發文件

[简体中文](./development.md) · [English](./development.en.md) · **繁體中文**

## 1. 開發環境

- Node.js 20 LTS 或更新版本
- npm
- Rust stable 工具鏈
- Tauri 2 對應的平台依賴

Windows 另外需要 Microsoft C++ Build Tools 和 WebView2 Runtime。

## 2. 啟動專案

在專案根目錄執行：

```bash
npm install
npm run tauri:dev
```

如果只需要調試前端介面，可以執行：

```bash
npm run dev
```

瀏覽器模式不具備系統密鑰庫、剪貼簿和本地資料目錄等 Tauri 能力。

## 3. 常用檢查

```bash
# 前端類型檢查與建置
npm run build

# Rust 格式、編譯與測試
cd src-tauri
cargo fmt --all -- --check
cargo check --locked
cargo test --locked
```

## 4. 目錄職責

| 目錄 | 說明 |
| --- | --- |
| `src/views/` | 儀表板、服務商和設定頁面 |
| `src/components/` | 可重用 Vue 元件和對話框 |
| `src/api/` | 前端呼叫 Tauri 命令的封裝 |
| `src/stores/` | Pinia 狀態與業務操作 |
| `src-tauri/src/` | Rust 命令、本地資料和系統能力 |
| `src-tauri/capabilities/` | Tauri 權限宣告 |
| `docs/` | 專案文件和 README 資源 |

新增前端功能時，請保持 Vue `<script setup lang="ts">` 風格；新增原生能力時，在 Rust 中使用 `#[tauri::command]`，並於 `src-tauri/src/lib.rs` 註冊命令。

## 5. 安全約定

- 完整 API Key 只能由 Rust 側讀取和處理，不要放入日誌、URL、DOM 持久狀態或前端持久化儲存。
- 列表介面只返回掩碼後的 Key；敏感資料使用系統密鑰庫保存。
- 外部檢測只能存取使用者設定的位址，並設定逾時和明確的錯誤分類。
- 不要在原始碼、測試資料、Issue、PR 或截圖中提交真實 API Key。
