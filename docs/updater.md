# 更新基础设施

当前 RC 版本通过 GitHub Releases 检查更新：设置页查询公开 Release，使用 SemVer 比较版本，再从目标 Release 的 `latest.json` 下载并验证签名更新包。

`tauri-plugin-updater` 仅在 Rust 侧注册，前端通过受控 Tauri 命令安装指定 Release，不开放通用 Updater 权限。发布构建通过 `bundle.createUpdaterArtifacts = true` 生成更新包、签名与清单。

## 发布要求

1. 使用 Tauri CLI 生成并离线备份更新签名密钥；私钥及密码只保存到发布环境的 Secrets，公钥写入 `tauri.conf.json`。
2. Git 标签必须与 `tauri.conf.json` 版本完全一致，例如 `v1.0.0-rc.1`。
3. 发布流水线必须注入签名私钥，并上传每个平台的更新包、签名和 `latest.json`。
4. Release 初始为草稿；检查附件完整后必须手动发布，客户端才能访问。
5. 正式发布前完成 Windows 安装、签名失败、网络中断、版本回退与跨架构更新测试。

签名私钥一旦用于首个正式版本，必须长期安全保管；不得提交到仓库、日志或发布附件。
