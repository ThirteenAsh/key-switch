use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tauri::Manager;
use tauri_plugin_clipboard_manager::ClipboardExt;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AppInfo {
    version: String,
    data_directory: String,
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

fn now() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .to_string()
}
const KEYRING_SERVICE: &str = "com.app.key-switch";
fn data_file(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    Ok(app
        .path()
        .app_data_dir()
        .map_err(|e| format!("无法解析应用数据目录：{e}"))?
        .join("key-switch-data.json"))
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

#[tauri::command]
fn get_app_info(app: tauri::AppHandle) -> Result<AppInfo, String> {
    Ok(AppInfo {
        version: app.package_info().version.to_string(),
        data_directory: app
            .path()
            .app_data_dir()
            .map_err(|e| format!("无法解析应用数据目录：{e}"))?
            .display()
            .to_string(),
    })
}
#[tauri::command]
fn open_data_directory(app: tauri::AppHandle) -> Result<(), String> {
    let directory = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("无法解析应用数据目录：{e}"))?;
    fs::create_dir_all(&directory).map_err(|e| format!("无法创建应用数据目录：{e}"))?;
    #[cfg(target_os = "windows")]
    let mut command = Command::new("explorer");
    #[cfg(target_os = "macos")]
    let mut command = Command::new("open");
    #[cfg(all(unix, not(target_os = "macos")))]
    let mut command = Command::new("xdg-open");
    command
        .arg(directory)
        .spawn()
        .map_err(|e| format!("无法打开应用数据目录：{e}"))?;
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
    for key in &provider.keys {
        keyring_entry(&key.secret_id)?
            .delete_credential()
            .map_err(|e| format!("无法从系统密钥库删除 API Key：{e}"))?;
    }
    data.providers.remove(provider_index);
    save_data(&app, &data)
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
    save_data(&app, &data)
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
        .map_err(|e| format!("无法写入剪贴板：{e}"))
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
    save_data(&app, &data)
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
    let url = provider
        .platform_url
        .clone()
        .ok_or("该供应商未配置可检测地址")?;
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| format!("无法初始化网络客户端：{e}"))?;
    for key in &mut provider.keys {
        key.status = match client.get(&url).bearer_auth(key_value(key)?).send().await {
            Ok(response)
                if response.status().is_success() || response.status().is_redirection() =>
            {
                "valid"
            }
            Ok(response)
                if response.status().as_u16() == 401 || response.status().as_u16() == 403 =>
            {
                "invalid"
            }
            Ok(_) | Err(_) => "error",
        }
        .into();
        key.last_checked_at = Some(now());
    }
    let result = provider
        .keys
        .iter()
        .map(key_summary)
        .collect::<Result<Vec<_>, _>>()?;
    save_data(&app, &data)?;
    Ok(result)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            get_app_info,
            open_data_directory,
            list_providers,
            create_provider,
            update_provider,
            delete_provider,
            reorder_providers,
            create_api_key,
            copy_api_key,
            delete_api_key,
            check_provider_keys
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
