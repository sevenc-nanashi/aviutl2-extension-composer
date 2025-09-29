use crate::models::Version;
use tokio::io::AsyncReadExt;

use anyhow::Context;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct InternalStore {
    pub version: u32,
    pub value: serde_json::Value,
}

#[derive(Debug)]
pub struct LockedStore<T: Store> {
    path: std::path::PathBuf,
    #[allow(dead_code)]
    file: fs_err::tokio::File,
    inner: T,
    original: T,
    saved: bool,
}

impl<T: Store> Drop for LockedStore<T> {
    fn drop(&mut self) {
        if !self.saved {
            log::warn!(
                "Warning: The store was not saved before being dropped. Changes will be lost."
            );
        }
    }
}

pub async fn open_store<T: Store>(path: &std::path::Path) -> anyhow::Result<LockedStore<T>> {
    log::info!("Opening store at {:?}", path);
    fs_err::tokio::create_dir_all(
        path.parent()
            .context("Failed to get the parent directory of the store file")?,
    )
    .await
    .context("Failed to create the parent directory of the store file")?;

    let file = fs_err::File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .context("Failed to create or open the store file")?;
    file.file()
        .lock()
        .context("Failed to lock the store file")?;
    let mut file = fs_err::tokio::File::from_std(file);
    let mut data = Vec::new();
    file.read_to_end(&mut data).await?;

    let (internal, saved): (InternalStore, bool) = {
        if data.is_empty() {
            (
                InternalStore {
                    version: T::CURRENT_VERSION,
                    value: serde_json::to_value(T::default())?,
                },
                false,
            )
        } else {
            (serde_json::from_slice(&data)?, true)
        }
    };

    let value = if internal.version == T::CURRENT_VERSION {
        internal.value
    } else {
        T::migrate(internal.version, T::CURRENT_VERSION, internal.value)?
    };
    let inner: T = serde_json::from_value(value)?;
    Ok(LockedStore {
        path: path.to_path_buf(),
        file,
        original: inner.clone(),
        inner,
        saved,
    })
}

impl<T: Store> LockedStore<T> {
    pub async fn save(&mut self) -> anyhow::Result<()> {
        let internal = InternalStore {
            version: T::CURRENT_VERSION,
            value: serde_json::to_value(&self.inner)?,
        };
        let data = serde_json::to_vec_pretty(&internal)?;
        let tmp_path = std::path::PathBuf::from(format!("{}.tmp", self.path.display()));
        fs_err::tokio::write(&tmp_path, data).await?;
        fs_err::tokio::rename(&tmp_path, &self.path).await?;
        self.saved = true;
        self.original = self.inner.clone();
        Ok(())
    }

    pub fn revert(&mut self) {
        self.inner = self.original.clone();
        self.saved = true;
    }
}

#[macro_export]
macro_rules! save_all {
    ($($store:ident),* $(,)?) => {
        match tokio::join!( $( $store.save() ),* ) {
            ($( $crate::one!($store, Ok(_)) ),*) => Ok(()),
            errs @ ($( $crate::one!($store, _) ),*) => {
                $(
                    $store.revert();
                )*
                let ($( $crate::one!($store, $store) ),*) = errs;
                Err(anyhow::anyhow!(
                    "Failed to save all stores: {}",
                    vec![$(format!("{:?}", $store)),*].join(", ")
                ))
            }
        }
    };
}
#[macro_export]
macro_rules! one {
    ($store:expr, $expr:pat) => {
        $expr
    };
}

impl<T: Store> std::ops::Deref for LockedStore<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<T: Store> std::ops::DerefMut for LockedStore<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.saved = false;
        &mut self.inner
    }
}

pub trait Store: serde::Serialize + serde::de::DeserializeOwned + Default + Clone {
    const CURRENT_VERSION: u32;
    fn migrate(from: u32, to: u32, value: serde_json::Value) -> anyhow::Result<serde_json::Value>;
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexStore {
    #[serde(default)]
    pub profiles: std::collections::BTreeMap<uuid::Uuid, IndexProfile>,
    #[serde(default)]
    pub registries: std::collections::BTreeMap<uuid::Uuid, url::Url>,
    #[serde(default)]
    pub manifests: std::collections::BTreeMap<uuid::Uuid, url::Url>,
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexProfile {
    pub name: String,
    pub path: std::path::PathBuf,
}

impl Store for IndexStore {
    const CURRENT_VERSION: u32 = 1;
    fn migrate(
        _from: u32,
        _to: u32,
        value: serde_json::Value,
    ) -> anyhow::Result<serde_json::Value> {
        // NOTE: 今回はマイグレーションがないのでそのまま返す
        Ok(value)
    }
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProfileStore {
    pub name: String,

    pub contents: std::collections::BTreeMap<crate::models::ManifestId, crate::models::Manifest>,
}

impl Store for ProfileStore {
    const CURRENT_VERSION: u32 = 1;
    fn migrate(
        _from: u32,
        _to: u32,
        value: serde_json::Value,
    ) -> anyhow::Result<serde_json::Value> {
        // NOTE: 今回はマイグレーションがないのでそのまま返す
        Ok(value)
    }
}
