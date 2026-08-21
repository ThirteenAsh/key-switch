use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    process::Command,
    sync::Mutex,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tauri::Manager;
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_updater::UpdaterExt;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AppInfo {
    version: String,
    data_directory: String,
    log_directory: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiKeyRecord {
    id: String,
    provider_id: String,
    remark: String,
    secret_id: String,
    status: String,
    last_checked_at: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProviderRecord {
    id: String,
    name: String,
    abbreviation: String,
    tone: String,
    logo: Option<String>,
    kind: String,
    platform_url: Option<String>,
    keys: Vec<ApiKeyRecord>,
}

#[derive(Default, Serialize, Deserialize)]
struct AppData {
    providers: Vec<ProviderRecord>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ApiKeySummary {
    id: String,
    provider_id: String,
    remark: String,
    masked_value: String,
    status: String,
    last_checked_at: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ProviderSummary {
    id: String,
    name: String,
    abbreviation: String,
    tone: String,
    logo: Option<String>,
    kind: String,
    platform_url: Option<String>,
    keys: Vec<ApiKeySummary>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateProviderInput {
    id: String,
    name: String,
    abbreviation: String,
    tone: String,
    logo: Option<String>,
    kind: String,
    platform_url: Option<String>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateProviderInput {
    id: String,
    name: String,
    platform_url: Option<String>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateKeyInput {
    provider_id: String,
    remark: String,
    value: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateInfo {
    current_version: String,
    latest_version: String,
    title: String,
    notes: String,
    release_url: String,
    prerelease: bool,
    published_at: Option<String>,
    release_tag: String,
}

#[derive(Deserialize)]
struct GithubRelease {
    tag_name: String,
    name: Option<String>,
    body: Option<String>,
    html_url: String,
    draft: bool,
    prerelease: bool,
    published_at: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum PrereleaseIdentifier {
    Numeric(u64),
    Text(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SemVersion {
    major: u64,
    minor: u64,
    patch: u64,
    prerelease: Vec<PrereleaseIdentifier>,
}

impl SemVersion {
    fn parse(value: &str) -> Option<Self> {
        let value = value.trim();
        let value = value.strip_prefix("app-").unwrap_or(value);
        let value = value
            .strip_prefix('v')
            .or_else(|| value.strip_prefix('V'))
            .unwrap_or(value);
        let value = value
            .split_once('+')
            .map(|(version, _)| version)
            .unwrap_or(value);
        let (core, prerelease) = value
            .split_once('-')
            .map(|(core, prerelease)| (core, Some(prerelease)))
            .unwrap_or((value, None));
        let mut numbers = core.split('.');
        let major = numbers.next()?.parse().ok()?;
        let minor = numbers.next()?.parse().ok()?;
        let patch = numbers.next()?.parse().ok()?;
        if numbers.next().is_some() {
            return None;
        }
        let prerelease = prerelease
            .map(|value| {
                value
                    .split('.')
                    .filter(|item| !item.is_empty())
                    .map(|item| {
                        item.parse::<u64>()
                            .map(PrereleaseIdentifier::Numeric)
                            .unwrap_or_else(|_| {
                                PrereleaseIdentifier::Text(item.to_ascii_lowercase())
                            })
                    })
                    .collect()
            })
            .unwrap_or_default();
        Some(Self {
            major,
            minor,
            patch,
            prerelease,
        })
    }

    fn is_prerelease(&self) -> bool {
        !self.prerelease.is_empty()
    }
}

impl Ord for SemVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        self.major
            .cmp(&other.major)
            .then_with(|| self.minor.cmp(&other.minor))
            .then_with(|| self.patch.cmp(&other.patch))
            .then_with(
                || match (self.prerelease.is_empty(), other.prerelease.is_empty()) {
                    (true, true) => Ordering::Equal,
                    (true, false) => Ordering::Greater,
                    (false, true) => Ordering::Less,
                    (false, false) => {
                        for (left, right) in self.prerelease.iter().zip(&other.prerelease) {
                            let ordering = match (left, right) {
                                (
                                    PrereleaseIdentifier::Numeric(left),
                                    PrereleaseIdentifier::Numeric(right),
                                ) => left.cmp(right),
                                (
                                    PrereleaseIdentifier::Numeric(_),
                                    PrereleaseIdentifier::Text(_),
                                ) => Ordering::Less,
                                (
                                    PrereleaseIdentifier::Text(_),
                                    PrereleaseIdentifier::Numeric(_),
                                ) => Ordering::Greater,
                                (
                                    PrereleaseIdentifier::Text(left),
                                    PrereleaseIdentifier::Text(right),
                                ) => left.cmp(right),
                            };
                            if ordering != Ordering::Equal {
                                return ordering;
                            }
                        }
                        self.prerelease.len().cmp(&other.prerelease.len())
                    }
                },
            )
    }
}

impl PartialOrd for SemVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Deserialize)]
struct UpdateKeyInput {
    id: String,
    remark: String,
    value: String,
}

fn now() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .to_string()
}
const KEYRING_SERVICE: &str = "com.app.key-switch";
const LOG_FILE_NAME: &str = "key-switch.log";
const LOG_BACKUP_FILE_NAME: &str = "key-switch.log.1";
const MAX_LOG_FILE_SIZE: u64 = 1024 * 1024;
const AUTOMATIC_UPDATES_ENABLED: bool = true;
static LOG_LOCK: Mutex<()> = Mutex::new(());

fn data_file(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    Ok(app
        .path()
        .app_data_dir()
        .map_err(|e| format!("无法解析应用数据目录：{e}"))?
        .join("key-switch-data.json"))
}

fn log_directory(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    Ok(app
        .path()
        .app_data_dir()
        .map_err(|e| format!("无法解析应用数据目录：{e}"))?
        .join("logs"))
}

fn log_file(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    Ok(log_directory(app)?.join(LOG_FILE_NAME))
}

fn append_log(
    app: &tauri::AppHandle,
    level: &str,
    event: &str,
    detail: &str,
) -> Result<(), String> {
    let _guard = LOG_LOCK
        .lock()
        .map_err(|_| "日志写入锁不可用".to_string())?;
    let directory = log_directory(app)?;
    fs::create_dir_all(&directory).map_err(|e| format!("无法创建日志目录：{e}"))?;
    let file = log_file(app)?;

    if file.metadata().map(|metadata| metadata.len()).unwrap_or(0) >= MAX_LOG_FILE_SIZE {
        let backup = directory.join(LOG_BACKUP_FILE_NAME);
        if backup.exists() {
            fs::remove_file(&backup).map_err(|e| format!("无法轮转旧日志：{e}"))?;
        }
        fs::rename(&file, backup).map_err(|e| format!("无法轮转日志：{e}"))?;
    }

    let mut output = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file)
        .map_err(|e| format!("无法打开日志文件：{e}"))?;
    writeln!(output, "{} [{}] {} {}", now(), level, event, detail)
        .map_err(|e| format!("无法写入日志：{e}"))
}

fn open_directory(directory: PathBuf) -> Result<(), String> {
    fs::create_dir_all(&directory).map_err(|e| format!("无法创建目录：{e}"))?;
    #[cfg(target_os = "windows")]
    let mut command = Command::new("explorer");
    #[cfg(target_os = "macos")]
    let mut command = Command::new("open");
    #[cfg(all(unix, not(target_os = "macos")))]
    let mut command = Command::new("xdg-open");
    command
        .arg(directory)
        .spawn()
        .map_err(|e| format!("无法打开目录：{e}"))?;
    Ok(())
}
fn load_data(app: &tauri::AppHandle) -> Result<AppData, String> {
    let file = data_file(app)?;
    if !file.exists() {
        return Ok(AppData::default());
    }
    serde_json::from_str(&fs::read_to_string(file).map_err(|e| format!("无法读取本地数据：{e}"))?)
        .map_err(|e| format!("本地数据格式错误：{e}"))
}
fn save_data(app: &tauri::AppHandle, data: &AppData) -> Result<(), String> {
    let file = data_file(app)?;
    fs::create_dir_all(file.parent().ok_or("无法创建应用数据目录")?)
        .map_err(|e| format!("无法创建应用数据目录：{e}"))?;
    fs::write(
        file,
        serde_json::to_string_pretty(data).map_err(|e| format!("无法序列化本地数据：{e}"))?,
    )
    .map_err(|e| format!("无法保存本地数据：{e}"))
}

fn keyring_entry(secret_id: &str) -> Result<Entry, String> {
    Entry::new(KEYRING_SERVICE, secret_id).map_err(|e| format!("系统密钥库不可用：{e}"))
}
fn key_value(key: &ApiKeyRecord) -> Result<String, String> {
    keyring_entry(&key.secret_id)?
        .get_password()
        .map_err(|e| format!("无法读取系统密钥库中的 API Key：{e}"))
}
fn mask(value: &str) -> String {
    let count = value.chars().count();
    if count <= 8 {
        return "••••••••".into();
    }
    format!(
        "{}••••{}",
        value.chars().take(4).collect::<String>(),
        value.chars().skip(count - 4).collect::<String>()
    )
}
fn key_summary(key: &ApiKeyRecord) -> Result<ApiKeySummary, String> {
    Ok(ApiKeySummary {
        id: key.id.clone(),
        provider_id: key.provider_id.clone(),
        remark: key.remark.clone(),
        masked_value: mask(&key_value(key)?),
        status: key.status.clone(),
        last_checked_at: key.last_checked_at.clone(),
    })
}
fn summary(provider: &ProviderRecord) -> Result<ProviderSummary, String> {
    Ok(ProviderSummary {
        id: provider.id.clone(),
        name: provider.name.clone(),
        abbreviation: provider.abbreviation.clone(),
        tone: provider.tone.clone(),
        logo: provider.logo.clone(),
        kind: provider.kind.clone(),
        platform_url: provider.platform_url.clone(),
        keys: provider
            .keys
            .iter()
            .map(key_summary)
            .collect::<Result<Vec<_>, _>>()?,
    })
}

enum KeyValidationSpec {
    Bearer(&'static str),
    ApiKeyHeader {
        url: &'static str,
        header_name: &'static str,
    },
    Anthropic,
}

fn key_validation_spec(provider_id: &str) -> Option<KeyValidationSpec> {
    match provider_id {
        "openai" => Some(KeyValidationSpec::Bearer(
            "https://api.openai.com/v1/models",
        )),
        "claude" | "anthropic" => Some(KeyValidationSpec::Anthropic),
        "gemini" | "aistudio" => Some(KeyValidationSpec::ApiKeyHeader {
            url: "https://generativelanguage.googleapis.com/v1beta/models?pageSize=1",
            header_name: "x-goog-api-key",
        }),
        "deepseek" => Some(KeyValidationSpec::Bearer("https://api.deepseek.com/models")),
        "mimo" => Some(KeyValidationSpec::ApiKeyHeader {
            url: "https://api.xiaomimimo.com/v1/models",
            header_name: "api-key",
        }),
        "qwen" => Some(KeyValidationSpec::Bearer(
            "https://dashscope.aliyuncs.com/api/v1/deployments?page_no=1&page_size=1",
        )),
        "kimi" => Some(KeyValidationSpec::Bearer(
            "https://api.moonshot.cn/v1/models",
        )),
        "grok" => Some(KeyValidationSpec::Bearer("https://api.x.ai/v1/models")),
        "openrouter" => Some(KeyValidationSpec::Bearer(
            "https://openrouter.ai/api/v1/key",
        )),
        "minimax" => Some(KeyValidationSpec::Bearer(
            "https://api.minimaxi.com/v1/models",
        )),
        "doubao" => Some(KeyValidationSpec::Bearer(
            "https://ark.cn-beijing.volces.com/ping",
        )),
        "hunyuan" => Some(KeyValidationSpec::Bearer(
            "https://tokenhub.tencentmaas.com/v1/models",
        )),
        "qianfan" => Some(KeyValidationSpec::Bearer(
            "https://qianfan.baidubce.com/v2/models",
        )),
        "zhipu" => Some(KeyValidationSpec::Bearer(
            "https://open.bigmodel.cn/api/paas/v4/files",
        )),
        _ => None,
    }
}

fn classify_validation_status(status: reqwest::StatusCode) -> &'static str {
    if status.is_success() {
        "valid"
    } else if status == reqwest::StatusCode::BAD_REQUEST
        || status == reqwest::StatusCode::UNAUTHORIZED
    {
        "invalid"
    } else {
        "error"
    }
}

fn validation_client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .connect_timeout(Duration::from_secs(5))
        .redirect(reqwest::redirect::Policy::none())
        .user_agent("Key-Switch/1.0.0-rc")
        .build()
        .map_err(|e| format!("无法初始化网络客户端：{e}"))
}

async fn validate_key(client: &reqwest::Client, provider_id: &str, value: &str) -> &'static str {
    let Some(spec) = key_validation_spec(provider_id) else {
        return "error";
    };

    let request = match spec {
        KeyValidationSpec::Bearer(url) => client.get(url).bearer_auth(value),
        KeyValidationSpec::ApiKeyHeader { url, header_name } => {
            client.get(url).header(header_name, value)
        }
        KeyValidationSpec::Anthropic => client
            .get("https://api.anthropic.com/v1/models?limit=1")
            .header("x-api-key", value)
            .header("anthropic-version", "2023-06-01"),
    };

    match request.send().await {
        Ok(response) => classify_validation_status(response.status()),
        Err(_) => "error",
    }
}

#[cfg(test)]
mod tests {
    use super::{classify_validation_status, SemVersion};
    use reqwest::StatusCode;

    #[test]
    fn classifies_key_validation_responses() {
        assert_eq!(classify_validation_status(StatusCode::OK), "valid");
        assert_eq!(
            classify_validation_status(StatusCode::BAD_REQUEST),
            "invalid"
        );
        assert_eq!(
            classify_validation_status(StatusCode::UNAUTHORIZED),
            "invalid"
        );
        assert_eq!(classify_validation_status(StatusCode::FORBIDDEN), "error");
        assert_eq!(
            classify_validation_status(StatusCode::TOO_MANY_REQUESTS),
            "error"
        );
        assert_eq!(
            classify_validation_status(StatusCode::INTERNAL_SERVER_ERROR),
            "error"
        );
    }

    #[test]
    fn compares_release_versions_using_semver_precedence() {
        let alpha = SemVersion::parse("v0.0.2-alpha").unwrap();
        let next_alpha = SemVersion::parse("app-v0.0.3-alpha.1").unwrap();
        let stable = SemVersion::parse("0.0.3").unwrap();
        assert!(next_alpha > alpha);
        assert!(stable > next_alpha);
        assert!(SemVersion::parse("invalid").is_none());
    }
}

#[tauri::command]
fn get_app_info(app: tauri::AppHandle) -> Result<AppInfo, String> {
    let log_directory = log_directory(&app)?;
    Ok(AppInfo {
        version: app.package_info().version.to_string(),
        data_directory: app
            .path()
            .app_data_dir()
            .map_err(|e| format!("无法解析应用数据目录：{e}"))?
            .display()
            .to_string(),
        log_directory: log_directory.display().to_string(),
    })
}

#[tauri::command]
async fn check_for_updates(app: tauri::AppHandle) -> Result<Option<UpdateInfo>, String> {
    const RELEASES_API: &str =
        "https://api.github.com/repos/ThirteenAsh/key-switch/releases?per_page=20";
    const RELEASE_URL_PREFIX: &str = "https://github.com/ThirteenAsh/key-switch/releases/";
    const MAX_RESPONSE_SIZE: usize = 512 * 1024;

    let current_version_text = app.package_info().version.to_string();
    let current_version =
        SemVersion::parse(&current_version_text).ok_or("当前应用版本不符合 SemVer 规范")?;
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(12))
        .connect_timeout(Duration::from_secs(5))
        .redirect(reqwest::redirect::Policy::limited(3))
        .user_agent("Key-Switch-Update-Check/1.0.0-rc")
        .build()
        .map_err(|e| format!("无法初始化更新检测客户端：{e}"))?;
    let response = client
        .get(RELEASES_API)
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await
        .map_err(|e| format!("无法连接 GitHub Releases：{e}"))?;
    if !response.status().is_success() {
        return Err(format!(
            "GitHub Releases 返回异常状态：{}",
            response.status()
        ));
    }
    if response.content_length().unwrap_or(0) > MAX_RESPONSE_SIZE as u64 {
        return Err("GitHub Releases 响应过大".into());
    }
    let body = response
        .bytes()
        .await
        .map_err(|e| format!("无法读取 GitHub Releases 响应：{e}"))?;
    if body.len() > MAX_RESPONSE_SIZE {
        return Err("GitHub Releases 响应过大".into());
    }
    let releases: Vec<GithubRelease> =
        serde_json::from_slice(&body).map_err(|e| format!("GitHub Releases 数据格式错误：{e}"))?;

    let candidate = releases
        .into_iter()
        .filter(|release| !release.draft && release.html_url.starts_with(RELEASE_URL_PREFIX))
        .filter_map(|release| {
            SemVersion::parse(&release.tag_name).map(|version| (release, version))
        })
        .filter(|(release, version)| {
            version > &current_version && (current_version.is_prerelease() || !release.prerelease)
        })
        .max_by(|(_, left), (_, right)| left.cmp(right));

    let Some((release, _)) = candidate else {
        let _ = append_log(&app, "INFO", "update_checked", "available=false");
        return Ok(None);
    };
    let notes = release
        .body
        .unwrap_or_default()
        .chars()
        .take(4000)
        .collect();
    let update = UpdateInfo {
        current_version: current_version_text,
        latest_version: release
            .tag_name
            .trim_start_matches("app-")
            .trim_start_matches('v')
            .into(),
        title: release.name.unwrap_or_else(|| release.tag_name.clone()),
        notes,
        release_url: release.html_url,
        prerelease: release.prerelease,
        published_at: release.published_at,
        release_tag: release.tag_name,
    };
    let _ = append_log(&app, "INFO", "update_checked", "available=true");
    Ok(Some(update))
}

#[tauri::command]
async fn install_update(app: tauri::AppHandle, release_tag: String) -> Result<(), String> {
    const RELEASE_TAG_PREFIX: &str = "v";
    const LEGACY_RELEASE_TAG_PREFIX: &str = "app-v";

    let tag_is_safe = release_tag.len() <= 64
        && release_tag
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '.' | '-'))
        && (release_tag.starts_with(RELEASE_TAG_PREFIX)
            || release_tag.starts_with(LEGACY_RELEASE_TAG_PREFIX));
    if !tag_is_safe {
        return Err("更新版本标签无效".into());
    }
    let expected_version = SemVersion::parse(&release_tag).ok_or("更新版本不符合 SemVer 规范")?;
    let endpoint = reqwest::Url::parse(&format!(
        "https://github.com/ThirteenAsh/key-switch/releases/download/{release_tag}/latest.json"
    ))
    .map_err(|e| format!("无法生成更新清单地址：{e}"))?;
    let updater = app
        .updater_builder()
        .endpoints(vec![endpoint])
        .map_err(|e| format!("无法配置更新端点：{e}"))?
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("无法初始化自动更新：{e}"))?;
    let update = updater
        .check()
        .await
        .map_err(|e| format!("无法读取签名更新清单：{e}"))?
        .ok_or("该 Release 没有可安装的更新")?;
    let manifest_version =
        SemVersion::parse(&update.version).ok_or("签名更新清单中的版本不符合 SemVer 规范")?;
    if manifest_version != expected_version {
        return Err("Release 标签与签名更新清单版本不一致".into());
    }

    let _ = append_log(
        &app,
        "INFO",
        "update_install_started",
        "signature_check=pending",
    );
    update
        .download_and_install(|_, _| {}, || {})
        .await
        .map_err(|e| format!("更新签名验证或安装失败：{e}"))?;
    let _ = append_log(&app, "INFO", "update_installed", "restart=pending");
    app.restart();
}
#[tauri::command]
fn open_data_directory(app: tauri::AppHandle) -> Result<(), String> {
    let directory = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("无法解析应用数据目录：{e}"))?;
    open_directory(directory)?;
    let _ = append_log(&app, "INFO", "data_directory_opened", "success");
    Ok(())
}

#[tauri::command]
fn open_log_directory(app: tauri::AppHandle) -> Result<(), String> {
    open_directory(log_directory(&app)?)?;
    let _ = append_log(&app, "INFO", "log_directory_opened", "success");
    Ok(())
}

#[tauri::command]
fn clear_logs(app: tauri::AppHandle) -> Result<(), String> {
    let _guard = LOG_LOCK
        .lock()
        .map_err(|_| "日志写入锁不可用".to_string())?;
    let directory = log_directory(&app)?;
    fs::create_dir_all(&directory).map_err(|e| format!("无法创建日志目录：{e}"))?;
    for file_name in [LOG_FILE_NAME, LOG_BACKUP_FILE_NAME] {
        let file = directory.join(file_name);
        if file.exists() {
            fs::remove_file(file).map_err(|e| format!("无法清空日志：{e}"))?;
        }
    }
    Ok(())
}
#[tauri::command]
fn list_providers(app: tauri::AppHandle) -> Result<Vec<ProviderSummary>, String> {
    load_data(&app)?.providers.iter().map(summary).collect()
}
#[tauri::command]
fn create_provider(
    app: tauri::AppHandle,
    input: CreateProviderInput,
) -> Result<ProviderSummary, String> {
    if input.name.trim().is_empty() {
        return Err("供应商名称不能为空".into());
    }
    let mut data = load_data(&app)?;
    if data
        .providers
        .iter()
        .any(|p| p.id == input.id || p.name == input.name.trim())
    {
        return Err("供应商已存在".into());
    }
    let provider = ProviderRecord {
        id: input.id,
        name: input.name.trim().into(),
        abbreviation: input.abbreviation,
        tone: input.tone,
        logo: input.logo,
        kind: input.kind,
        platform_url: input.platform_url.filter(|u| !u.trim().is_empty()),
        keys: vec![],
    };
    let result = summary(&provider)?;
    data.providers.push(provider);
    save_data(&app, &data)?;
    let _ = append_log(&app, "INFO", "provider_created", "success");
    Ok(result)
}
#[tauri::command]
fn update_provider(
    app: tauri::AppHandle,
    input: UpdateProviderInput,
) -> Result<ProviderSummary, String> {
    let mut data = load_data(&app)?;
    if input.name.trim().is_empty() {
        return Err("供应商名称不能为空".into());
    }
    if data
        .providers
        .iter()
        .any(|p| p.id != input.id && p.name == input.name.trim())
    {
        return Err("供应商名称已存在".into());
    }
    let provider = data
        .providers
        .iter_mut()
        .find(|p| p.id == input.id)
        .ok_or("未找到供应商")?;
    provider.name = input.name.trim().into();
    provider.platform_url = input.platform_url.filter(|u| !u.trim().is_empty());
    let result = summary(provider)?;
    save_data(&app, &data)?;
    let _ = append_log(&app, "INFO", "provider_updated", "success");
    Ok(result)
}
#[tauri::command]
fn delete_provider(app: tauri::AppHandle, provider_id: String) -> Result<(), String> {
    let mut data = load_data(&app)?;
    let provider_index = data
        .providers
        .iter()
        .position(|provider| provider.id == provider_id)
        .ok_or("未找到供应商")?;
    let provider = data.providers[provider_index].clone();
    let removed_key_count = provider.keys.len();
    for key in &provider.keys {
        keyring_entry(&key.secret_id)?
            .delete_credential()
            .map_err(|e| format!("无法从系统密钥库删除 API Key：{e}"))?;
    }
    data.providers.remove(provider_index);
    save_data(&app, &data)?;
    let _ = append_log(
        &app,
        "INFO",
        "provider_deleted",
        &format!("key_count={removed_key_count}"),
    );
    Ok(())
}
#[tauri::command]
fn reorder_providers(app: tauri::AppHandle, provider_ids: Vec<String>) -> Result<(), String> {
    let mut data = load_data(&app)?;
    if provider_ids.len() != data.providers.len() {
        return Err("供应商排序数据不完整".into());
    }
    data.providers.sort_by_key(|p| {
        provider_ids
            .iter()
            .position(|id| id == &p.id)
            .unwrap_or(usize::MAX)
    });
    save_data(&app, &data)?;
    let _ = append_log(&app, "INFO", "providers_reordered", "success");
    Ok(())
}
#[tauri::command]
fn create_api_key(app: tauri::AppHandle, input: CreateKeyInput) -> Result<ApiKeySummary, String> {
    if input.value.trim().is_empty() {
        return Err("API Key 不能为空".into());
    }
    let mut data = load_data(&app)?;
    let provider = data
        .providers
        .iter_mut()
        .find(|p| p.id == input.provider_id)
        .ok_or("未找到供应商")?;
    let id = format!("key-{}", now());
    keyring_entry(&id)?
        .set_password(input.value.trim())
        .map_err(|e| format!("无法写入系统密钥库：{e}"))?;
    let key = ApiKeyRecord {
        id: id.clone(),
        provider_id: input.provider_id,
        remark: if input.remark.trim().is_empty() {
            "未命名 Key".into()
        } else {
            input.remark.trim().into()
        },
        secret_id: id,
        status: "untested".into(),
        last_checked_at: None,
    };
    let result = key_summary(&key)?;
    provider.keys.push(key);
    save_data(&app, &data)?;
    let _ = append_log(&app, "INFO", "api_key_created", "success");
    Ok(result)
}

#[tauri::command]
fn update_api_key(app: tauri::AppHandle, input: UpdateKeyInput) -> Result<ApiKeySummary, String> {
    if input.value.trim().is_empty() {
        return Err("请输入新的 API Key".into());
    }

    let mut data = load_data(&app)?;
    let key = data
        .providers
        .iter_mut()
        .flat_map(|provider| &mut provider.keys)
        .find(|key| key.id == input.id)
        .ok_or("未找到 API Key")?;
    let entry = keyring_entry(&key.secret_id)?;
    let previous_value = entry
        .get_password()
        .map_err(|e| format!("无法读取系统密钥库中的 API Key：{e}"))?;

    let next_value = input.value.trim();
    entry
        .set_password(next_value)
        .map_err(|e| format!("无法写入系统密钥库：{e}"))?;
    key.remark = if input.remark.trim().is_empty() {
        "未命名 Key".into()
    } else {
        input.remark.trim().into()
    };
    key.status = "untested".into();
    key.last_checked_at = None;
    let result = ApiKeySummary {
        id: key.id.clone(),
        provider_id: key.provider_id.clone(),
        remark: key.remark.clone(),
        masked_value: mask(next_value),
        status: key.status.clone(),
        last_checked_at: None,
    };

    if let Err(error) = save_data(&app, &data) {
        let _ = entry.set_password(&previous_value);
        return Err(error);
    }

    let _ = append_log(&app, "INFO", "api_key_replaced", "success");
    Ok(result)
}
#[tauri::command]
fn copy_api_key(app: tauri::AppHandle, key_id: String) -> Result<(), String> {
    let data = load_data(&app)?;
    let value = key_value(
        data.providers
            .iter()
            .flat_map(|p| &p.keys)
            .find(|k| k.id == key_id)
            .ok_or("未找到 API Key")?,
    )?;
    app.clipboard()
        .write_text(value)
        .map_err(|e| format!("无法写入剪贴板：{e}"))?;
    let _ = append_log(&app, "INFO", "api_key_copied", "success");
    Ok(())
}
#[tauri::command]
fn delete_api_key(app: tauri::AppHandle, key_id: String) -> Result<(), String> {
    let mut data = load_data(&app)?;
    let provider = data
        .providers
        .iter_mut()
        .find(|provider| provider.keys.iter().any(|key| key.id == key_id))
        .ok_or("未找到 API Key")?;
    let key_index = provider
        .keys
        .iter()
        .position(|key| key.id == key_id)
        .ok_or("未找到 API Key")?;
    let secret_id = provider.keys[key_index].secret_id.clone();
    keyring_entry(&secret_id)?
        .delete_credential()
        .map_err(|e| format!("无法从系统密钥库删除 API Key：{e}"))?;
    provider.keys.remove(key_index);
    save_data(&app, &data)?;
    let _ = append_log(&app, "INFO", "api_key_deleted", "success");
    Ok(())
}
#[tauri::command]
async fn check_provider_keys(
    app: tauri::AppHandle,
    provider_id: String,
) -> Result<Vec<ApiKeySummary>, String> {
    let mut data = load_data(&app)?;
    let provider = data
        .providers
        .iter_mut()
        .find(|p| p.id == provider_id)
        .ok_or("未找到供应商")?;
    let validation_provider_id = provider.id.clone();
    let client = validation_client()?;
    for key in &mut provider.keys {
        let value = key_value(key)?;
        key.status = validate_key(&client, &validation_provider_id, &value)
            .await
            .into();
        key.last_checked_at = Some(now());
    }
    let result = provider
        .keys
        .iter()
        .map(key_summary)
        .collect::<Result<Vec<_>, _>>()?;
    let valid_count = result.iter().filter(|key| key.status == "valid").count();
    let invalid_count = result.iter().filter(|key| key.status == "invalid").count();
    let error_count = result.iter().filter(|key| key.status == "error").count();
    save_data(&app, &data)?;
    let _ = append_log(
        &app,
        "INFO",
        "api_keys_checked",
        &format!(
            "total={} valid={} invalid={} error={}",
            result.len(),
            valid_count,
            invalid_count,
            error_count
        ),
    );
    Ok(result)
}

#[tauri::command]
async fn check_api_key(
    app: tauri::AppHandle,
    provider_id: String,
    key_id: String,
) -> Result<ApiKeySummary, String> {
    let mut data = load_data(&app)?;
    let provider = data
        .providers
        .iter_mut()
        .find(|provider| provider.id == provider_id)
        .ok_or("未找到供应商")?;
    let validation_provider_id = provider.id.clone();
    let key = provider
        .keys
        .iter_mut()
        .find(|key| key.id == key_id)
        .ok_or("未找到 API Key")?;
    let value = key_value(key)?;
    let client = validation_client()?;
    key.status = validate_key(&client, &validation_provider_id, &value)
        .await
        .into();
    key.last_checked_at = Some(now());
    let result = ApiKeySummary {
        id: key.id.clone(),
        provider_id: key.provider_id.clone(),
        remark: key.remark.clone(),
        masked_value: mask(&value),
        status: key.status.clone(),
        last_checked_at: key.last_checked_at.clone(),
    };
    save_data(&app, &data)?;
    let _ = append_log(
        &app,
        "INFO",
        "api_key_checked",
        &format!("status={}", result.status),
    );
    Ok(result)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init());
    if AUTOMATIC_UPDATES_ENABLED {
        builder = builder.plugin(tauri_plugin_updater::Builder::new().build());
    }
    builder
        .setup(|app| {
            let _ = append_log(app.handle(), "INFO", "application_started", "success");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_app_info,
            check_for_updates,
            install_update,
            open_data_directory,
            open_log_directory,
            clear_logs,
            list_providers,
            create_provider,
            update_provider,
            delete_provider,
            reorder_providers,
            create_api_key,
            update_api_key,
            copy_api_key,
            delete_api_key,
            check_api_key,
            check_provider_keys
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
