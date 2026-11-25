## Key Switch v{{VERSION}}

Key Switch 是一款本地 API Key 管理桌面应用。v{{VERSION}} 支持在供应商管理中编辑自定义供应商的图标。

### 改进

- 在自定义供应商的配置弹窗中可预览、替换和移除图标。支持最大 2 MB 的 PNG、JPG 和 WebP。
- 保存后，供应商管理卡片与主页面侧栏同步更新。移除图标后恢复默认图标。
- 供应商和 Key 元数据通过临时文件原子保存。保存出错时，配置弹窗保留编辑内容，方便重试。

### 修复

- 保存供应商配置时等待正在运行的 Key 检测完成，避免旧检测结果覆盖新配置。
- 未填写平台管理地址的自定义供应商也可以保存图标修改。

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
