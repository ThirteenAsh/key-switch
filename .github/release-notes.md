## Key Switch v{{VERSION}}

Key Switch 是一款本地 API Key 管理桌面应用。v{{VERSION}} 改进自动更新连接与本地日志，并精简首页的 Key 列标题。

### 改进

- 更新检测、清单获取和安装包下载支持代理环境变量及受支持的系统代理；没有代理时直接连接。
- 通过代理更新失败时，手动操作可选择本次使用直连重试，也可取消。该选择不会保存为永久偏好，下一次更新重新使用默认代理策略。
- 直连重试仍失败时沿用现有错误提示；启动自动检查失败保持安静，不弹出确认窗口。
- 直连确认弹窗沿用应用主题和动画，支持四种语言、键盘操作与减少动画设置。
- 日志补充版本、系统、会话、操作阶段、代理来源、耗时、下载字节数和签名校验结果，便于排查更新问题。
- 日志文件达到 1 MB 时轮转，保留 3 个备份；清空日志会同时清理备份。
- 首页四种语言的 API Key 列标题统一显示为 `API Key`。

### 修复

- 启用系统代理支持，改善仅开启系统代理时更新请求仍直接连接的问题。
- 补齐 Key 检测中 DNS 解析等提前返回路径的失败日志。
- 加强凭据与 URL 脱敏，补充日志写入失败时的备用输出、前端资源加载异常和 Rust panic 位置记录。
- 调整网络图标与标题的对齐，将两段更新说明统一排列。

### 数据兼容

- 保持供应商与 Key 元数据、设置文件结构和系统密钥库标识不变，无需数据迁移。
- 兼容现有供应商名称和 Key 备注；语言切换不会改写已有用户数据。

### 下载

请在本 Release 的 **Assets** 中下载对应系统的安装包。`Source code` 与 macOS 的 `.app.tar.gz` 不适用于普通安装。

#### Windows（x64）

- 推荐下载 `Key.Switch_{{VERSION}}_x64_en-US.msi`。
- 也可下载 `Key.Switch_{{VERSION}}_x64-setup.exe`；两者任选其一安装。
- 当前版本尚未进行代码签名。如 Windows SmartScreen 提示风险，请先确认文件来自本 Release 页面，再选择“更多信息”→“仍要运行”。

#### macOS

- **Apple Silicon（M1 / M2 / M3 / M4）**：下载 `Key.Switch_{{VERSION}}_aarch64.dmg`。
- **Intel Mac**：下载 `Key.Switch_{{VERSION}}_x64.dmg`。
- 打开 `.dmg` 后，将 Key Switch 拖入“应用程序（Applications）”文件夹。
- 当前版本尚未进行 Apple 公证或签名。如系统阻止打开，请先确认文件来自本 Release 页面，再在“系统设置 → 隐私与安全性”中选择“仍要打开”。

#### Linux（x86_64 / amd64）

- **Ubuntu / Debian**：下载 `Key.Switch_{{VERSION}}_amd64.deb`，可使用系统软件安装器打开。
- **Fedora / RHEL / openSUSE**：下载 `Key.Switch-{{VERSION}}-1.x86_64.rpm`，可使用系统软件安装器打开。
- **其他 Linux 发行版**：下载 `Key.Switch_{{VERSION}}_amd64.AppImage`，赋予执行权限后运行：

  ```bash
  chmod +x Key.Switch_{{VERSION}}_amd64.AppImage
  ./Key.Switch_{{VERSION}}_amd64.AppImage
  ```

### 已包含

- 集中管理多个 API 服务商及其 Key。
- 支持内置和自定义供应商的新增、编辑、删除与排序。
- API Key 保存至系统凭据库；列表默认仅显示掩码，可按需复制。
- 支持通过供应商配置的地址检测 Key 状态。
- 在本地保存供应商、备注和状态等业务数据，不依赖云端账户。
- 支持检查可用的应用更新。

### 注意事项

- 删除供应商会同时删除其关联的 API Key 凭据，请谨慎操作。
- 请保留原始 API Key 或自行准备备份方案。

### 反馈

欢迎通过 GitHub Issues 提交问题、建议和使用反馈。
