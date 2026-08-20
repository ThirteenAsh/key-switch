# Key Switch 二次开发文档

简体中文 · [English](./development.en.md) · [繁體中文](./development.zh-TW.md)

## 1. 开发环境

- Node.js 20 LTS 或更高版本
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

浏览器模式不具备系统密钥库、剪贴板和本地数据目录等 Tauri 能力。

## 3. 常用检查

```bash
# 前端类型检查与构建
npm run build

# Rust 格式、编译和测试
cd src-tauri
cargo fmt --all -- --check
cargo check --locked
cargo test --locked
```

## 4. 目录职责

| 目录 | 说明 |
| --- | --- |
| `src/views/` | 仪表盘、供应商、设置页面 |
| `src/components/` | 可复用 Vue 组件和弹窗 |
| `src/api/` | 前端调用 Tauri 命令的封装 |
| `src/stores/` | Pinia 状态与业务操作 |
| `src-tauri/src/` | Rust 命令、本地数据和系统能力 |
| `src-tauri/capabilities/` | Tauri 权限声明 |

新增前端功能时，保持 Vue `<script setup lang="ts">` 风格；新增原生能力时，在 Rust 中使用 `#[tauri::command]` 并在 `src-tauri/src/lib.rs` 注册命令。

## 5. 安全约定

- 完整 API Key 只能由 Rust 侧读取和处理，不要放入日志、URL、DOM 持久状态或前端持久化存储。
- 列表接口只返回掩码后的 Key；敏感数据使用系统密钥库保存。
- 外部检测只访问用户配置的地址，并设置超时和明确的错误分类。
- 不要在源码、测试数据、Issue、PR 或截图中提交真实 API Key。
