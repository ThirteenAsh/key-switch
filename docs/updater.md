# 更新基础设施

当前 Alpha 版本仅检查 GitHub Releases：设置页会查询公开 Release，使用 SemVer 比较版本，发现新版本后引导用户前往 GitHub 下载。应用不会下载或安装更新。

`tauri-plugin-updater` 的前端与 Rust 依赖已安装，但 Rust 侧通过 `AUTOMATIC_UPDATES_ENABLED = false` 保持未注册状态，能力文件也没有开放 Updater 权限；`bundle.createUpdaterArtifacts` 同样保持为 `false`。

## 正式版启用自动更新

1. 使用 Tauri CLI 生成并离线备份更新签名密钥；私钥及密码只保存到发布环境的 Secrets，公钥写入 `tauri.conf.json`。
2. 配置 GitHub Release 中的 `latest.json` 更新端点，并将 `AUTOMATIC_UPDATES_ENABLED` 改为 `true`。
3. 将 `bundle.createUpdaterArtifacts` 改为 `true`，在发布流水线中注入签名私钥。
4. 在能力文件中加入 Updater 所需权限，并接入下载进度、用户确认、安装及重启流程。
5. 完成 Windows 安装、签名失败、网络中断、版本回退与跨架构更新测试后，才可开放应用内安装。

签名私钥一旦用于首个正式版本，必须长期安全保管；不得提交到仓库、日志或发布附件。
