use cached::proc_macro::cached;
use std::time::Duration;

use crate::{anyhow_to_string, models, store::open_store};

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

    let mut index_store = crate::utils::open_index_store(app).await?;

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
    let index_store = crate::utils::open_index_store(app).await?;

    Ok(index_store.profiles.clone())
}

pub async fn list_registries(
    app: &tauri::AppHandle,
) -> anyhow::Result<std::collections::HashMap<uuid::Uuid, url::Url>> {
    let index_store = crate::utils::open_index_store(app).await?;

    Ok(index_store.registries.clone())
}

pub async fn fetch_registry(registry: url::Url) -> anyhow::Result<models::Registry> {
    fetch_may_follow_url(&registry, "#invalid_as_registry").await
}

#[cached(time = 60, result = true)]
pub async fn fetch_registry_cached(registry: url::Url) -> Result<models::Registry, String> {
    fetch_may_follow_url(&registry, "#invalid_as_registry")
        .await
        .map_err(anyhow_to_string)
}

pub async fn add_registry(app: &tauri::AppHandle, registry: url::Url) -> anyhow::Result<()> {
    let mut index_store = crate::utils::open_index_store(app).await?;

    if index_store.registries.values().any(|r| r == &registry) {
        anyhow::bail!("#already_added");
    }

    let _ = fetch_may_follow_url::<models::Registry>(&registry, "#invalid_as_registry").await?;

    index_store
        .registries
        .insert(uuid::Uuid::new_v4(), registry);
    index_store.save().await?;

    Ok(())
}

async fn fetch_json_or_yaml<T: serde::de::DeserializeOwned + Send>(
    registry: &url::Url,
    on_unexpected_response: &str,
) -> anyhow::Result<T> {
    match fetch_json_yaml_or_ok_response(registry, on_unexpected_response).await {
        Ok(v) => Ok(v),
        Err(e) => match e {
            either::Either::Left(err) => Err(err),
            either::Either::Right(_) => {
                anyhow::bail!("{on_unexpected_response}")
            }
        },
    }
}

async fn fetch_json_yaml_or_ok_response<T: serde::de::DeserializeOwned + Send>(
    registry: &url::Url,
    on_unexpected_response: &str,
) -> Result<T, either::Either<anyhow::Error, reqwest::Response>> {
    let client = reqwest::Client::new();
    let res = client
        .get(registry.clone())
        .header(
            "User-Agent",
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        )
        .send()
        .await
        .map_err(|e| either::Either::Left(e.into()))?
        .error_for_status()
        .map_err(|e| either::Either::Left(e.into()))?;
    let content_type = res
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if content_type.starts_with("application/json") || content_type.starts_with("text/json") {
        let text = res
            .text()
            .await
            .map_err(|e| either::Either::Left(e.into()))?;
        let parsed: T = serde_json::from_str(&text).map_err(|e| either::Either::Left(e.into()))?;
        return Ok(parsed);
    }
    if content_type.starts_with("application/yaml") || content_type.starts_with("text/yaml") {
        let text = res
            .text()
            .await
            .map_err(|e| either::Either::Left(e.into()))?;
        let parsed: T = serde_yml::from_str(&text).map_err(|e| either::Either::Left(e.into()))?;
        return Ok(parsed);
    }
    if content_type.starts_with("text/plain") {
        let text = res
            .text()
            .await
            .map_err(|e| either::Either::Left(e.into()))?;
        let parsed = serde_json::from_str::<T>(&text)
            .ok()
            .or_else(|| serde_yml::from_str::<T>(&text).ok());
        if let Some(v) = parsed {
            return Ok(v);
        }
        return Err(either::Either::Left(anyhow::anyhow!(
            "{on_unexpected_response}"
        )));
    }
    Err(either::Either::Right(res))
}

async fn fetch_may_follow_url<T: serde::de::DeserializeOwned + Send>(
    registry: &url::Url,
    on_unexpected_response: &str,
) -> anyhow::Result<T> {
    let response = match fetch_json_yaml_or_ok_response::<T>(registry, on_unexpected_response).await
    {
        Ok(v) => return Ok(v),
        Err(e) => match e {
            either::Either::Left(err) => return Err(err),
            either::Either::Right(res) => res,
        },
    };
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if !content_type.starts_with("text/html") {
        anyhow::bail!("{on_unexpected_response}");
    }

    let html = response.text().await?;
    let url = 'url: {
        let document = scraper::Html::parse_document(&html);

        let link_alternate = document
            .select(&scraper::Selector::parse("html > head > link[rel='alternate', type='application/yaml+aviutl2-extension-composer']").unwrap())
            .filter_map(|el| {
                el.value()
                    .attr("href")
                    .and_then(|href| registry.join(href).ok())
            })
            .next();
        if link_alternate.is_some() {
            break 'url link_alternate;
        }

        let selector = scraper::Selector::parse("pre, code").unwrap();
        let pre_code = document.select(&selector);

        let pattern = lazy_regex::lazy_regex!(
            r"\s*aviutl2-extension-composer:alternate:(?<url>https?:\/\/[^\s]+)\s*"
        );
        for el in pre_code {
            let text = el.text().collect::<Vec<_>>().join("");
            if let Some(url) = pattern
                .captures(&text)
                .and_then(|caps| caps.name("url"))
                .and_then(|m| m.as_str().parse::<url::Url>().ok())
            {
                break 'url Some(url);
            }
        }

        None
    };
    if let Some(url) = url {
        return fetch_json_or_yaml(&url, on_unexpected_response).await;
    }

    anyhow::bail!("{on_unexpected_response}");
}
