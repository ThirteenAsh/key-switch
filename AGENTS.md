# Key Switch 项目开发规范

本文件约束所有在本仓库中工作的自动化开发代理。详细技术说明见 [`docs/development.md`](./docs/development.md)。

## 1. 基本要求

- 使用简体中文与用户沟通，代码标识符和协议字段按项目现有英文命名。
- 遇到不了解的实现、依赖或数据结构时，不要猜测，先在仓库中搜索并阅读相关代码。
- 修改前先检查工作区状态；已有改动属于用户，必须保留并避免覆盖无关内容。
- 只修改当前任务需要的文件，不进行未经授权的重构、依赖升级、文件删除或数据迁移。
- 实现功能时同步考虑类型、安全、错误处理、国际化、无障碍、兼容性和测试。
- 不提交生成目录、运行时数据或本地文件，包括 `node_modules/`、`dist/`、`src-tauri/target/`、日志、`settings.json` 和 `key-switch-data.json`。

## 2. 当前技术栈与代码边界

- 桌面容器：Tauri 2。
- 原生与敏感逻辑：Rust 2021。
- 前端：Vue 3、TypeScript、Vite 6、Vue Router、Pinia、Vue I18n 11。
- 完整 API Key：系统密钥库。
- 非敏感业务元数据：用户应用数据目录下的 `key-switch-data.json`。
- 非敏感应用偏好：用户应用数据目录下的 `settings.json`。

职责边界：

- `src/views/`：页面组合和页面级交互。
- `src/components/`：可复用组件、弹窗和基础 UI。
- `src/api/`：Tauri 命令的 TypeScript 类型与调用封装。
- `src/stores/`：Pinia 状态、业务操作和设置写入队列。
- `src/i18n/`：语言解析、翻译资源和错误码翻译。
- `src/data/`：内置供应商目录等非敏感静态数据。
- `src-tauri/src/`：持久化、密钥库、剪贴板、网络检测、更新和其他系统能力。
- `src-tauri/capabilities/`：Tauri 最小权限声明。

前端不得承担敏感数据持久化、API Key 解密或外部验证逻辑。

## 3. 前端开发规范

- Vue 组件使用 `<script setup lang="ts">`，保持严格类型，不使用无必要的 `any`。
- 页面不要直接调用 `invoke`；统一通过 `src/api/app.ts` 封装。
- 跨页面业务状态使用 Pinia；短生命周期、仅组件内部使用的状态保留在组件中。
- 可复用交互优先放入 `src/components/ui/`，保持现有轻量、圆角、低饱和度主题。
- 所有交互都要覆盖加载、空状态、失败和禁用状态。
- 自定义控件必须具备正确的 ARIA 语义和键盘操作。选择框至少支持方向键、Enter/Space、Escape、Tab 和点击外部关闭。
- 动画使用短时长和现有缓动风格，并支持 `prefers-reduced-motion`。
- 不将完整 API Key 放入 Pinia、DOM 持久状态、URL、浏览器存储或错误对象。

## 4. 国际化规范

当前支持 `zh-CN`、`zh-TW`、`en-US`、`ja-JP` 和 `system` 偏好。

- 所有用户可见文本都必须通过 Vue I18n，包括按钮、标题、Toast、错误、占位符、`title`、`aria-label` 和图片 `alt`。
- 使用按领域组织的语义键，不使用中文原文作为键。
- `src/i18n/messages/zh-CN.ts` 是 `MessageSchema` 的结构基准。
- 新增、重命名或删除翻译键时，同步修改简中、繁中、英语和日语四套资源。
- 修改国际化资源后必须运行 `npm run build`，利用 TypeScript 检查语言键结构。
- 无法读取系统语言时使用简体中文；已读取但不支持的非中文语言使用英语。
- 自定义供应商名称和数据文件中已有的名称属于用户数据，不翻译、不随语言切换改名。
- 新增内置供应商时，中文界面使用中文名；所有非中文界面使用英文名，并把当时显示的名称写入数据文件。
- `src/data/providerCatalog.ts` 中的内置供应商必须使用稳定 ID，并同时提供中英文名称。

## 5. 设置文件开发规范

桌面端设置统一写入 Tauri 用户应用数据目录下的 `settings.json`。除一次性迁移旧 `key-switch.locale` 外，不得使用 `localStorage` 保存新设置。

当前协议：

```json
{
  "schemaVersion": 1,
  "localePreference": "system"
}
```

新增设置项时必须完成以下工作：

1. 在 Rust `AppSettings` 或嵌套设置结构中增加字段。
2. 在 Rust `Default` 中提供安全、明确的默认值。
3. 在 Rust 中校验枚举、范围、路径或其他约束。
4. 同步更新 TypeScript `AppSettings`。
5. 更新 `src/stores/settings.ts` 的 `normalizeSettings()`，确保完整保留全部已声明字段。
6. 通过 `settingsStore.updateSettings()` 保存局部修改，不绕过设置 Store 直接调用保存命令。
7. 补充设置界面、四套翻译和 Rust/前端验证。

兼容性规则：

- 设置命令每次写入完整对象，不是单字段补丁；遗漏字段会在下一次保存时丢失。
- 仅新增带默认值的兼容字段时可以保留当前 `schemaVersion`。
- 删除字段、修改类型、重命名字段或改变字段含义时，必须提升版本并实现旧版本到新版本的显式迁移。
- 不得只提升版本常量；当前读取逻辑会拒绝不受支持的版本。
- 已有设置文件优先于旧 `localStorage`；只有首次创建设置文件时允许迁移旧语言偏好。
- 必须保留前端写入队列、Rust `SETTINGS_LOCK`、临时文件、落盘同步、备份替换和失败回滚机制。
- `settings.json` 只能保存非敏感偏好，绝不保存 API Key、访问令牌、主密钥或可用于恢复凭据的材料。

## 6. Rust 与 Tauri 命令规范

- 新增命令使用 `#[tauri::command]`，并在 `tauri::generate_handler!` 中注册。
- 命令参数和返回值使用明确的 Serde 结构，并与 `src/api/app.ts` 的 TypeScript 类型保持一致。
- Rust 对前端只返回稳定、非敏感的 `CommandError.code`，不要返回包含系统路径、网络响应或底层库详情的原始错误。
- 前端通过 `src/i18n/errors.ts` 翻译错误码；业务状态不得依赖匹配中文或英文错误文本。
- 新增错误场景时，同步增加 Rust 错误码分类、TypeScript 映射、四套翻译和测试。
- 文件写入必须明确处理创建目录、序列化失败、写入失败和中断恢复。
- 外部请求必须设置超时、响应体大小上限，并限制到明确允许的端点。
- 新增插件或系统能力前检查 `src-tauri/capabilities/`，只申请完成任务所需的最小权限。

## 7. API Key 安全底线

- 完整 API Key 只能在 Rust 侧和系统密钥库之间流转。
- 列表命令只返回掩码与非敏感元数据。
- 不在日志、错误、遥测、URL、截图、测试夹具、Issue 或 PR 中记录真实凭据。
- 复制和检测由 Rust 完成；前端不得请求或长期持有完整 Key。
- 尽量缩短明文在内存中的生命周期，并避免在格式化错误中携带敏感值。
- 删除供应商或 Key 时，必须同步处理系统密钥库和元数据的一致性；失败时不得静默丢失关联。

## 8. 数据与兼容性规范

- 用户数据优先，禁止因语言切换、目录升级或默认值变化静默改写已有供应商名称和 Key 备注。
- 修改 JSON 数据结构时提供默认值或显式迁移，并为旧文件增加测试。
- 保存失败时要维持磁钥库、内存状态和元数据文件之间的一致性，必要时执行回滚。
- 自定义供应商输入必须继续校验名称、URL 协议、图片类型和大小。
- 不在没有迁移与回滚方案时改变运行时文件名、应用标识符或密钥库 service 名称。

## 9. 文档规范

- 技术行为变化时更新 `docs/development.md`。
- 同步维护 `docs/development.en.md` 和 `docs/development.zh-TW.md`，三份文档的章节和规则应保持一致。
- README 只保留面向用户的项目介绍和入口；详细实现与开发约定放在开发文档。
- 文档中的版本、命令、文件名和目录必须与仓库当前实现一致，不描述已经过时的规划状态。

## 10. 完成前验证

按改动范围执行，涉及前后端协议、设置或安全逻辑时必须全部执行：

```bash
npm run build

cd src-tauri
cargo fmt --all -- --check
cargo check --locked
cargo test --locked
```

另外检查：

- `git diff --check` 无空白错误。
- 没有残留未国际化的用户可见文案。
- 没有把敏感值、运行时数据或构建产物加入提交。
- 新设置能够读取、保存、失败回滚，并兼容旧文件。
- 新 UI 在四种语言、键盘操作和减少动画模式下可用。
