## Key Switch v{{VERSION}}

Key Switch 是一款本地 API Key 管理桌面应用。v1.1.0 重构主页面和设置导航，让供应商与 Key 的管理更紧凑。

### 改进

- 主页面改为左侧供应商列表、右侧对应 Key 的布局，支持拖动排序、键盘调整顺序和供应商切换动画。
- 供应商管理与设置入口移至左下角，设置分为通用、数据与日志、关于三个分类。
- 统一主页面与设置页的尺寸，收窄侧栏，精简图标、标题及统计信息。
- 保留右侧 Key 备注与掩码搜索，以及新增、编辑、复制、删除和检测操作。

### 修复

- 修复 Key 操作按钮和英文导航超出容器的问题。
- 供应商排序保存失败时恢复原顺序，并显示错误提示。
- 增加供应商加载失败后的重试入口。

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
