use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fs::{self, OpenOptions},
    io::Write,
    net::{IpAddr, SocketAddr},
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CommandError {
    code: &'static str,
}

impl CommandError {
    fn from_message(message: &str) -> Self {
        Self {
            code: command_error_code(message),
        }
    }
}

impl From<String> for CommandError {
    fn from(message: String) -> Self {
        Self::from_message(&message)
    }
}

impl From<&str> for CommandError {
    fn from(message: &str) -> Self {
        Self::from_message(message)
    }
}

fn command_error_code(message: &str) -> &'static str {
    if message.starts_with("无法解析应用数据目录") {
        "DATA_DIRECTORY_UNAVAILABLE"
    } else if message.starts_with("无法创建日志目录")
        || message.starts_with("无法创建目录")
        || message.starts_with("无法创建应用数据目录")
    {
        "DIRECTORY_CREATE_FAILED"
    } else if message.starts_with("日志写入锁不可用")
        || message.starts_with("无法轮转")
        || message.starts_with("无法打开日志文件")
        || message.starts_with("无法写入日志")
    {
        "LOG_UNAVAILABLE"
    } else if message.starts_with("无法打开目录") {
        "DIRECTORY_OPEN_FAILED"
    } else if message.starts_with("无法读取本地数据") {
        "DATA_READ_FAILED"
    } else if message.starts_with("本地数据格式错误") {
        "DATA_INVALID"
    } else if message.starts_with("无法序列化本地数据") || message.starts_with("无法保存本地数据")
    {
        "DATA_SAVE_FAILED"
    } else if message.starts_with("系统密钥库不可用") {
        "KEYRING_UNAVAILABLE"
    } else if message.starts_with("无法读取系统密钥库") {
        "KEYRING_READ_FAILED"
    } else if message.starts_with("无法写入系统密钥库") {
        "KEYRING_WRITE_FAILED"
    } else if message.starts_with("无法从系统密钥库删除") {
        "KEYRING_DELETE_FAILED"
    } else if message.starts_with("无法初始化网络客户端") {
        "NETWORK_CLIENT_FAILED"
    } else if message.starts_with("当前应用版本不符合")
        || message.starts_with("更新版本不符合")
        || message.starts_with("签名更新清单中的版本不符合")
    {
        "VERSION_INVALID"
    } else if message.starts_with("GitHub Releases 响应过大") {
        "UPDATE_RESPONSE_TOO_LARGE"
    } else if message.starts_with("GitHub Releases 数据格式错误") {
        "UPDATE_DATA_INVALID"
    } else if message.starts_with("无法初始化更新检测客户端")
        || message.starts_with("无法连接 GitHub Releases")
        || message.starts_with("GitHub Releases 返回异常状态")
        || message.starts_with("无法读取 GitHub Releases 响应")
    {
        "UPDATE_CHECK_FAILED"
    } else if message.starts_with("更新版本标签无效") {
        "UPDATE_TAG_INVALID"
    } else if message.starts_with("无法生成更新清单地址")
        || message.starts_with("无法配置更新端点")
        || message.starts_with("无法初始化自动更新")
        || message.starts_with("无法读取签名更新清单")
    {
        "UPDATE_MANIFEST_FAILED"
    } else if message.starts_with("更新下载超时") {
        "UPDATE_DOWNLOAD_TIMEOUT"
    } else if message.starts_with("该 Release 没有可安装的更新") {
        "UPDATE_NOT_AVAILABLE"
    } else if message.starts_with("Release 标签与签名更新清单版本不一致") {
        "UPDATE_DATA_INVALID"
    } else if message.starts_with("更新下载或签名验证失败") {
        "UPDATE_DOWNLOAD_FAILED"
    } else if message.starts_with("更新安装失败") {
        "UPDATE_INSTALL_FAILED"
    } else if message.starts_with("无法清空日志") {
        "LOG_CLEAR_FAILED"
    } else if message.starts_with("供应商名称不能为空") {
        "PROVIDER_NAME_REQUIRED"
    } else if message.starts_with("供应商名称已存在") {
        "PROVIDER_NAME_EXISTS"
    } else if message.starts_with("供应商已存在") {
        "PROVIDER_EXISTS"
    } else if message.starts_with("未找到供应商") {
        "PROVIDER_NOT_FOUND"
    } else if message.starts_with("供应商排序数据不完整") {
        "PROVIDER_ORDER_INVALID"
    } else if message.starts_with("供应商检测配置无效") {
        "PROVIDER_VALIDATION_INVALID"
    } else if message.starts_with("API Key 不能为空") {
        "API_KEY_REQUIRED"
    } else if message.starts_with("未找到 API Key") {
        "API_KEY_NOT_FOUND"
    } else if message.starts_with("无法写入剪贴板") {
        "CLIPBOARD_WRITE_FAILED"
    } else if message.starts_with("无法读取设置文件") {
        "SETTINGS_READ_FAILED"
    } else if message.starts_with("设置文件格式错误")
        || message.starts_with("设置文件版本不受支持")
        || message.starts_with("语言设置无效")
    {
        "SETTINGS_INVALID"
    } else if message.starts_with("无法序列化设置文件")
        || message.starts_with("无法保存设置文件")
        || message.starts_with("无法写入设置临时文件")
        || message.starts_with("无法备份设置文件")
        || message.starts_with("无法替换设置文件")
        || message.starts_with("无法恢复设置文件")
    {
        "SETTINGS_SAVE_FAILED"
    } else {
        "UNKNOWN"
    }
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
    #[serde(default)]
    check_error_code: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mode")]
enum ValidationConfig {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "openai-compatible")]
    OpenAiCompatible {
        #[serde(rename = "baseUrl")]
        base_url: String,
    },
    #[serde(rename = "bearer")]
    Bearer { endpoint: String },
    #[serde(rename = "api-key-header")]
    ApiKeyHeader {
        endpoint: String,
        #[serde(rename = "headerName")]
        header_name: String,
    },
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self::None
    }
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
    #[serde(default)]
    validation: ValidationConfig,
    keys: Vec<ApiKeyRecord>,
}

#[derive(Default, Serialize, Deserialize)]
struct AppData {
    providers: Vec<ProviderRecord>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
// 仅保存非敏感应用偏好；API Key 等凭据仍必须保留在系统密钥库中。
struct AppSettings {
    schema_version: u32,
    locale_preference: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            schema_version: SETTINGS_SCHEMA_VERSION,
            locale_preference: "system".into(),
        }
    }
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
    check_error_code: Option<String>,
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
    validation: ValidationConfig,
    validation_supported: bool,
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
    #[serde(default)]
    validation: ValidationConfig,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateProviderInput {
    id: String,
    name: String,
    platform_url: Option<String>,
    #[serde(default)]
    validation: ValidationConfig,
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
const SETTINGS_FILE_NAME: &str = "settings.json";
const SETTINGS_BACKUP_FILE_NAME: &str = "settings.json.bak";
const SETTINGS_TEMP_FILE_NAME: &str = "settings.json.tmp";
const SETTINGS_SCHEMA_VERSION: u32 = 1;
const MAX_LOG_FILE_SIZE: u64 = 1024 * 1024;
const AUTOMATIC_UPDATES_ENABLED: bool = true;
static LOG_LOCK: Mutex<()> = Mutex::new(());
static SETTINGS_LOCK: Mutex<()> = Mutex::new(());

fn data_file(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    Ok(app
        .path()
        .app_data_dir()
        .map_err(|e| format!("无法解析应用数据目录：{e}"))?
        .join("key-switch-data.json"))
}

fn settings_file(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    Ok(app
        .path()
        .app_data_dir()
        .map_err(|e| format!("无法解析应用数据目录：{e}"))?
        .join(SETTINGS_FILE_NAME))
}

fn settings_backup_file(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    Ok(settings_file(app)?
        .parent()
        .ok_or("无法创建应用数据目录")?
        .join(SETTINGS_BACKUP_FILE_NAME))
}

fn valid_locale_preference(value: &str) -> bool {
    matches!(value, "system" | "zh-CN" | "zh-TW" | "en-US" | "ja-JP")
}

fn validate_settings(settings: &AppSettings) -> Result<(), String> {
    if settings.schema_version != SETTINGS_SCHEMA_VERSION {
        return Err(format!("设置文件版本不受支持：{}", settings.schema_version));
    }
    if !valid_locale_preference(&settings.locale_preference) {
        return Err("语言设置无效".into());
    }
    Ok(())
}

fn load_settings(app: &tauri::AppHandle) -> Result<AppSettings, String> {
    let file = settings_file(app)?;
    load_settings_from_file(&file)
}

fn load_settings_from_file(file: &PathBuf) -> Result<AppSettings, String> {
    let directory = file.parent().ok_or("无法创建应用数据目录")?;
    let backup = directory.join(SETTINGS_BACKUP_FILE_NAME);

    if !file.exists() && backup.exists() {
        let temporary = file
            .parent()
            .ok_or("无法创建应用数据目录")?
            .join(SETTINGS_TEMP_FILE_NAME);
        if temporary.exists() {
            fs::rename(&temporary, file).map_err(|e| format!("无法恢复设置文件：{e}"))?;
            let _ = fs::remove_file(&backup);
        } else {
            fs::rename(&backup, file).map_err(|e| format!("无法恢复设置文件：{e}"))?;
        }
    }

    let content = fs::read_to_string(file).map_err(|e| format!("无法读取设置文件：{e}"))?;
    let settings: AppSettings =
        serde_json::from_str(&content).map_err(|e| format!("设置文件格式错误：{e}"))?;
    validate_settings(&settings)?;
    Ok(settings)
}

fn save_settings(app: &tauri::AppHandle, settings: &AppSettings) -> Result<(), String> {
    let file = settings_file(app)?;
    save_settings_to_file(&file, settings)
}

fn save_settings_to_file(file: &PathBuf, settings: &AppSettings) -> Result<(), String> {
    validate_settings(settings)?;
    let directory = file.parent().ok_or("无法创建应用数据目录")?;
    fs::create_dir_all(directory).map_err(|e| format!("无法创建应用数据目录：{e}"))?;

    let serialized =
        serde_json::to_string_pretty(settings).map_err(|e| format!("无法序列化设置文件：{e}"))?;
    let temporary = directory.join(SETTINGS_TEMP_FILE_NAME);
    let backup = directory.join(SETTINGS_BACKUP_FILE_NAME);
    let mut output = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&temporary)
        .map_err(|e| format!("无法写入设置临时文件：{e}"))?;
    output
        .write_all(serialized.as_bytes())
        .and_then(|_| output.sync_all())
        .map_err(|e| format!("无法写入设置临时文件：{e}"))?;
    drop(output);

    if backup.exists() {
        fs::remove_file(&backup).map_err(|e| format!("无法备份设置文件：{e}"))?;
    }
    let had_existing_file = file.exists();
    if had_existing_file {
        fs::rename(file, &backup).map_err(|e| format!("无法备份设置文件：{e}"))?;
    }

    if let Err(error) = fs::rename(&temporary, file) {
        if had_existing_file && backup.exists() {
            fs::rename(&backup, file).map_err(|restore_error| {
                format!("无法恢复设置文件：{restore_error}；原始错误：{error}")
            })?;
        }
        let _ = fs::remove_file(&temporary);
        return Err(format!("无法替换设置文件：{error}"));
    }

    if backup.exists() {
        let _ = fs::remove_file(backup);
    }
    Ok(())
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
        check_error_code: key.check_error_code.clone(),
    })
}
fn summary(provider: &ProviderRecord) -> Result<ProviderSummary, String> {
    let validation_supported = key_validation_spec(provider).is_some();
    Ok(ProviderSummary {
        id: provider.id.clone(),
        name: provider.name.clone(),
        abbreviation: provider.abbreviation.clone(),
        tone: provider.tone.clone(),
        logo: provider.logo.clone(),
        kind: provider.kind.clone(),
        platform_url: provider.platform_url.clone(),
        validation: provider.validation.clone(),
        validation_supported,
        keys: provider
            .keys
            .iter()
            .map(|key| {
                let mut result = key_summary(key)?;
                if !validation_supported {
                    result.status = "unsupported".into();
                    result.last_checked_at = None;
                    result.check_error_code = None;
                }
                Ok::<ApiKeySummary, String>(result)
            })
            .collect::<Result<Vec<_>, _>>()?,
    })
}

enum KeyValidationSpec {
    Bearer(String),
    ApiKeyHeader { url: String, header_name: String },
    Anthropic,
}

impl KeyValidationSpec {
    fn endpoint(&self) -> &str {
        match self {
            Self::Bearer(url) | Self::ApiKeyHeader { url, .. } => url,
            Self::Anthropic => "https://api.anthropic.com/v1/models?limit=1",
        }
    }
}

fn key_validation_spec(provider: &ProviderRecord) -> Option<KeyValidationSpec> {
    let builtin = match provider.id.as_str() {
        "openai" => Some(KeyValidationSpec::Bearer(
            "https://api.openai.com/v1/models".into(),
        )),
        "claude" | "anthropic" => Some(KeyValidationSpec::Anthropic),
        "gemini" | "aistudio" => Some(KeyValidationSpec::ApiKeyHeader {
            url: "https://generativelanguage.googleapis.com/v1beta/models?pageSize=1".into(),
            header_name: "x-goog-api-key".into(),
        }),
        "deepseek" => Some(KeyValidationSpec::Bearer(
            "https://api.deepseek.com/models".into(),
        )),
        "mimo" => Some(KeyValidationSpec::ApiKeyHeader {
            url: "https://api.xiaomimimo.com/v1/models".into(),
            header_name: "api-key".into(),
        }),
        "qwen" => Some(KeyValidationSpec::Bearer(
            "https://dashscope.aliyuncs.com/api/v1/deployments?page_no=1&page_size=1".into(),
        )),
        "kimi" => Some(KeyValidationSpec::Bearer(
            "https://api.moonshot.cn/v1/models".into(),
        )),
        "grok" => Some(KeyValidationSpec::Bearer(
            "https://api.x.ai/v1/models".into(),
        )),
        "openrouter" => Some(KeyValidationSpec::Bearer(
            "https://openrouter.ai/api/v1/key".into(),
        )),
        "minimax" => Some(KeyValidationSpec::Bearer(
            "https://api.minimaxi.com/v1/models".into(),
        )),
        "doubao" => Some(KeyValidationSpec::Bearer(
            "https://ark.cn-beijing.volces.com/ping".into(),
        )),
        "hunyuan" => Some(KeyValidationSpec::Bearer(
            "https://tokenhub.tencentmaas.com/v1/models".into(),
        )),
        "qianfan" => Some(KeyValidationSpec::Bearer(
            "https://qianfan.baidubce.com/v2/models".into(),
        )),
        "zhipu" => Some(KeyValidationSpec::Bearer(
            "https://open.bigmodel.cn/api/paas/v4/files".into(),
        )),
        _ => None,
    };
    if provider.kind == "builtin" {
        return builtin;
    }

    match &provider.validation {
        ValidationConfig::None => None,
        ValidationConfig::OpenAiCompatible { base_url } => Some(KeyValidationSpec::Bearer(
            format!("{}/models", base_url.trim_end_matches('/')),
        )),
        ValidationConfig::Bearer { endpoint } => Some(KeyValidationSpec::Bearer(endpoint.clone())),
        ValidationConfig::ApiKeyHeader {
            endpoint,
            header_name,
        } => Some(KeyValidationSpec::ApiKeyHeader {
            url: endpoint.clone(),
            header_name: header_name.clone(),
        }),
    }
}

#[derive(Debug, PartialEq)]
struct ValidationOutcome {
    status: &'static str,
    error_code: Option<&'static str>,
}

fn classify_validation_status(status: reqwest::StatusCode) -> ValidationOutcome {
    if status.is_success() {
        ValidationOutcome {
            status: "valid",
            error_code: None,
        }
    } else if status == reqwest::StatusCode::BAD_REQUEST
        || status == reqwest::StatusCode::UNAUTHORIZED
        || status == reqwest::StatusCode::FORBIDDEN
    {
        ValidationOutcome {
            status: "invalid",
            error_code: None,
        }
    } else if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
        ValidationOutcome {
            status: "error",
            error_code: Some("rateLimited"),
        }
    } else if status.is_server_error() {
        ValidationOutcome {
            status: "error",
            error_code: Some("serverUnavailable"),
        }
    } else if status.is_redirection() {
        ValidationOutcome {
            status: "error",
            error_code: Some("endpointInvalid"),
        }
    } else {
        ValidationOutcome {
            status: "error",
            error_code: Some("unexpectedStatus"),
        }
    }
}

fn is_blocked_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => {
            let [first, second, ..] = ip.octets();
            ip.is_private()
                || ip.is_loopback()
                || ip.is_link_local()
                || ip.is_broadcast()
                || ip.is_documentation()
                || ip.is_unspecified()
                || ip.is_multicast()
                || first == 0
                || (first == 100 && (64..=127).contains(&second))
                || (first == 198 && (18..=19).contains(&second))
                || first >= 240
        }
        IpAddr::V6(ip) => {
            let segments = ip.segments();
            ip.is_loopback()
                || ip.is_unspecified()
                || ip.is_unique_local()
                || ip.is_unicast_link_local()
                || ip.is_multicast()
                || (segments[0] == 0x2001 && segments[1] == 0x0db8)
                || ip
                    .to_ipv4()
                    .is_some_and(|mapped| is_blocked_ip(IpAddr::V4(mapped)))
        }
    }
}

fn parse_safe_validation_url(value: &str) -> Result<reqwest::Url, String> {
    if value.len() > 2048 {
        return Err("供应商检测配置无效：检测地址过长".into());
    }
    let url = reqwest::Url::parse(value)
        .map_err(|_| "供应商检测配置无效：检测地址格式错误".to_string())?;
    if url.scheme() != "https" {
        return Err("供应商检测配置无效：检测地址必须使用 HTTPS".into());
    }
    if !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err("供应商检测配置无效：检测地址不能包含凭据、查询参数或片段".into());
    }
    let host = url
        .host_str()
        .ok_or_else(|| "供应商检测配置无效：检测地址缺少域名".to_string())?;
    let lower_host = host.to_ascii_lowercase();
    if lower_host == "localhost"
        || lower_host.ends_with(".localhost")
        || lower_host.ends_with(".local")
        || lower_host.ends_with(".internal")
        || host.parse::<IpAddr>().is_ok_and(is_blocked_ip)
    {
        return Err("供应商检测配置无效：不允许本地或私有网络地址".into());
    }
    Ok(url)
}

fn valid_header_name(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    !value.is_empty()
        && value.len() <= 64
        && reqwest::header::HeaderName::from_bytes(value.as_bytes()).is_ok()
        && !matches!(
            lower.as_str(),
            "authorization"
                | "cookie"
                | "content-length"
                | "host"
                | "proxy-authorization"
                | "set-cookie"
        )
}

fn normalize_validation_config(
    kind: &str,
    validation: ValidationConfig,
) -> Result<ValidationConfig, String> {
    if kind != "custom" {
        return Ok(ValidationConfig::None);
    }
    match validation {
        ValidationConfig::None => Ok(ValidationConfig::None),
        ValidationConfig::OpenAiCompatible { base_url } => {
            let normalized = base_url.trim().trim_end_matches('/').to_string();
            parse_safe_validation_url(&normalized)?;
            Ok(ValidationConfig::OpenAiCompatible {
                base_url: normalized,
            })
        }
        ValidationConfig::Bearer { endpoint } => {
            let normalized = endpoint.trim().to_string();
            parse_safe_validation_url(&normalized)?;
            Ok(ValidationConfig::Bearer {
                endpoint: normalized,
            })
        }
        ValidationConfig::ApiKeyHeader {
            endpoint,
            header_name,
        } => {
            let normalized_endpoint = endpoint.trim().to_string();
            let normalized_header = header_name.trim().to_string();
            parse_safe_validation_url(&normalized_endpoint)?;
            if !valid_header_name(&normalized_header) {
                return Err("供应商检测配置无效：请求头名称无效或不受允许".into());
            }
            Ok(ValidationConfig::ApiKeyHeader {
                endpoint: normalized_endpoint,
                header_name: normalized_header,
            })
        }
    }
}

struct ResolvedValidationEndpoint {
    host: String,
    addresses: Vec<SocketAddr>,
}

async fn resolve_validation_endpoint(
    spec: &KeyValidationSpec,
) -> Result<ResolvedValidationEndpoint, &'static str> {
    let url = reqwest::Url::parse(spec.endpoint()).map_err(|_| "endpointInvalid")?;
    let host = url.host_str().ok_or("endpointInvalid")?;
    if host.parse::<IpAddr>().is_ok_and(is_blocked_ip) {
        return Err("endpointInvalid");
    }
    let port = url.port_or_known_default().ok_or("endpointInvalid")?;
    let addresses = tokio::net::lookup_host((host, port))
        .await
        .map_err(|_| "network")?;
    let mut safe_addresses = Vec::new();
    for address in addresses {
        if is_blocked_ip(address.ip()) {
            return Err("endpointInvalid");
        }
        safe_addresses.push(address);
    }
    if safe_addresses.is_empty() {
        return Err("network");
    }
    Ok(ResolvedValidationEndpoint {
        host: host.into(),
        addresses: safe_addresses,
    })
}

fn validation_client(resolved: &ResolvedValidationEndpoint) -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .connect_timeout(Duration::from_secs(5))
        .redirect(reqwest::redirect::Policy::none())
        .user_agent("Key-Switch/1.0.1")
        .resolve_to_addrs(&resolved.host, &resolved.addresses)
        .build()
        .map_err(|e| format!("无法初始化网络客户端：{e}"))
}

fn build_validation_request(
    client: &reqwest::Client,
    spec: &KeyValidationSpec,
    value: &str,
) -> Result<reqwest::Request, ()> {
    let request = match spec {
        KeyValidationSpec::Bearer(url) => client.get(url).bearer_auth(value),
        KeyValidationSpec::ApiKeyHeader { url, header_name } => {
            let header =
                reqwest::header::HeaderName::from_bytes(header_name.as_bytes()).map_err(|_| ())?;
            client.get(url).header(header, value)
        }
        KeyValidationSpec::Anthropic => client
            .get("https://api.anthropic.com/v1/models?limit=1")
            .header("x-api-key", value)
            .header("anthropic-version", "2023-06-01"),
    };
    request.build().map_err(|_| ())
}

async fn validate_key(
    client: &reqwest::Client,
    spec: &KeyValidationSpec,
    value: &str,
) -> ValidationOutcome {
    let request = match build_validation_request(client, spec, value) {
        Ok(request) => request,
        Err(()) => {
            return ValidationOutcome {
                status: "error",
                error_code: Some("endpointInvalid"),
            }
        }
    };

    match client.execute(request).await {
        Ok(response) => classify_validation_status(response.status()),
        Err(error) => ValidationOutcome {
            status: "error",
            error_code: Some(if error.is_timeout() {
                "timeout"
            } else {
                "network"
            }),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{
        build_validation_request, classify_validation_status, command_error_code,
        load_settings_from_file, normalize_validation_config, parse_safe_validation_url,
        save_settings_to_file, valid_header_name, valid_locale_preference, AppData, AppSettings,
        KeyValidationSpec, SemVersion, ValidationConfig, ValidationOutcome,
        SETTINGS_BACKUP_FILE_NAME, SETTINGS_FILE_NAME, SETTINGS_SCHEMA_VERSION,
        SETTINGS_TEMP_FILE_NAME,
    };
    use reqwest::StatusCode;
    use std::{fs, process};

    #[test]
    fn classifies_key_validation_responses() {
        assert_eq!(
            classify_validation_status(StatusCode::OK),
            ValidationOutcome {
                status: "valid",
                error_code: None
            }
        );
        assert_eq!(
            classify_validation_status(StatusCode::UNAUTHORIZED),
            ValidationOutcome {
                status: "invalid",
                error_code: None
            }
        );
        assert_eq!(
            classify_validation_status(StatusCode::FORBIDDEN),
            ValidationOutcome {
                status: "invalid",
                error_code: None
            }
        );
        assert_eq!(
            classify_validation_status(StatusCode::TOO_MANY_REQUESTS),
            ValidationOutcome {
                status: "error",
                error_code: Some("rateLimited")
            }
        );
        assert_eq!(
            classify_validation_status(StatusCode::INTERNAL_SERVER_ERROR),
            ValidationOutcome {
                status: "error",
                error_code: Some("serverUnavailable")
            }
        );
    }

    #[test]
    fn keeps_old_provider_data_compatible() {
        let data: AppData = serde_json::from_str(
            r#"{"providers":[{"id":"custom-old","name":"Old","abbreviation":"OL","tone":"gray","logo":null,"kind":"custom","platformUrl":"https://example.com","keys":[{"id":"key-1","providerId":"custom-old","remark":"","secretId":"key-1","status":"untested","lastCheckedAt":null}]}]}"#,
        )
        .unwrap();
        assert_eq!(data.providers[0].validation, ValidationConfig::None);
        assert_eq!(data.providers[0].keys[0].check_error_code, None);
    }

    #[test]
    fn validates_custom_provider_check_configuration() {
        assert!(parse_safe_validation_url("https://api.example.com/v1/models").is_ok());
        for unsafe_url in [
            "http://api.example.com/v1/models",
            "https://localhost/v1/models",
            "https://127.0.0.1/v1/models",
            "https://10.0.0.1/v1/models",
            "https://user:pass@example.com/v1/models",
            "https://api.example.com/v1/models?key=value",
        ] {
            assert!(
                parse_safe_validation_url(unsafe_url).is_err(),
                "{unsafe_url}"
            );
        }
        assert!(valid_header_name("x-api-key"));
        assert!(!valid_header_name("Authorization"));
        assert!(!valid_header_name("bad header"));

        assert_eq!(
            normalize_validation_config(
                "custom",
                ValidationConfig::OpenAiCompatible {
                    base_url: " https://api.example.com/v1/ ".into(),
                },
            )
            .unwrap(),
            ValidationConfig::OpenAiCompatible {
                base_url: "https://api.example.com/v1".into(),
            }
        );
    }

    #[test]
    fn builds_supported_key_authentication_requests() {
        let client = reqwest::Client::new();
        let bearer = build_validation_request(
            &client,
            &KeyValidationSpec::Bearer("https://api.example.com/v1/models".into()),
            "test-key",
        )
        .unwrap();
        assert_eq!(bearer.headers()["authorization"], "Bearer test-key");

        let header = build_validation_request(
            &client,
            &KeyValidationSpec::ApiKeyHeader {
                url: "https://api.example.com/v1/models".into(),
                header_name: "x-api-key".into(),
            },
            "test-key",
        )
        .unwrap();
        assert_eq!(header.headers()["x-api-key"], "test-key");
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

    #[test]
    fn maps_internal_errors_to_stable_command_codes() {
        assert_eq!(
            command_error_code("更新下载超时，请检查网络后重试"),
            "UPDATE_DOWNLOAD_TIMEOUT"
        );
        assert_eq!(
            command_error_code("供应商名称已存在"),
            "PROVIDER_NAME_EXISTS"
        );
        assert_eq!(
            command_error_code("无法保存设置文件：disk full"),
            "SETTINGS_SAVE_FAILED"
        );
        assert_eq!(
            command_error_code("供应商检测配置无效：检测地址必须使用 HTTPS"),
            "PROVIDER_VALIDATION_INVALID"
        );
        assert_eq!(command_error_code("unexpected"), "UNKNOWN");
    }

    #[test]
    fn validates_supported_locale_preferences() {
        for locale in ["system", "zh-CN", "zh-TW", "en-US", "ja-JP"] {
            assert!(valid_locale_preference(locale));
        }
        assert!(!valid_locale_preference("fr-FR"));
    }

    #[test]
    fn fills_missing_settings_fields_with_schema_defaults() {
        let settings: AppSettings =
            serde_json::from_str(r#"{"localePreference":"en-US"}"#).unwrap();
        assert_eq!(settings.schema_version, SETTINGS_SCHEMA_VERSION);
        assert_eq!(settings.locale_preference, "en-US");
    }

    #[test]
    fn persists_settings_and_recovers_an_interrupted_replacement() {
        let directory = std::env::temp_dir().join(format!(
            "key-switch-settings-test-{}-{}",
            process::id(),
            super::now()
        ));
        fs::create_dir_all(&directory).unwrap();
        let file = directory.join(SETTINGS_FILE_NAME);
        let initial = AppSettings {
            schema_version: SETTINGS_SCHEMA_VERSION,
            locale_preference: "en-US".into(),
        };
        save_settings_to_file(&file, &initial).unwrap();
        assert_eq!(
            load_settings_from_file(&file).unwrap().locale_preference,
            "en-US"
        );

        let replacement = AppSettings {
            schema_version: SETTINGS_SCHEMA_VERSION,
            locale_preference: "ja-JP".into(),
        };
        fs::write(
            directory.join(SETTINGS_TEMP_FILE_NAME),
            serde_json::to_string_pretty(&replacement).unwrap(),
        )
        .unwrap();
        fs::rename(&file, directory.join(SETTINGS_BACKUP_FILE_NAME)).unwrap();
        assert_eq!(
            load_settings_from_file(&file).unwrap().locale_preference,
            "ja-JP"
        );

        fs::remove_dir_all(directory).unwrap();
    }
}

#[tauri::command]
fn load_app_settings(
    app: tauri::AppHandle,
    legacy_locale_preference: Option<String>,
) -> Result<AppSettings, CommandError> {
    let _guard = SETTINGS_LOCK
        .lock()
        .map_err(|_| CommandError::from("无法读取设置文件：设置锁不可用"))?;
    let file = settings_file(&app)?;
    let backup = settings_backup_file(&app)?;

    let settings = if file.exists() || backup.exists() {
        load_settings(&app)?
    } else {
        let mut settings = AppSettings::default();
        if let Some(preference) = legacy_locale_preference {
            if valid_locale_preference(&preference) {
                settings.locale_preference = preference;
            }
        }
        save_settings(&app, &settings)?;
        let _ = append_log(&app, "INFO", "settings_initialized", "success");
        settings
    };

    Ok(settings)
}

#[tauri::command]
fn save_app_settings(
    app: tauri::AppHandle,
    settings: AppSettings,
) -> Result<AppSettings, CommandError> {
    let _guard = SETTINGS_LOCK
        .lock()
        .map_err(|_| CommandError::from("无法写入设置临时文件：设置锁不可用"))?;
    save_settings(&app, &settings)?;
    let _ = append_log(&app, "INFO", "settings_saved", "success");
    Ok(settings)
}

#[tauri::command]
fn get_app_info(app: tauri::AppHandle) -> Result<AppInfo, CommandError> {
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
async fn check_for_updates(app: tauri::AppHandle) -> Result<Option<UpdateInfo>, CommandError> {
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
        .user_agent("Key-Switch-Update-Check/1.0.1")
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
        return Err(format!("GitHub Releases 返回异常状态：{}", response.status()).into());
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
async fn install_update(app: tauri::AppHandle, release_tag: String) -> Result<(), CommandError> {
    const RELEASE_TAG_PREFIX: &str = "v";
    const LEGACY_RELEASE_TAG_PREFIX: &str = "app-v";
    const DOWNLOAD_TIMEOUT: Duration = Duration::from_secs(90);

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
        // 单个 HTTP 请求的上限略高于业务层总超时，由业务层统一返回可识别的超时错误。
        .timeout(Duration::from_secs(120))
        .build()
        .map_err(|e| format!("无法初始化自动更新：{e}"))?;
    let update = tokio::time::timeout(DOWNLOAD_TIMEOUT, updater.check())
        .await
        .map_err(|_| {
            let _ = append_log(
                &app,
                "WARN",
                "update_manifest_timeout",
                "timeout_seconds=90",
            );
            "更新下载超时，请检查网络后重试".to_string()
        })?
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
    let update_bytes = tokio::time::timeout(DOWNLOAD_TIMEOUT, update.download(|_, _| {}, || {}))
        .await
        .map_err(|_| {
            let _ = append_log(
                &app,
                "WARN",
                "update_download_timeout",
                "timeout_seconds=90",
            );
            "更新下载超时，请检查网络后重试".to_string()
        })?
        .map_err(|e| format!("更新下载或签名验证失败：{e}"))?;
    update
        .install(update_bytes)
        .map_err(|e| format!("更新安装失败：{e}"))?;
    let _ = append_log(&app, "INFO", "update_installed", "restart=pending");
    app.restart();
}
#[tauri::command]
fn open_data_directory(app: tauri::AppHandle) -> Result<(), CommandError> {
    let directory = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("无法解析应用数据目录：{e}"))?;
    open_directory(directory)?;
    let _ = append_log(&app, "INFO", "data_directory_opened", "success");
    Ok(())
}

#[tauri::command]
fn open_log_directory(app: tauri::AppHandle) -> Result<(), CommandError> {
    open_directory(log_directory(&app)?)?;
    let _ = append_log(&app, "INFO", "log_directory_opened", "success");
    Ok(())
}

#[tauri::command]
fn clear_logs(app: tauri::AppHandle) -> Result<(), CommandError> {
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
fn list_providers(app: tauri::AppHandle) -> Result<Vec<ProviderSummary>, CommandError> {
    load_data(&app)?
        .providers
        .iter()
        .map(|provider| summary(provider).map_err(CommandError::from))
        .collect()
}
#[tauri::command]
fn create_provider(
    app: tauri::AppHandle,
    input: CreateProviderInput,
) -> Result<ProviderSummary, CommandError> {
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
    let validation = normalize_validation_config(&input.kind, input.validation)?;
    let provider = ProviderRecord {
        id: input.id,
        name: input.name.trim().into(),
        abbreviation: input.abbreviation,
        tone: input.tone,
        logo: input.logo,
        kind: input.kind,
        platform_url: input.platform_url.filter(|u| !u.trim().is_empty()),
        validation,
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
) -> Result<ProviderSummary, CommandError> {
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
    let next_validation = normalize_validation_config(&provider.kind, input.validation)?;
    let validation_changed = provider.validation != next_validation;
    provider.name = input.name.trim().into();
    provider.platform_url = input.platform_url.filter(|u| !u.trim().is_empty());
    provider.validation = next_validation;
    if validation_changed {
        let validation_supported = key_validation_spec(provider).is_some();
        for key in &mut provider.keys {
            key.status = if validation_supported {
                "untested".into()
            } else {
                "unsupported".into()
            };
            key.last_checked_at = None;
            key.check_error_code = None;
        }
    }
    let result = summary(provider)?;
    save_data(&app, &data)?;
    let _ = append_log(&app, "INFO", "provider_updated", "success");
    Ok(result)
}
#[tauri::command]
fn delete_provider(app: tauri::AppHandle, provider_id: String) -> Result<(), CommandError> {
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
fn reorder_providers(app: tauri::AppHandle, provider_ids: Vec<String>) -> Result<(), CommandError> {
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
fn create_api_key(
    app: tauri::AppHandle,
    input: CreateKeyInput,
) -> Result<ApiKeySummary, CommandError> {
    if input.value.trim().is_empty() {
        return Err("API Key 不能为空".into());
    }
    let mut data = load_data(&app)?;
    let provider = data
        .providers
        .iter_mut()
        .find(|p| p.id == input.provider_id)
        .ok_or("未找到供应商")?;
    let validation_supported = key_validation_spec(provider).is_some();
    let id = format!("key-{}", now());
    keyring_entry(&id)?
        .set_password(input.value.trim())
        .map_err(|e| format!("无法写入系统密钥库：{e}"))?;
    let key = ApiKeyRecord {
        id: id.clone(),
        provider_id: input.provider_id,
        remark: input.remark.trim().into(),
        secret_id: id,
        status: if validation_supported {
            "untested"
        } else {
            "unsupported"
        }
        .into(),
        last_checked_at: None,
        check_error_code: None,
    };
    let result = key_summary(&key)?;
    provider.keys.push(key);
    save_data(&app, &data)?;
    let _ = append_log(&app, "INFO", "api_key_created", "success");
    Ok(result)
}

#[tauri::command]
fn update_api_key(
    app: tauri::AppHandle,
    input: UpdateKeyInput,
) -> Result<ApiKeySummary, CommandError> {
    let mut data = load_data(&app)?;
    let key = data
        .providers
        .iter_mut()
        .flat_map(|provider| &mut provider.keys)
        .find(|key| key.id == input.id)
        .ok_or("未找到 API Key")?;
    let next_remark = input.remark.trim().to_string();
    let next_value = input.value.trim();

    if next_value.is_empty() {
        key.remark = next_remark;
        let result = key_summary(key)?;
        save_data(&app, &data)?;
        let _ = append_log(&app, "INFO", "api_key_remark_updated", "success");
        return Ok(result);
    }

    let entry = keyring_entry(&key.secret_id)?;
    let previous_value = entry
        .get_password()
        .map_err(|e| format!("无法读取系统密钥库中的 API Key：{e}"))?;

    entry
        .set_password(next_value)
        .map_err(|e| format!("无法写入系统密钥库：{e}"))?;
    key.remark = next_remark;
    key.status = "untested".into();
    key.last_checked_at = None;
    key.check_error_code = None;
    let result = ApiKeySummary {
        id: key.id.clone(),
        provider_id: key.provider_id.clone(),
        remark: key.remark.clone(),
        masked_value: mask(next_value),
        status: key.status.clone(),
        last_checked_at: None,
        check_error_code: None,
    };

    if let Err(error) = save_data(&app, &data) {
        let _ = entry.set_password(&previous_value);
        return Err(error.into());
    }

    let _ = append_log(&app, "INFO", "api_key_replaced", "success");
    Ok(result)
}
#[tauri::command]
fn copy_api_key(app: tauri::AppHandle, key_id: String) -> Result<(), CommandError> {
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
fn delete_api_key(app: tauri::AppHandle, key_id: String) -> Result<(), CommandError> {
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
) -> Result<Vec<ApiKeySummary>, CommandError> {
    let mut data = load_data(&app)?;
    let provider = data
        .providers
        .iter_mut()
        .find(|p| p.id == provider_id)
        .ok_or("未找到供应商")?;
    let Some(validation_spec) = key_validation_spec(provider) else {
        for key in &mut provider.keys {
            key.status = "unsupported".into();
            key.last_checked_at = None;
            key.check_error_code = None;
        }
        let result = provider
            .keys
            .iter()
            .map(key_summary)
            .collect::<Result<Vec<_>, _>>()?;
        save_data(&app, &data)?;
        return Ok(result);
    };
    let resolved = match resolve_validation_endpoint(&validation_spec).await {
        Ok(resolved) => resolved,
        Err(error_code) => {
            for key in &mut provider.keys {
                key.status = "error".into();
                key.last_checked_at = Some(now());
                key.check_error_code = Some(error_code.into());
            }
            let result = provider
                .keys
                .iter()
                .map(key_summary)
                .collect::<Result<Vec<_>, _>>()?;
            save_data(&app, &data)?;
            return Ok(result);
        }
    };
    let client = validation_client(&resolved)?;
    for key in &mut provider.keys {
        let value = key_value(key)?;
        let outcome = validate_key(&client, &validation_spec, &value).await;
        key.status = outcome.status.into();
        key.check_error_code = outcome.error_code.map(str::to_string);
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
) -> Result<ApiKeySummary, CommandError> {
    let mut data = load_data(&app)?;
    let provider = data
        .providers
        .iter_mut()
        .find(|provider| provider.id == provider_id)
        .ok_or("未找到供应商")?;
    let validation_spec = key_validation_spec(provider);
    let key = provider
        .keys
        .iter_mut()
        .find(|key| key.id == key_id)
        .ok_or("未找到 API Key")?;
    let Some(validation_spec) = validation_spec else {
        key.status = "unsupported".into();
        key.last_checked_at = None;
        key.check_error_code = None;
        let result = key_summary(key)?;
        save_data(&app, &data)?;
        return Ok(result);
    };
    let resolved = match resolve_validation_endpoint(&validation_spec).await {
        Ok(resolved) => resolved,
        Err(error_code) => {
            key.status = "error".into();
            key.last_checked_at = Some(now());
            key.check_error_code = Some(error_code.into());
            let result = key_summary(key)?;
            save_data(&app, &data)?;
            return Ok(result);
        }
    };
    let value = key_value(key)?;
    let client = validation_client(&resolved)?;
    let outcome = validate_key(&client, &validation_spec, &value).await;
    key.status = outcome.status.into();
    key.check_error_code = outcome.error_code.map(str::to_string);
    key.last_checked_at = Some(now());
    let result = ApiKeySummary {
        id: key.id.clone(),
        provider_id: key.provider_id.clone(),
        remark: key.remark.clone(),
        masked_value: mask(&value),
        status: key.status.clone(),
        last_checked_at: key.last_checked_at.clone(),
        check_error_code: key.check_error_code.clone(),
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
            load_app_settings,
            save_app_settings,
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
