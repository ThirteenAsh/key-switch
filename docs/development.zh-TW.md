# Key Switch 二次開發文件

[简体中文](./development.md) · [English](./development.en.md) · 繁體中文

## 1. 開發環境

- Node.js 22 LTS 或更新的相容版本
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

如果只需要除錯前端介面，可以執行：

```bash
npm run dev
```

瀏覽器模式不具備系統密鑰庫、剪貼簿、應用程式資料目錄和持久化設定等 Tauri 能力。瀏覽器模式中的設定只在目前工作階段有效。

## 3. 常用檢查

```bash
# 前端型別檢查與生產建置
npm run build

# Rust 格式、編譯與測試
cd src-tauri
cargo fmt --all -- --check
cargo check --locked
cargo test --locked
```

提交涉及前後端協定、設定檔或安全邏輯的變更前，必須同時執行前端建置與 Rust 測試。

## 4. 架構與目錄職責

| 目錄 | 說明 |
| --- | --- |
| `src/views/` | 儀表板、供應商和設定頁面 |
| `src/components/` | 可重用 Vue 元件、對話框和基礎 UI 元件 |
| `src/api/` | Tauri 命令的 TypeScript 型別與呼叫封裝 |
| `src/stores/` | Pinia 狀態、業務操作和設定持久化佇列 |
| `src/i18n/` | 語言解析、Vue I18n 初始化、錯誤翻譯和四套語言資源 |
| `src/data/` | 內建供應商目錄等非敏感靜態資料 |
| `src-tauri/src/` | Rust 命令、本機檔案、密鑰庫、網路檢測和更新能力 |
| `src-tauri/capabilities/` | Tauri 最小權限宣告 |
| `docs/` | 專案開發文件和 README 資源 |

前端負責顯示、互動和非敏感狀態；敏感資料、持久化檔案、外部請求和系統能力必須放在 Rust 側。新增前端程式碼時，保持 Vue `<script setup lang="ts">` 風格和完整型別定義。

## 5. 本機資料與設定

執行階段資料位於 Tauri 的每位使用者應用程式資料目錄：

| 內容 | 儲存位置 | 說明 |
| --- | --- | --- |
| 供應商與 Key 中繼資料 | `key-switch-data.json` | 不包含完整 API Key |
| 應用程式設定 | `settings.json` | 只保存非敏感偏好 |
| 完整 API Key | 系統密鑰庫 | 不寫入 JSON 檔案 |
| 執行記錄 | `logs/` | 不得記錄完整 Key 或其他憑據 |

目前設定檔範例：

```json
{
  "schemaVersion": 1,
  "localePreference": "system"
}
```

設定啟動流程：

1. `src/main.ts` 在掛載 Vue 前呼叫設定 Store。
2. Rust 讀取並驗證 `settings.json`；首次執行時建立預設檔案。
3. 升級使用者如果仍有舊的 `key-switch.locale`，只在首次建立設定檔時遷移。
4. 成功遷移後刪除舊 `localStorage` 鍵。
5. 設定回傳前端後才掛載介面，避免語言閃切。

設定寫入包含以下保護：

- 前端寫入佇列確保快速連續修改依序提交。
- Rust `SETTINGS_LOCK` 防止同時寫入檔案。
- 新內容先寫入 `settings.json.tmp` 並明確同步至磁碟。
- 原檔暫時改名為 `settings.json.bak`，再由暫存檔替換正式檔案。
- 寫入失敗時恢復上一個檔案，前端同時回復到最近一次成功設定。
- 啟動時若偵測到中斷狀態，會優先恢復已完整寫入的暫存檔，否則恢復備份。

新增設定項目時必須同步修改：

1. Rust `AppSettings` 或對應的巢狀設定結構。
2. Rust `Default` 預設值、欄位範圍驗證和必要的遷移邏輯。
3. TypeScript `AppSettings`。
4. `src/stores/settings.ts` 中的 `normalizeSettings()`。
5. 設定介面、四套翻譯和相關測試。

前端透過 `settingsStore.updateSettings()` 合併局部修改，但每次傳送與寫入的是完整設定物件。只新增帶有預設值的相容欄位時，可以保留目前的 `schemaVersion`；刪除欄位、修改型別或改變含義時，必須提升版本並實作舊版本遷移。不能只修改版本數字，目前實作會拒絕不支援的版本。

## 6. 國際化規範

目前支援：

- `zh-CN`：簡體中文
- `zh-TW`：繁體中文
- `en-US`：英文
- `ja-JP`：日文
- `system`：跟隨系統

無法讀取系統語言時回退到簡體中文；能夠讀取但尚未支援的非中文系統語言使用英文。

開發要求：

- 所有使用者可見文案都必須使用 Vue I18n，包括按鈕、Toast、錯誤、預留位置、`title`、`aria-label` 和圖片 `alt`。
- 簡體中文資源 `src/i18n/messages/zh-CN.ts` 是型別結構基準，其他語言必須符合相同的 `MessageSchema`。
- 新增或刪除翻譯鍵時，必須同步四套語言資源，並執行 `npm run build` 檢查鍵結構。
- 不要把中文原文當作翻譯鍵；使用依領域組織的語意鍵。
- 自訂供應商名稱和已寫入使用者資料的名稱屬於使用者內容，不翻譯、不自動改名。
- 新增內建供應商時，中文介面使用中文目錄名稱；英文、日文及其他非中文介面使用英文名稱，並將當時顯示的名稱寫入資料檔。
- 內建供應商目錄只維護穩定 ID、中文名和英文名，不因目前語言改寫已儲存記錄。
- 動畫元件必須支援 `prefers-reduced-motion`；自訂選擇框等控制項必須保留鍵盤操作和無障礙語意。

## 7. Tauri 命令與錯誤處理

- 前端統一透過 `src/api/app.ts` 呼叫 `invoke`，不要在頁面元件中散落命令字串。
- 新增 Rust 命令使用 `#[tauri::command]`，並在 `src-tauri/src/lib.rs` 的 `tauri::generate_handler!` 中註冊。
- 命令參數和回傳結構必須同步維護 Rust 與 TypeScript 型別。
- Rust 對介面只回傳穩定、非敏感的錯誤碼；底層原始錯誤不得直接顯示給使用者。
- 前端透過 `src/i18n/errors.ts` 將錯誤碼對應到翻譯鍵。
- 業務邏輯不得透過比對中文或英文錯誤文字判斷狀態。
- 新增錯誤碼時，同步更新 Rust 分類、前端錯誤碼對應、四套語言文案和測試。

## 8. 安全約定

- 完整 API Key 只能由 Rust 側讀取和處理，不得進入日誌、URL、DOM 持久狀態、前端 Store、`localStorage` 或 `settings.json`。
- 清單命令只回傳掩碼和非敏感中繼資料；敏感值保存到系統密鑰庫。
- 複製、解密和檢測必須在 Rust 側完成，並盡量縮短明文存活時間。
- 外部檢測只能存取明確允許的供應商端點，並設定逾時、回應內容大小上限和錯誤分類。
- 設定檔只保存非敏感應用程式偏好；未來新增設定欄位前必須進行敏感性審查。
- 新增 Tauri 外掛或系統能力前，檢查 `src-tauri/capabilities/` 並維持最小權限。
- 不得在原始碼、測試資料、Issue、PR、日誌或截圖中提交真實 API Key。

### 自訂供應商 Key 檢查

- 自訂供應商預設使用「不檢查」，舊資料缺少檢查設定時也必須回退到「不檢查」與「不支援檢查」狀態。
- 平台管理網址 `platformUrl` 僅用於開啟管理後台，不得作為檢查端點。檢查設定獨立儲存在供應商中繼資料，目前支援 OpenAI 相容、Bearer Token 與 API Key Header 三種方式。
- 檢查設定只保存 HTTPS 端點、驗證方式與非敏感 Header 名稱；完整 Key 仍僅由 Rust 側從系統密鑰庫暫時讀取。
- 自訂檢查網址不得包含憑據、查詢參數或片段，預設拒絕回環、區域網路、鏈路本地、保留位址以及解析到這些位址的網域，並繼續禁止 HTTP 重新導向。
- 檢查請求不讀取或記錄回應本文。限流、逾時、網路故障、伺服器故障與端點錯誤必須使用穩定且非敏感的結果碼區分，不得誤判為 Key 無效。
- 修改供應商檢查設定後，必須清除該供應商既有 Key 的檢查時間與舊狀態；沒有檢查能力時不得發起網路請求。

## 9. 變更檢查清單

- 介面變更：檢查四種語言、長文字、鍵盤操作和減少動畫設定。
- 設定變更：檢查預設值、驗證、完整物件合併、失敗回復和舊檔相容性。
- Rust 命令變更：檢查前後端型別、命令註冊和結構化錯誤碼。
- 資料模型變更：不得無提示改寫使用者既有名稱、備註或其他業務資料。
- 安全變更：確認敏感資訊不會寫入前端持久化、設定檔或日誌。
- 文件變更：同步 `development.md`、`development.en.md` 和 `development.zh-TW.md`。
- 完成後至少執行 `npm run build`、`cargo fmt --all -- --check`、`cargo check --locked` 和 `cargo test --locked`。
