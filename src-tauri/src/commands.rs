use tauri::Manager;

use crate::store::open_store;

fn index_store_path(app: &tauri::AppHandle) -> anyhow::Result<std::path::PathBuf> {
    Ok(app.path().app_data_dir()?.join("index_store.json"))
}

pub async fn initialize_profile(
    app: &tauri::AppHandle,
    name: String,
    path: std::path::PathBuf,
    reinit: bool,
) -> anyhow::Result<uuid::Uuid> {
    anyhow::ensure!(!name.is_empty(), "#name_empty");
    anyhow::ensure!(path.exists(), "#not_exists");
    anyhow::ensure!(path.is_dir(), "#not_directory");
    log::info!("initialize profile: {}, {:?}", name, path);

    let index_store = index_store_path(app)?;
    let mut index_store =
        crate::store::open_store::<crate::store::IndexStore>(&index_store).await?;

    let id = uuid::Uuid::new_v4();

    let store_path = path.join("au2ec").join("content_store.json");
    if store_path.exists() {
        if index_store.profiles.values().any(|p| p.path == path) {
            anyhow::bail!("#already_initialized");
        } else if reinit {
            fs_err::tokio::remove_file(&store_path).await?;
            log::warn!("existing store file removed: {:?}", store_path);
        } else {
            anyhow::bail!("#reinit_required");
        }
    }

    fs_err::tokio::create_dir_all(store_path.parent().unwrap()).await?;

    let mut content_store = open_store::<crate::store::ContentStore>(&store_path).await?;
    content_store.name = name.clone();

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

pub async fn list_profiles(
    app: &tauri::AppHandle,
) -> anyhow::Result<std::collections::HashMap<uuid::Uuid, crate::store::IndexProfile>> {
    let index_store_path = index_store_path(app)?;
    let index_store =
        crate::store::open_store::<crate::store::IndexStore>(&index_store_path).await?;

    Ok(index_store.profiles.clone())
}

pub async fn list_registries(
    app: &tauri::AppHandle,
) -> anyhow::Result<std::collections::HashSet<url::Url>> {
    let index_store_path = index_store_path(app)?;
    let index_store =
        crate::store::open_store::<crate::store::IndexStore>(&index_store_path).await?;

    Ok(index_store.registries.clone())
}
