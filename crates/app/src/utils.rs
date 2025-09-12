use tauri::Manager;
pub fn anyhow_to_string(e: anyhow::Error) -> String {
    if e.to_string().starts_with('#') {
        return e.to_string();
    }
    format!("{e:#?}")
}

pub fn index_store_path(app: &tauri::AppHandle) -> anyhow::Result<std::path::PathBuf> {
    Ok(app.path().app_data_dir()?.join("index_store.json"))
}
pub async fn open_index_store(
    app: &tauri::AppHandle,
) -> anyhow::Result<crate::store::LockedStore<crate::store::IndexStore>> {
    let index_store_path = index_store_path(app)?;
    crate::store::open_store::<crate::store::IndexStore>(&index_store_path).await
}

pub async fn registry_or_url_to_url(
    handle: &tauri::AppHandle,
    registry_or_url: &str,
) -> anyhow::Result<url::Url> {
    if let Ok(url) = url::Url::parse(registry_or_url) {
        Ok(url)
    } else {
        let uuid = uuid::Uuid::parse_str(registry_or_url)
            .map_err(|_| anyhow::anyhow!("#invalid_registry_or_url"))?;
        let store = open_index_store(handle).await?;
        store
            .registries
            .get(&uuid)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("#registry_not_found"))
    }
}
