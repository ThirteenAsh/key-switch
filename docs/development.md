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

# Rust 格式、编译和测试
cd src-tauri
cargo fmt --all -- --check
cargo check --locked
cargo test --locked
```

提交涉及前后端协议、设置文件或安全逻辑的改动前，必须同时执行前端构建和 Rust 测试。

## 4. 架构与目录职责

| 目录 | 说明 |
| --- | --- |
| `src/views/` | 仪表盘、供应商和设置页面 |
| `src/components/` | 可复用 Vue 组件、弹窗和基础 UI 组件 |
| `src/api/` | Tauri 命令的 TypeScript 类型与调用封装 |
| `src/stores/` | Pinia 状态、业务操作和设置持久化队列 |
| `src/i18n/` | 语言解析、Vue I18n 初始化、错误翻译和四套语言资源 |
| `src/data/` | 内置供应商目录等非敏感静态数据 |
| `src-tauri/src/` | Rust 命令、本地文件、密钥库、网络检测和更新能力 |
| `src-tauri/capabilities/` | Tauri 最小权限声明 |
| `docs/` | 项目开发文档和 README 资源 |

前端负责展示、交互和非敏感状态；敏感数据、持久化文件、外部请求和系统能力必须放在 Rust 侧。新增前端代码保持 Vue `<script setup lang="ts">` 风格和完整类型定义。

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
  "localePreference": "system"
}
```

设置启动流程：

1. `src/main.ts` 在挂载 Vue 前调用设置 Store。
2. Rust 读取并校验 `settings.json`；首次运行时创建默认文件。
3. 升级用户如果仍有旧的 `key-switch.locale`，只在首次创建设置文件时迁移。
4. 成功迁移后删除旧 `localStorage` 键。
5. 设置返回前端后再挂载界面，避免语言闪切。

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

## 9. 变更检查清单

- 界面改动：检查四种语言、长文本、键盘操作和减少动画设置。
- 设置改动：检查默认值、校验、完整对象合并、失败回滚和旧文件兼容。
- Rust 命令改动：检查前后端类型、命令注册和结构化错误码。
- 数据模型改动：不得静默改写用户已有名称、备注或其他业务数据。
- 安全改动：确认敏感信息不会写入前端持久化、设置文件或日志。
- 文档改动：同步 `development.md`、`development.en.md` 和 `development.zh-TW.md`。
- 完成后至少运行 `npm run build`、`cargo fmt --all -- --check`、`cargo check --locked` 和 `cargo test --locked`。
