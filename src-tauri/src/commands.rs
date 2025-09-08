use tauri::Manager;

use crate::store::open_store;

fn index_store_path(app: &tauri::AppHandle) -> anyhow::Result<std::path::PathBuf> {
    Ok(app.path().app_data_dir()?.join("index_store.json"))
}

pub fn initialize_profile(
    app: &tauri::AppHandle,
    name: String,
    path: std::path::PathBuf,
    force: bool,
) -> anyhow::Result<uuid::Uuid> {
    anyhow::ensure!(!name.is_empty(), "名前が空です");
    anyhow::ensure!(path.exists(), "パスが存在しません");
    anyhow::ensure!(path.is_dir(), "パスがフォルダではありません");
    log::info!("initialize profile: {}, {:?}", name, path);

    let id = uuid::Uuid::new_v4();

    let store_path = path.join("store.json");
    if store_path.exists() {
        anyhow::bail!("#exists");
    }

    let mut content_store = open_store::<crate::store::ContentStore>(&store_path)?;
    content_store.name = name.clone();

    let index_store = index_store_path(app)?;
    let mut index_store = crate::store::open_store::<crate::store::IndexStore>(&index_store)?;
    index_store.profiles.insert(
        id,
        crate::store::IndexProfile {
            name: name.clone(),
            path: path.clone(),
        },
    );
    crate::save_all!(index_store, content_store)?;
    log::info!("profile initialized: {}, {:?}", name, path);

    Ok(id)
}

pub fn list_profiles(
    app: &tauri::AppHandle,
) -> anyhow::Result<std::collections::HashMap<uuid::Uuid, crate::store::IndexProfile>> {
    let index_store_path = index_store_path(app)?;
    let index_store = crate::store::open_store::<crate::store::IndexStore>(&index_store_path)?;

    Ok(index_store.profiles.clone())
}
