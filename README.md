# Key Switch

Key Switch 是一个规划中的本地 API Key 管理桌面应用，用于集中管理不同服务商的 API Key、记录用途备注、检查可用状态，并在需要时安全复制或短暂查看密钥。

> 当前状态：已完成设计稿对应的前端界面骨架（仪表盘、供应商、设置、导航、搜索与折叠交互）。页面暂以脱敏演示数据驱动；数据库、真实 Key 管理、密钥安全存储和供应商状态检测仍在后续阶段。

开发命令：

```bash
npm install
npm run tauri:dev
```

详细范围、技术选型、安全原则和阶段验收标准见[首版实施计划](./plan/initial-implementation-plan.md)。
