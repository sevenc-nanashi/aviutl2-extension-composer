use cached::proc_macro::cached;
use std::time::Duration;

use crate::{
    anyhow_to_string, installer::DATA_DIR, models, path_match::matches_path, store::open_store,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnExist {
    ReuseExisting,
    RemoveExisting,
    Abort,
}

pub async fn initialize_profile(
    app: &tauri::AppHandle,
    name: String,
    path: std::path::PathBuf,
    on_exist: OnExist,
) -> anyhow::Result<uuid::Uuid> {
    anyhow::ensure!(!name.is_empty(), "#name_empty");
    anyhow::ensure!(path.exists(), "#not_exists");
    anyhow::ensure!(path.is_dir(), "#not_directory");
    log::info!("initialize profile: {}, {:?}", name, path);

    let mut index_store = crate::utils::open_index_store(app).await?;

    let id = uuid::Uuid::now_v7();

    let store_path = path.join(DATA_DIR).join("store.json");
    if store_path.exists() {
        if index_store.profiles.values().any(|p| p.path == path) {
            anyhow::bail!("#already_initialized");
        }
        match on_exist {
            OnExist::RemoveExisting => {
                fs_err::tokio::remove_file(&store_path).await?;
                log::warn!("existing store file removed: {:?}", store_path);
            }
            OnExist::ReuseExisting => {
                let content_store = open_store::<crate::store::ProfileStore>(&store_path).await?;
                index_store.profiles.insert(
                    id,
                    crate::store::IndexProfile {
                        name: content_store.name.clone(),
                        path: path.clone(),
                    },
                );
                index_store.save().await?;
                log::info!("existing store file reused: {:?}", store_path);
                return Ok(id);
            }
            OnExist::Abort => {
                anyhow::bail!("#already_exists");
            }
        }
    }

    fs_err::tokio::create_dir_all(store_path.parent().unwrap()).await?;

    let mut content_store = open_store::<crate::store::ProfileStore>(&store_path).await?;
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
) -> anyhow::Result<std::collections::BTreeMap<uuid::Uuid, crate::store::IndexProfile>> {
    let index_store = crate::utils::open_index_store(app).await?;

    Ok(index_store.profiles.clone())
}

pub async fn unregister_profile(
    app: &tauri::AppHandle,
    profile_id: uuid::Uuid,
) -> anyhow::Result<()> {
    let mut index_store = crate::utils::open_index_store(app).await?;

    if index_store.profiles.remove(&profile_id).is_none() {
        anyhow::bail!("#profile_not_found");
    }

    index_store.save().await?;

    Ok(())
}

pub async fn remove_profile(app: &tauri::AppHandle, profile_id: uuid::Uuid) -> anyhow::Result<()> {
    let index_store = crate::utils::open_index_store(app).await?;
    let profile = index_store
        .profiles
        .get(&profile_id)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("#profile_not_found"))?;
    drop(index_store);

    unregister_profile(app, profile_id).await?;

    let store_dir = profile.path.join(DATA_DIR);
    if store_dir.exists() {
        fs_err::tokio::remove_dir_all(&store_dir).await?;
    }

    Ok(())
}

pub async fn open_profile_folder(
    app: &tauri::AppHandle,
    profile_id: uuid::Uuid,
) -> anyhow::Result<()> {
    let index_store = crate::utils::open_index_store(app).await?;
    let profile = index_store
        .profiles
        .get(&profile_id)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("#profile_not_found"))?;

    tauri_plugin_opener::open_path(&profile.path, None::<&str>)?;

    Ok(())
}

pub async fn list_registries(
    app: &tauri::AppHandle,
) -> anyhow::Result<std::collections::BTreeMap<uuid::Uuid, url::Url>> {
    let index_store = crate::utils::open_index_store(app).await?;

    Ok(index_store.registries.clone())
}

pub async fn fetch_registry(registry: url::Url) -> anyhow::Result<models::Registry> {
    crate::fetch::fetch_may_follow_url(&registry, "#invalid_as_registry").await
}

#[cached(time = 60, result = true)]
pub async fn fetch_registry_cached(registry: url::Url) -> Result<models::Registry, String> {
    crate::fetch::fetch_may_follow_url(&registry, "#invalid_as_registry")
        .await
        .map_err(anyhow_to_string)
}

pub async fn fetch_manifest(
    app: &tauri::AppHandle,
    manifest_url: url::Url,
) -> anyhow::Result<models::Manifest> {
    if manifest_url.scheme() == "local" {
        let path = manifest_url.path();
        // local:///manifests/{id}
        // local:///profiles/{profile_id}/manifests/{id}
        if let Some(params) = matches_path(path, "/manifests/:id") {
            let id = params.get("id").unwrap();
            let manifests_dir = crate::utils::manifests_dir(app)?;
            let manifest_path = manifests_dir.join(format!("{id}.yml"));
            if !manifest_path.exists() {
                anyhow::bail!("#not_found");
            }
            let manifest_bytes = fs_err::tokio::read(&manifest_path).await?;
            let mut manifest: models::Manifest = serde_yml::from_slice(&manifest_bytes)?;
            if manifest.manifest_url.is_none() {
                manifest.manifest_url = Some(crate::models::HttpUrl(manifest_url.to_owned()));
            }
            return Ok(manifest);
        } else if let Some(params) = matches_path(path, "/profiles/:profile_id/manifests/:id") {
            let profile_id = params.get("profile_id").unwrap();
            let id = params.get("id").unwrap();
            let index_store = crate::utils::open_index_store(app).await?;
            let profile = index_store
                .profiles
                .values()
                .find(|p| {
                    p.path.file_name().map(|s| s.to_string_lossy()) == Some(profile_id.into())
                })
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("#profile_not_found"))?;
            let store = get_profile_store(app, profile_id.parse()?).await?;
            let manifest = store
                .contents
                .get(&id.parse()?)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("#not_found"))?;
            return Ok(manifest);
        } else {
            anyhow::bail!("#invalid_local_manifest_url");
        }
    }
    let mut manifest =
        crate::fetch::fetch_json_or_yaml::<models::Manifest>(&manifest_url, "#invalid_as_manifest")
            .await?;
    if manifest.manifest_url.is_none() {
        manifest.manifest_url = Some(crate::models::HttpUrl(manifest_url.to_owned()));
    }

    Ok(manifest)
}

#[cached(
    time = 60,
    result = true,
    key = "String",
    convert = r#"{ manifest_url.as_str().to_string() }"#
)]
pub async fn fetch_manifest_cached(
    app: &tauri::AppHandle,
    manifest_url: url::Url,
) -> Result<models::Manifest, String> {
    fetch_manifest(app, manifest_url)
        .await
        .map_err(anyhow_to_string)
}

pub async fn add_registry(app: &tauri::AppHandle, registry: url::Url) -> anyhow::Result<()> {
    let mut index_store = crate::utils::open_index_store(app).await?;

    if index_store.registries.values().any(|r| r == &registry) {
        anyhow::bail!("#already_added");
    }

    let _ =
        crate::fetch::fetch_may_follow_url::<models::Registry>(&registry, "#invalid_as_registry")
            .await?;

    index_store
        .registries
        .insert(uuid::Uuid::now_v7(), registry);
    index_store.save().await?;

    Ok(())
}

pub async fn list_manifests(
    app: &tauri::AppHandle,
) -> anyhow::Result<std::collections::BTreeMap<uuid::Uuid, url::Url>> {
    let index_store = crate::utils::open_index_store(app).await?;
    Ok(index_store.manifests.clone())
}

pub async fn add_manifest_url(
    app: &tauri::AppHandle,
    manifest_url: url::Url,
) -> anyhow::Result<()> {
    let mut index_store = crate::utils::open_index_store(app).await?;

    if index_store.manifests.values().any(|r| r == &manifest_url) {
        anyhow::bail!("#already_added");
    }

    // Validate
    let _ = fetch_manifest(app, manifest_url.clone()).await?;

    index_store
        .manifests
        .insert(uuid::Uuid::now_v7(), manifest_url);
    index_store.save().await?;

    Ok(())
}

pub async fn add_manifest_local(app: &tauri::AppHandle, file: Vec<u8>) -> anyhow::Result<()> {
    let mut index_store = crate::utils::open_index_store(app).await?;

    // Read and parse
    let manifest: models::Manifest = serde_yml::from_slice(&file)?;
    let id = manifest.id.to_string();

    let manifests_dir = crate::utils::manifests_dir(app)?;
    fs_err::create_dir_all(&manifests_dir)?;
    let dest_path = manifests_dir.join(format!("{id}.yml"));
    fs_err::tokio::write(&dest_path, file).await?;

    let url = url::Url::parse(&format!("local:///manifests/{id}"))?;
    if index_store.manifests.values().any(|r| r == &url) {
        anyhow::bail!("#already_added");
    }
    index_store.manifests.insert(uuid::Uuid::now_v7(), url);
    index_store.save().await?;
    Ok(())
}

pub async fn remove_manifest(app: &tauri::AppHandle, manifest: uuid::Uuid) -> anyhow::Result<()> {
    let mut index_store = crate::utils::open_index_store(app).await?;

    if index_store.manifests.remove(&manifest).is_none() {
        anyhow::bail!("#not_found");
    }

    index_store.save().await?;

    Ok(())
}

pub async fn remove_registry(app: &tauri::AppHandle, registry: uuid::Uuid) -> anyhow::Result<()> {
    let mut index_store = crate::utils::open_index_store(app).await?;

    if index_store.registries.remove(&registry).is_none() {
        anyhow::bail!("#not_found");
    }

    index_store.save().await?;

    Ok(())
}

pub async fn get_profile_store(
    app: &tauri::AppHandle,
    profile_id: uuid::Uuid,
) -> anyhow::Result<crate::store::LockedStore<crate::store::ProfileStore>> {
    let index_store = crate::utils::open_index_store(app).await?;
    let profile = index_store
        .profiles
        .get(&profile_id)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("#profile_not_found"))?;
    let store_path = profile.path.join(DATA_DIR).join("store.json");
    if !store_path.exists() {
        anyhow::bail!("#store_not_found");
    }
    let content_store = open_store::<crate::store::ProfileStore>(&store_path).await?;
    Ok(content_store)
}

pub async fn plan_installation(
    app: &tauri::AppHandle,
    profile_id: uuid::Uuid,
    desired: Vec<models::Manifest>,
) -> anyhow::Result<crate::installer::InstallPlan> {
    let index_store = crate::utils::open_index_store(app).await?;
    if !index_store.profiles.contains_key(&profile_id) {
        anyhow::bail!("#profile_not_found");
    }
    let profile_path = index_store.profiles.get(&profile_id).unwrap().path.clone();
    drop(index_store);
    let store = get_profile_store(app, profile_id).await?;
    let existing: Vec<models::Manifest> = store.contents.values().cloned().collect();
    let plan = crate::installer::InstallPlan::plan(&profile_path, &existing, &desired)?;
    Ok(plan)
}

pub async fn perform_installation(
    app: &tauri::AppHandle,
    profile_id: uuid::Uuid,
    plan: crate::installer::InstallPlan,
    ch: tauri::ipc::Channel<(crate::models::ManifestId, crate::installer::InstallProgress)>,
) -> anyhow::Result<()> {
    let index_store = crate::utils::open_index_store(app).await?;
    if !index_store.profiles.contains_key(&profile_id) {
        anyhow::bail!("#profile_not_found");
    }
    let profile_path = index_store.profiles.get(&profile_id).unwrap().path.clone();
    drop(index_store);

    plan.perform(&profile_path, ch).await?;

    Ok(())
}
