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

# 前端導覽、更新互動與日誌測試
npm run test:frontend

# Rust 格式、編譯與測試
cd src-tauri
cargo fmt --all -- --check
cargo check --locked
cargo test --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
```

提交涉及前後端協定、設定檔或安全邏輯的變更前，必須同時執行前端建置與 Rust 測試。

### 版本與發布

目前應用程式版本為 `1.1.1`。更新版本時同步 `package.json`、`package-lock.json` 根套件版本、`src-tauri/Cargo.toml`、`src-tauri/Cargo.lock` 的應用套件版本、`src-tauri/tauri.conf.json` 及 Windows WiX 版本，並更新四份 README 版本徽章與發布流程預設標籤。

介面版本回退值讀取 `package.json`，Rust 請求標頭使用 `CARGO_PKG_VERSION`；不再個別維護版本字串。實際桌面版本仍以 Tauri 回傳的資訊為準。

CI 與發布建置使用 Node.js 22，執行 `npm run test:frontend`。發布標籤必須與應用版本一致；`.github/release-notes.md` 儲存目前版本的發布說明，發布流程只建立草稿。程式碼提交、推送、建立標籤與發布應依明確的發布指令執行。

## 4. 架構與目錄職責

| 目錄 | 說明 |
| --- | --- |
| `src/views/` | 主頁面、供應商管理與設定分類頁面 |
| `src/components/` | 可重用 Vue 元件、對話框和基礎 UI 元件 |
| `src/api/` | Tauri 命令的 TypeScript 型別與呼叫封裝 |
| `src/stores/` | Pinia 狀態、業務操作和設定持久化佇列 |
| `src/i18n/` | 語言解析、Vue I18n 初始化、錯誤翻譯和四套語言資源 |
| `src/data/` | 內建供應商目錄等非敏感靜態資料 |
| `src-tauri/src/` | Rust 命令、本機檔案、密鑰庫、網路檢測和更新能力 |
| `src-tauri/capabilities/` | Tauri 最小權限宣告 |
| `docs/` | 專案開發文件和 README 資源 |

前端負責顯示、互動和非敏感狀態；敏感資料、持久化檔案、外部請求和系統能力必須放在 Rust 側。新增前端程式碼時，保持 Vue `<script setup lang="ts">` 風格和完整型別定義。

### 頁面導覽與排序

- 側欄透過隱藏的內建中英文名稱寬度基準與 CSS 行內尺寸包含自動定寬，不依自訂長名稱擴展。兩側標題共用高度；供應商類型與平台網址置於右側標題後，長網址截斷但保留完整提示與開啟入口。
- 切換供應商時，標題與 Key 內容使用短淡入淡出過渡；內容附帶輕微位移，減少動畫模式停用過渡與位移。退出中的內容暫停指標操作。操作欄保留四個按鈕所需固定寬度，遮罩儲存格維持表格佈局。
- 主頁面採用左側供應商、右側目前供應商 Key 的佈局；供應商管理與設定入口位於左下角。
- 供應商選取狀態僅保留在 Pinia 目前工作階段，不新增資料檔案或設定欄位。左側不提供供應商搜尋，始終顯示完整清單；首次載入預設選取第一個供應商，已選供應商遭刪除時回退到首項。右側 Key 搜尋關鍵字僅保留在頁面元件中。
- 左側供應商排序沿用指標捕獲、4px 啟動門檻與 FLIP 落位動畫，並支援在拖曳手柄上使用 `Alt + ↑ / ↓` 調整順序。減少動畫模式略過落位動畫。
- 排序仍透過既有 `reorder_providers` 命令儲存完整 ID 順序；儲存期間禁止重複排序，失敗時恢復原順序並顯示翻譯後的錯誤。
- `/settings` 預設進入 `/settings/general`。設定依現有功能分為一般（語言、外觀）、資料與日誌（儲存目錄、日誌）、關於（版本、儲存庫、更新），分別對應 `/settings/general`、`/settings/data`、`/settings/about`；返回按鈕回到主頁面。
- 版本與更新安裝狀態顯示於側欄底部；設定儲存仍使用既有 Store 佇列，設定與業務資料檔結構保持不變；更新命令新增僅本次生效的連線模式與操作 ID。
- 設定分類頁沿用主頁面的標題高度、文字與圖示尺寸，控制項維持緊湊。側欄底部導覽在剩餘寬度不足時自動換行，完整保留各語言的標籤。

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
  "localePreference": "system",
  "themePreference": "system"
}
```

`themePreference` 支援 `system`、`light` 和 `dark`。主題在設定頁面選擇；`system` 會即時跟隨作業系統的淺色或深色外觀。主題會同時套用到 WebView 內容與 Tauri 原生視窗標題列，變更透過設定 Store 的寫入佇列持久化。

設定啟動流程：

1. `src/main.ts` 在掛載 Vue 前呼叫設定 Store。
2. Rust 讀取並驗證 `settings.json`；首次執行時建立預設檔案。
3. 升級使用者如果仍有舊的 `key-switch.locale`，只在首次建立設定檔時遷移。
4. 成功遷移後刪除舊 `localStorage` 鍵。
5. 設定回傳前端後先套用語言和主題，再掛載介面，避免啟動時閃切。

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

日誌行為：Rust 事件寫入 `logs/key-switch.log`，每個檔案達到 1 MB 時輪轉，保留 3 個備份；清除日誌會移除全部備份。新記錄使用含時區的 RFC 3339 時間，附帶工作階段 ID、處理程序 ID；啟動包含版本、系統與架構。Rust 命令原始錯誤經遮蔽後記錄，介面仍只接收穩定錯誤碼。更新包含操作 ID、階段、代理來源、耗時、位元組數與簽章結果；Key 檢查包含非敏感 ID、HTTP 狀態與失敗分類，也記錄 DNS 解析提前返回的失敗。不得記錄完整 Key、憑據、URL 路徑/查詢參數或回應本文。

前端繼續捕捉 Vue 錯誤、未處理的 Promise 拒絕和資源載入失敗，資源錯誤使用捕獲階段監聽。Rust 入口統一遮蔽與截斷；寫入失敗輸出已遮蔽的標準錯誤備用記錄，前端只輸出一次不含詳情的警告。Rust panic 只記錄位置，不記錄可能含使用者資料的 payload；強制終止處理程序不保證有最後一筆記錄。

### 更新連線與直接連線確認

- 版本檢查、清單及安裝包預設讀取代理環境變數與支援的系統代理，遵循 `NO_PROXY`；環境變數優先。Windows 支援手動代理，PAC 自動設定不在此實作範圍內。
- 代理網路、連線、逾時或 HTTP 請求失敗時，手動操作詢問是否直接連線；啟動自動檢查失敗僅記錄日誌。更新資料或簽章錯誤不詢問直接連線。
- 同意後透過 `.no_proxy()` 重試一次；模式與操作 ID 從檢查延續到本次安裝，取消後不再請求，下次檢查重新使用預設代理。沒有代理時直接請求。不新增設定或永久偏好；應用層直接連線不繞過 TUN 路由。
- 檢查上限 12 秒、清單 15 秒、下載 300 秒、連線 5 秒；安裝包限制 256 MB。先結束前次請求再重試，直接連線仍失敗沿用現有提示。
- 確認彈窗沿用主題變數及短過渡，支援四種語言、Esc/遮罩取消、Tab 焦點限制、焦點復原與減少動畫。

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
- 修改供應商檢查設定後，必須清除該供應商既有 Key 的舊狀態；沒有檢查能力時不得發起網路請求。

## 9. 變更檢查清單

- 介面變更：檢查四種語言、長文字、鍵盤操作和減少動畫設定。
- 設定變更：檢查預設值、驗證、完整物件合併、失敗回復和舊檔相容性。
- Rust 命令變更：檢查前後端型別、命令註冊和結構化錯誤碼。
- 資料模型變更：不得無提示改寫使用者既有名稱、備註或其他業務資料。
- 安全變更：確認敏感資訊不會寫入前端持久化、設定檔或日誌。
- 文件變更：同步 `development.md`、`development.en.md` 和 `development.zh-TW.md`。
- 完成後執行 `npm run build`、`npm run test:frontend`、`cargo fmt --all -- --check`、`cargo check --locked`、`cargo test --locked` 及 CI 使用的 `cargo clippy --all-targets --all-features --locked -- -D warnings`。
