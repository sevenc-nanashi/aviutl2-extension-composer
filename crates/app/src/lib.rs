mod commands;
mod models;
mod store;
mod utils;
use utils::anyhow_to_string;

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
) -> Result<std::collections::BTreeMap<uuid::Uuid, store::IndexProfile>, String> {
    commands::list_profiles(&handle)
        .await
        .map_err(anyhow_to_string)
}

#[tauri::command]
async fn list_registries(
    handle: tauri::AppHandle,
) -> Result<std::collections::BTreeMap<uuid::Uuid, url::Url>, String> {
    commands::list_registries(&handle)
        .await
        .map_err(anyhow_to_string)
}

#[tauri::command]
async fn add_registry(handle: tauri::AppHandle, registry: String) -> Result<(), String> {
    let registry_url = utils::registry_or_url_to_url(&handle, &registry)
        .await
        .map_err(anyhow_to_string)?;
    commands::add_registry(&handle, registry_url)
        .await
        .map_err(anyhow_to_string)
}

#[tauri::command]
async fn fetch_registry(
    handle: tauri::AppHandle,
    registry: String,
) -> Result<models::Registry, String> {
    let registry_url = utils::registry_or_url_to_url(&handle, &registry)
        .await
        .map_err(anyhow_to_string)?;
    commands::fetch_registry(registry_url)
        .await
        .map_err(anyhow_to_string)
}

#[tauri::command]
async fn fetch_registry_cached(
    handle: tauri::AppHandle,
    registry: String,
) -> Result<models::Registry, String> {
    let registry_url = utils::registry_or_url_to_url(&handle, &registry)
        .await
        .map_err(anyhow_to_string)?;
    commands::fetch_registry_cached(registry_url).await
}

#[tauri::command]
async fn get_registry_url(handle: tauri::AppHandle, registry: String) -> Result<String, String> {
    let registry_url = utils::registry_or_url_to_url(&handle, &registry)
        .await
        .map_err(anyhow_to_string)?;
    Ok(registry_url.to_string())
}

#[tauri::command]
async fn get_profile_store(
    handle: tauri::AppHandle,
    profile_id: uuid::Uuid,
) -> Result<serde_json::Value, String> {
    commands::get_profile_store(&handle, profile_id)
        .await
        .map(|store| serde_json::to_value(&*store).unwrap())
        .map_err(anyhow_to_string)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            initialize_profile,
            list_profiles,
            list_registries,
            add_registry,
            fetch_registry,
            fetch_registry_cached,
            get_registry_url,
            get_profile_store,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
