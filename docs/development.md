# Key Switch 二次开发文档

简体中文 · [English](./development.en.md) · [繁體中文](./development.zh-TW.md)

## 1. 开发环境

- Node.js 22 LTS 或更高兼容版本
- npm
- Rust stable 工具链
- Tauri 2 对应的平台依赖

Windows 还需要 Microsoft C++ Build Tools 和 WebView2 Runtime。

## 2. 启动项目

在项目根目录执行：

```bash
npm install
npm run tauri:dev
```

如果只调试前端界面，可以执行：

```bash
npm run dev
```

浏览器模式不具备系统密钥库、剪贴板、应用数据目录和持久化设置等 Tauri 能力。浏览器模式中的设置只对当前会话生效。

## 3. 常用检查

```bash
# 前端类型检查与生产构建
npm run build

# 前端导航、更新交互与日志测试
npm run test:frontend

# Rust 格式、编译和测试
cd src-tauri
cargo fmt --all -- --check
cargo check --locked
cargo test --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
```

提交涉及前后端协议、设置文件或安全逻辑的改动前，必须同时执行前端构建和 Rust 测试。

### 版本与发布

当前应用版本为 `1.1.1`。更新版本时同步 `package.json`、`package-lock.json` 根包版本、`src-tauri/Cargo.toml`、`src-tauri/Cargo.lock` 的应用包版本、`src-tauri/tauri.conf.json` 及 Windows WiX 版本，并更新四份 README 版本徽章与发布流程默认标签。

界面版本回退值读取 `package.json`，Rust 请求头使用 `CARGO_PKG_VERSION`；不再单独维护版本字符串。实际桌面版本仍以 Tauri 返回的信息为准。

CI 和发布构建使用 Node.js 22，运行 `npm run test:frontend`。发布标签必须与应用版本一致；`.github/release-notes.md` 保存当前版本的发布说明，发布流程只创建草稿。代码提交、推送、打标签和发布应按明确的发布指令执行。

## 4. 架构与目录职责

| 目录 | 说明 |
| --- | --- |
| `src/views/` | 主页面、供应商管理与设置分类页面 |
| `src/components/` | 可复用 Vue 组件、弹窗和基础 UI 组件 |
| `src/api/` | Tauri 命令的 TypeScript 类型与调用封装 |
| `src/stores/` | Pinia 状态、业务操作和设置持久化队列 |
| `src/i18n/` | 语言解析、Vue I18n 初始化、错误翻译和四套语言资源 |
| `src/data/` | 内置供应商目录等非敏感静态数据 |
| `src-tauri/src/` | Rust 命令、本地文件、密钥库、网络检测和更新能力 |
| `src-tauri/capabilities/` | Tauri 最小权限声明 |
| `docs/` | 项目开发文档和 README 资源 |

前端负责展示、交互和非敏感状态；敏感数据、持久化文件、外部请求和系统能力必须放在 Rust 侧。新增前端代码保持 Vue `<script setup lang="ts">` 风格和完整类型定义。

### 页面导航与排序

- 侧栏通过隐藏的内置中英文名称宽度基准和 CSS 内联尺寸包含自动定宽，不根据自定义长名称扩展。两侧标题共享高度；供应商类型与平台地址放在右侧标题后，长地址截断但保留完整提示和打开入口。
- 切换供应商时，标题与 Key 内容使用短淡入淡出过渡；内容附带轻微位移，减少动画模式禁用过渡与位移。退出中的内容暂停指针操作。操作列保留四个按钮所需固定宽度，掩码单元格保持表格布局。
- 主页面采用左侧供应商、右侧当前供应商 Key 的布局；供应商管理与设置入口位于左下角。
- 供应商选中状态仅保留在 Pinia 当前会话中，不新增数据文件或设置字段。左侧不提供供应商搜索，始终显示完整列表；首次加载默认选中首个供应商，已选供应商删除时回退到首项。右侧 Key 搜索关键词仅保留在页面组件中。
- 左侧供应商排序复用指针捕获、4px 激活阈值和 FLIP 落位动画，并支持在拖动手柄上使用 `Alt + ↑ / ↓` 调整顺序。减少动画模式跳过落位动画。
- 排序仍通过现有 `reorder_providers` 命令保存完整 ID 顺序；保存期间禁止重复排序，失败时恢复原顺序并显示翻译后的错误。
- `/settings` 默认进入 `/settings/general`。设置按现有功能划分为通用（语言、外观）、数据与日志（存储目录、日志）、关于（版本、仓库、更新），分别对应 `/settings/general`、`/settings/data`、`/settings/about`；返回按钮回到主页面。
- 版本与更新安装状态显示在侧栏底部；设置保存仍使用原有 Store 队列，设置与业务数据文件结构保持不变；更新命令增加仅本次生效的连接模式和操作 ID。
- 设置分类页沿用主页面的标题高度、文字与图标尺寸，控件保持紧凑。侧栏底部导航在剩余宽度不足时自动换行，完整保留各语言的标签。

## 5. 本地数据与设置

运行时数据位于 Tauri 的用户应用数据目录：

| 内容 | 存储位置 | 说明 |
| --- | --- | --- |
| 供应商与 Key 元数据 | `key-switch-data.json` | 不包含完整 API Key |
| 应用设置 | `settings.json` | 只保存非敏感偏好 |
| 完整 API Key | 系统密钥库 | 不写入 JSON 文件 |
| 运行日志 | `logs/` | 不得记录完整 Key 或其他凭据 |

当前设置文件示例：

```json
{
  "schemaVersion": 1,
  "localePreference": "system",
  "themePreference": "system"
}
```

`themePreference` 支持 `system`、`light` 和 `dark`。主题在设置页面选择；`system` 会实时跟随操作系统的浅色或深色外观。主题同时应用到 WebView 内容和 Tauri 原生窗口标题栏，变更通过设置 Store 的写入队列持久化。

设置启动流程：

1. `src/main.ts` 在挂载 Vue 前调用设置 Store。
2. Rust 读取并校验 `settings.json`；首次运行时创建默认文件。
3. 升级用户如果仍有旧的 `key-switch.locale`，只在首次创建设置文件时迁移。
4. 成功迁移后删除旧 `localStorage` 键。
5. 设置返回前端后先应用语言和主题，再挂载界面，避免启动时闪切。

设置保存使用以下保护：

- 前端写入队列保证快速连续修改按顺序提交。
- Rust `SETTINGS_LOCK` 防止并发写文件。
- 新内容先写入 `settings.json.tmp` 并调用落盘同步。
- 原文件临时改名为 `settings.json.bak`，再由临时文件替换正式文件。
- 写入失败时恢复上一次文件，前端同时回滚到最近一次成功设置。
- 启动时如果检测到中断状态，会优先恢复已完整写入的临时文件，否则恢复备份。

新增设置项时必须同步修改：

1. Rust `AppSettings` 或对应的嵌套设置结构。
2. Rust `Default` 默认值、字段范围校验和必要的迁移逻辑。
3. TypeScript `AppSettings`。
4. `src/stores/settings.ts` 中的 `normalizeSettings()`。
5. 设置界面、四套翻译和相关测试。

前端通过 `settingsStore.updateSettings()` 合并局部修改，但每次发送和写入的是完整设置对象。仅新增带默认值的兼容字段时可以保留当前 `schemaVersion`；删除字段、修改类型或改变含义时必须提升版本并实现旧版本迁移。不能只修改版本数字，当前实现会拒绝不受支持的版本。

日志行为：Rust 业务事件写入 `logs/key-switch.log`，每个文件达到 1 MB 时轮转，保留 3 个备份；清空日志同时移除全部备份。新记录使用带时区的 RFC 3339 时间，附带会话 ID、进程 ID；启动记录包含版本、系统和架构。命令原始错误在 Rust 侧经脱敏后记录，界面仍只接收稳定错误码。更新记录包含操作 ID、阶段、代理来源、耗时、下载字节数和签名结果；Key 检测记录非敏感 ID、HTTP 状态和失败原因，并覆盖 DNS 解析提前返回路径。记录不得包含完整 Key、凭据、URL 路径/查询参数或响应正文。

前端继续捕获 Vue 异常、未处理的 Promise 拒绝和资源加载失败，资源错误使用捕获阶段监听。Rust 日志入口统一脱敏和截断；写入失败输出脱敏备用记录到标准错误，前端仅输出一次不含详情的写入失败提示。Rust panic 只记录发生位置，不记录可能含用户数据的 payload；强制终止进程等情况不保证有最后一条记录。

### 更新连接与直连确认

- 版本检测、清单和安装包默认读取代理环境变量及受支持的系统代理配置，遵循 `NO_PROXY`；环境变量优先。Windows 目前支持手动代理配置，PAC 自动配置不在此实现范围内。
- 通过代理发生网络、连接、超时或 HTTP 请求失败时，手动操作弹出直连确认；启动自动检测失败只记录日志。更新数据或签名校验失败不触发直连询问。
- 确认后使用 `.no_proxy()` 重试一次；该模式和操作 ID 从检测延续到本次安装，取消不再请求，下一次检测重新使用默认代理策略。没有代理时直接请求。不新增设置字段或永久连接偏好；应用层直连不绕过 TUN 等网络层接管。
- 检测请求上限 12 秒，清单上限 15 秒，下载上限 300 秒，连接上限 5 秒；安装包限制为 256 MB。重试先结束前一次请求；直连仍失败沿用现有错误提示。
- 确认弹窗使用主题变量和短过渡，支持四种语言、Esc/遮罩取消、Tab 焦点限制、焦点恢复和减少动画。

## 6. 国际化规范

当前支持：

- `zh-CN`：简体中文
- `zh-TW`：繁体中文
- `en-US`：英语
- `ja-JP`：日语
- `system`：跟随系统

无法读取系统语言时回退到简体中文；能够读取但暂不支持的非中文系统语言使用英语。

开发要求：

- 所有用户可见文案必须使用 Vue I18n，包括按钮、Toast、错误、占位符、`title`、`aria-label` 和图片 `alt`。
- 简体中文资源 `src/i18n/messages/zh-CN.ts` 是类型结构基准，其他语言必须满足同一 `MessageSchema`。
- 新增或删除翻译键时必须同步四套语言资源，并执行 `npm run build` 检查键结构。
- 不要把中文原文作为翻译键；使用按领域组织的语义键。
- 自定义供应商名称和已有数据文件中的名称属于用户数据，不翻译、不自动改名。
- 新增内置供应商时，中文界面使用中文目录名称；英语、日语及其他非中文界面使用英文名称，并将当时显示的名称写入数据文件。
- 内置供应商目录只维护稳定 ID、中文名和英文名，不根据当前语言重写已存储记录。
- 动画组件必须支持 `prefers-reduced-motion`；自定义选择框等控件必须保留键盘和无障碍语义。

## 7. Tauri 命令与错误处理

- 前端统一通过 `src/api/app.ts` 调用 `invoke`，不要在页面中直接散落命令字符串。
- 新增 Rust 命令使用 `#[tauri::command]`，并在 `src-tauri/src/lib.rs` 的 `tauri::generate_handler!` 中注册。
- 命令参数和返回结构必须同时维护 Rust 与 TypeScript 类型。
- Rust 对界面只返回稳定、非敏感的错误码；底层错误不得直接显示给用户。
- 前端通过 `src/i18n/errors.ts` 将错误码映射为翻译键。
- 业务逻辑不得通过匹配中文或英文错误文本判断状态。
- 新增错误码时同步更新 Rust 分类、前端错误码映射、四套语言文案和测试。

## 8. 安全约定

- 完整 API Key 只能由 Rust 侧读取和处理，不得进入日志、URL、DOM 持久状态、前端 Store、`localStorage` 或 `settings.json`。
- 列表接口只返回掩码和非敏感元数据；敏感值保存到系统密钥库。
- 复制、解密和检测必须在 Rust 侧完成，并尽量缩短明文生命周期。
- 外部检测只访问明确允许的供应商端点，并设置超时、响应体上限和错误分类。
- 设置文件只保存非敏感应用偏好；未来新增设置字段前必须进行敏感性审查。
- 新增 Tauri 插件或系统能力前，检查 `src-tauri/capabilities/` 并坚持最小权限。
- 不得在源码、测试数据、Issue、PR、日志或截图中提交真实 API Key。

### 自定义供应商 Key 检测

- 自定义供应商默认使用“不检测”，旧数据缺少检测配置时也必须回退到“不检测”和“不支持检测”状态。
- 平台管理地址 `platformUrl` 仅用于打开管理后台，不能作为检测端点。检测配置独立保存在供应商元数据中，目前支持 OpenAI 兼容、Bearer Token 和 API Key Header 三种方式。
- 检测配置只保存 HTTPS 端点、认证方式和非敏感 Header 名称；完整 Key 仍只在 Rust 侧从系统密钥库临时读取。
- 自定义检测地址不得包含凭据、查询参数或片段，默认拒绝回环、局域网、链路本地、保留地址以及解析到这些地址的域名，并继续禁止 HTTP 重定向。
- 检测请求不读取或记录响应正文。限流、超时、网络故障、服务器故障和端点错误必须使用稳定的非敏感结果码区分，不能误判为 Key 无效。
- 修改供应商检测配置后，必须清除该供应商已有 Key 的旧状态；没有检测能力时不得发起网络请求。

## 9. 变更检查清单

- 界面改动：检查四种语言、长文本、键盘操作和减少动画设置。
- 设置改动：检查默认值、校验、完整对象合并、失败回滚和旧文件兼容。
- Rust 命令改动：检查前后端类型、命令注册和结构化错误码。
- 数据模型改动：不得静默改写用户已有名称、备注或其他业务数据。
- 安全改动：确认敏感信息不会写入前端持久化、设置文件或日志。
- 文档改动：同步 `development.md`、`development.en.md` 和 `development.zh-TW.md`。
- 完成后运行 `npm run build`、`npm run test:frontend`、`cargo fmt --all -- --check`、`cargo check --locked`、`cargo test --locked` 和 CI 使用的 `cargo clippy --all-targets --all-features --locked -- -D warnings`。
