use serde::Serialize;
use tauri::Manager;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AppInfo {
    version: String,
    data_directory: String,
}

#[tauri::command]
fn get_app_info(app: tauri::AppHandle) -> Result<AppInfo, String> {
    let data_directory = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("无法解析应用数据目录：{error}"))?;

    Ok(AppInfo {
        version: app.package_info().version.to_string(),
        data_directory: data_directory.display().to_string(),
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_app_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
