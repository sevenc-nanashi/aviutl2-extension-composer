mod models;
mod commands;
mod store;

#[tauri::command]
async fn initialize_profile(
    handle: tauri::AppHandle,
    name: String,
    path: String,
    reinit: bool,
) -> Result<uuid::Uuid, String> {
    let path = std::path::PathBuf::from(path);
    commands::initialize_profile(&handle, name, path, reinit)
        .await
        .map_err(anyhow_to_string)
}

#[tauri::command]
async fn list_profiles(
    handle: tauri::AppHandle,
) -> Result<std::collections::HashMap<uuid::Uuid, store::IndexProfile>, String> {
    commands::list_profiles(&handle)
        .await
        .map_err(anyhow_to_string)
}

#[tauri::command]
async fn list_registries(
    handle: tauri::AppHandle,
) -> Result<std::collections::HashSet<url::Url>, String> {
    commands::list_registries(&handle)
        .await
        .map_err(anyhow_to_string)
}

fn anyhow_to_string(e: anyhow::Error) -> String {
    if e.to_string().starts_with('#') {
        return e.to_string();
    }
    format!("{e:#?}")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![initialize_profile, list_profiles, list_registries])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
