mod commands;
mod fetch;
mod installer;
mod models;
mod path_match;
mod store;
mod utils;
use base64::{engine::general_purpose::STANDARD as base64, Engine as _};
use utils::anyhow_to_string;

#[tauri::command]
async fn initialize_profile(
    handle: tauri::AppHandle,
    name: String,
    path: String,
    on_exist: crate::commands::OnExist,
) -> Result<uuid::Uuid, String> {
    let path = std::path::PathBuf::from(path);
    commands::initialize_profile(&handle, name, path, on_exist)
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
async fn unregister_profile(
    handle: tauri::AppHandle,
    profile_id: uuid::Uuid,
) -> Result<(), String> {
    commands::unregister_profile(&handle, profile_id)
        .await
        .map_err(anyhow_to_string)
}

#[tauri::command]
async fn remove_profile(handle: tauri::AppHandle, profile_id: uuid::Uuid) -> Result<(), String> {
    commands::remove_profile(&handle, profile_id)
        .await
        .map_err(anyhow_to_string)
}

#[tauri::command]
async fn open_profile_folder(
    handle: tauri::AppHandle,
    profile_id: uuid::Uuid,
) -> Result<(), String> {
    commands::open_profile_folder(&handle, profile_id)
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
async fn remove_registry(handle: tauri::AppHandle, registry: uuid::Uuid) -> Result<(), String> {
    commands::remove_registry(&handle, registry)
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

#[tauri::command]
async fn fetch_manifest(
    handle: tauri::AppHandle,
    manifest_url: url::Url,
) -> Result<models::Manifest, String> {
    commands::fetch_manifest(&handle, manifest_url)
        .await
        .map_err(anyhow_to_string)
}

#[tauri::command]
async fn fetch_manifest_cached(
    handle: tauri::AppHandle,
    manifest_url: url::Url,
) -> Result<models::Manifest, String> {
    commands::fetch_manifest_cached(&handle, manifest_url).await
}

#[tauri::command]
async fn list_manifests(
    handle: tauri::AppHandle,
) -> Result<std::collections::BTreeMap<uuid::Uuid, url::Url>, String> {
    commands::list_manifests(&handle)
        .await
        .map_err(anyhow_to_string)
}

#[tauri::command]
async fn add_manifest_url(handle: tauri::AppHandle, manifest: url::Url) -> Result<(), String> {
    commands::add_manifest_url(&handle, manifest)
        .await
        .map_err(anyhow_to_string)
}

#[tauri::command]
async fn add_manifest_local(handle: tauri::AppHandle, file: String) -> Result<(), String> {
    let file = base64
        .decode(file)
        .map_err(|_| "#invalid_base64".to_string())?;
    commands::add_manifest_local(&handle, file)
        .await
        .map_err(anyhow_to_string)
}

#[tauri::command]
async fn remove_manifest(handle: tauri::AppHandle, manifest: uuid::Uuid) -> Result<(), String> {
    commands::remove_manifest(&handle, manifest)
        .await
        .map_err(anyhow_to_string)
}

#[tauri::command]
async fn plan_installation(
    handle: tauri::AppHandle,
    profile_id: uuid::Uuid,
    desired_manifests: Vec<crate::models::Manifest>,
) -> Result<installer::InstallPlan, String> {
    commands::plan_installation(&handle, profile_id, desired_manifests)
        .await
        .map_err(anyhow_to_string)
}

#[tauri::command]
async fn perform_installation(
    handle: tauri::AppHandle,
    profile_id: uuid::Uuid,
    plan: installer::InstallPlan,
    ch: tauri::ipc::Channel<(crate::models::ManifestId, crate::installer::InstallProgress)>,
) -> Result<(), String> {
    commands::perform_installation(&handle, profile_id, plan, ch)
        .await
        .map_err(anyhow_to_string)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            initialize_profile,
            list_profiles,
            unregister_profile,
            remove_profile,
            open_profile_folder,
            list_registries,
            add_registry,
            remove_registry,
            fetch_registry,
            fetch_registry_cached,
            get_registry_url,
            get_profile_store,
            fetch_manifest,
            fetch_manifest_cached,
            list_manifests,
            add_manifest_url,
            add_manifest_local,
            remove_manifest,
            plan_installation,
            perform_installation,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
