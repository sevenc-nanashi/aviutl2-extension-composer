use async_zip::tokio::write::ZipFileWriter;
use futures::StreamExt;
use tokio::io::AsyncWriteExt;
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

pub static DATA_DIR: &str = "au2ec";
pub static TEMP_DIR: &str = "temp";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum PathUsageType {
    Resource,
    Configuration,
    Disposable,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct PathUsage {
    pub path: crate::models::DataDirRelativePath,
    pub usage_type: PathUsageType,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct ConflictInfo {
    pub path: std::path::PathBuf,
    pub existing: PathUsage,
    pub new: PathUsage,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum InstallProgress {
    Download {
        file: usize,
        num_files: usize,
        name: String,
        bytes: u64,
        total_bytes: Option<u64>,
    },
    Remove {
        file: usize,
        num_files: usize,
        name: String,
    },
    Backup {
        file: usize,
        num_files: usize,
        name: String,
    },
    Install {
        file: usize,
        num_files: usize,
        name: String,
    },
    Complete,

    Error {
        message: String,
    },
}

pub trait AsPathUsage {
    fn to_path_usage(&self) -> PathUsage;
}
impl AsPathUsage for crate::models::ManifestResourcesItem {
    fn to_path_usage(&self) -> PathUsage {
        PathUsage {
            path: self.destination.clone(),
            usage_type: PathUsageType::Resource,
            scope: self.scope.as_deref().cloned(),
        }
    }
}

impl AsPathUsage for crate::models::ManifestConfigurationsItem {
    fn to_path_usage(&self) -> PathUsage {
        PathUsage {
            path: self.path.clone(),
            usage_type: PathUsageType::Configuration,
            scope: self.scope.as_deref().cloned(),
        }
    }
}

impl AsPathUsage for crate::models::ManifestDisposablesItem {
    fn to_path_usage(&self) -> PathUsage {
        PathUsage {
            path: self.path.clone(),
            usage_type: PathUsageType::Disposable,
            scope: None,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PlanError {
    #[error("#conflicting_file_paths[{}]", serde_json::to_string(.0).unwrap())]
    Conflict(Vec<ConflictInfo>),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
pub trait DataDirPathExt {
    fn to_absolute_path(&self, root: &std::path::Path) -> std::path::PathBuf;

    fn as_relative_path(&self) -> std::path::PathBuf;
}

impl DataDirPathExt for crate::models::DataDirRelativePath {
    fn to_absolute_path(&self, root: &std::path::Path) -> std::path::PathBuf {
        root.join(self.as_relative_path())
    }

    fn as_relative_path(&self) -> std::path::PathBuf {
        match self.root {
            crate::models::RootType::Data => std::path::PathBuf::from(&self.path),
            root_type => std::path::PathBuf::from(match root_type {
                crate::models::RootType::Alias => "Alias",
                crate::models::RootType::Palette => "Default",
                crate::models::RootType::Figure => "Figure",
                crate::models::RootType::Language => "Language",
                crate::models::RootType::Plugin => "Plugin",
                crate::models::RootType::Script => "Script",
                crate::models::RootType::Transition => "Transition",
                crate::models::RootType::Theme => const_format::formatc!("{DATA_DIR}/themes"),
                crate::models::RootType::Data => unreachable!(),
            })
            .join(&self.path),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InstallPlan {
    pub to_keep: Vec<crate::models::Manifest>,
    pub to_uninstall: Vec<crate::models::Manifest>,
    pub to_update: Vec<(crate::models::Manifest, crate::models::Manifest)>,
    pub to_install: Vec<crate::models::Manifest>,
}

impl InstallPlan {
    pub fn plan(
        _root: &std::path::Path,
        existing: &[crate::models::Manifest],
        desired: &[crate::models::Manifest],
        uninstall: &[crate::models::ManifestId],
    ) -> Result<Self, PlanError> {
        let existing_ids: std::collections::HashMap<
            crate::models::ManifestId,
            crate::models::Manifest,
        > = existing
            .iter()
            .cloned()
            .map(|m| (m.id.clone(), m))
            .collect();
        let desired_ids: std::collections::HashMap<
            crate::models::ManifestId,
            crate::models::Manifest,
        > = desired.iter().cloned().map(|m| (m.id.clone(), m)).collect();

        let mut to_keep = Vec::new();
        let mut to_uninstall = Vec::new();
        let mut to_update = Vec::new();
        let mut to_install = Vec::new();

        for (id, desired_manifest) in &desired_ids {
            if let Some(existing_manifest) = existing_ids.get(id) {
                to_update.push((existing_manifest.clone(), desired_manifest.clone()));
            } else {
                to_install.push(desired_manifest.clone());
            }
        }

        for (id, existing_manifest) in &existing_ids {
            if !desired_ids.contains_key(id) && uninstall.contains(id) {
                to_keep.push(existing_manifest.clone());
            }
        }

        for id in uninstall {
            if let Some(existing_manifest) = existing_ids.get(id) {
                to_uninstall.push(existing_manifest.clone());
            }
        }

        // TODO: rootをチェックして既存ファイルがあったらエラーにする

        let mut used_paths = std::collections::HashMap::new();
        let mut conflicts = Vec::new();
        for manifest in to_keep.iter().chain(to_install.iter()) {
            for resource in &manifest.resources {
                let new_path = resource.destination.as_relative_path();
                if let Some(usage) = Self::check_conflict(&used_paths, &resource.to_path_usage()) {
                    conflicts.push(ConflictInfo {
                        path: new_path,
                        existing: usage,
                        new: resource.to_path_usage(),
                    });
                } else {
                    used_paths.insert(new_path, resource.to_path_usage());
                }
            }
        }

        Ok(Self {
            to_keep,
            to_uninstall,
            to_update,
            to_install,
        })
    }

    fn check_conflict(
        used_paths: &std::collections::HashMap<std::path::PathBuf, PathUsage>,
        new_usage: &PathUsage,
    ) -> Option<PathUsage> {
        let new_path = new_usage.path.as_relative_path();
        for (used_path, existing_usage) in used_paths {
            if !Self::check_path_conflict(used_path, &new_path) {
                continue;
            }
            if existing_usage == new_usage {
                continue;
            }
            return Some(existing_usage.clone());
        }

        None
    }

    fn check_path_conflict(path1: &std::path::Path, path2: &std::path::Path) -> bool {
        let components1: Vec<&std::ffi::OsStr> =
            path1.components().map(|c| c.as_os_str()).collect();
        let components2: Vec<&std::ffi::OsStr> =
            path2.components().map(|c| c.as_os_str()).collect();
        for (comp1, comp2) in components1.iter().zip(components2.iter()) {
            if comp1 != comp2 {
                return false;
            }
        }
        true
    }

    pub async fn perform(
        &self,
        root: &std::path::Path,
        ch: tauri::ipc::Channel<(crate::models::ManifestId, InstallProgress)>,
    ) -> Result<(), anyhow::Error> {
        for manifest in &self.to_uninstall {
            let lch = ch.clone();
            if let Err(e) = self.perform_uninstall(root, manifest, lch).await {
                ch.send((
                    manifest.id.clone(),
                    InstallProgress::Error {
                        message: format!("{}", e),
                    },
                ))?;
            }
        }
        for (old, new) in &self.to_update {
            let lch = ch.clone();
            if let Err(e) = self.perform_update(root, old, new, lch).await {
                ch.send((
                    new.id.clone(),
                    InstallProgress::Error {
                        message: format!("{}", e),
                    },
                ))?;
            }
        }
        for manifest in &self.to_install {
            let lch = ch.clone();
            if let Err(e) = self.perform_install(root, manifest, lch).await {
                ch.send((
                    manifest.id.clone(),
                    InstallProgress::Error {
                        message: format!("{}", e),
                    },
                ))?;
            }
        }
        Ok(())
    }

    async fn uninstall_internal(
        &self,
        root: &std::path::Path,
        usages: &[(crate::models::ManifestId, PathUsage)],
        ch: tauri::ipc::Channel<(crate::models::ManifestId, InstallProgress)>,
    ) -> Result<(), anyhow::Error> {
        let total_files = usages.len();
        for (i, (id, usage)) in usages.iter().enumerate() {
            ch.send((
                id.clone(),
                InstallProgress::Remove {
                    file: i,
                    num_files: total_files,
                    name: usage.path.as_relative_path().to_string_lossy().to_string(),
                },
            ))?;

            let abs_path = usage.path.to_absolute_path(root);
            if abs_path.exists() {
                tokio::fs::remove_file(&abs_path).await?;
            }
        }

        Ok(())
    }

    async fn shallow_uninstall(
        &self,
        root: &std::path::Path,
        manifest: &crate::models::Manifest,
        ch: tauri::ipc::Channel<(crate::models::ManifestId, InstallProgress)>,
    ) -> Result<(), anyhow::Error> {
        self.uninstall_internal(
            root,
            &manifest
                .resources
                .iter()
                .map(|r| (manifest.id.clone(), r.to_path_usage()))
                .collect::<Vec<_>>(),
            ch.clone(),
        )
        .await
    }

    async fn full_uninstall(
        &self,
        root: &std::path::Path,
        manifest: &crate::models::Manifest,
        ch: tauri::ipc::Channel<(crate::models::ManifestId, InstallProgress)>,
    ) -> Result<(), anyhow::Error> {
        let mut usages = Vec::new();
        usages.extend(
            manifest
                .resources
                .iter()
                .map(|r| (manifest.id.clone(), r.to_path_usage())),
        );
        usages.extend(
            manifest
                .disposables
                .iter()
                .map(|d| (manifest.id.clone(), d.to_path_usage())),
        );
        self.uninstall_internal(root, &usages, ch.clone()).await
    }

    async fn backup_configuration(
        &self,
        root: &std::path::Path,
        manifest: &crate::models::Manifest,
        ch: tauri::ipc::Channel<(crate::models::ManifestId, InstallProgress)>,
    ) -> Result<(), anyhow::Error> {
        let configuration_backup_dir = root.join(DATA_DIR).join("config_backups");
        let configuration_backup = format!(
            "{}_{}_{}.zip",
            chrono::Local::now().format("%Y%m%d%H%M%S"),
            &manifest.name,
            &manifest.version
        );

        fs_err::tokio::create_dir_all(&configuration_backup_dir).await?;

        let mut configuration_files = vec![];
        for config in manifest.configurations.iter() {
            let abs_path = config.path.to_absolute_path(root);
            let mut walk = async_walkdir::WalkDir::new(&abs_path);
            while let Some(entry) = walk.next().await {
                let entry = entry?;
                if entry.file_type().await?.is_file() {
                    configuration_files.push(entry.path().to_path_buf());
                }
            }
        }

        let zip =
            fs_err::tokio::File::open(configuration_backup_dir.join(&configuration_backup)).await?;
        let mut zip = ZipFileWriter::new(zip.compat_write());
        let num_files = configuration_files.len();
        for (i, file) in configuration_files.into_iter().enumerate() {
            ch.send((
                manifest.id.clone(),
                InstallProgress::Backup {
                    file: i,
                    num_files,
                    name: file
                        .strip_prefix(root)
                        .unwrap_or(&file)
                        .to_string_lossy()
                        .to_string(),
                },
            ))?;

            let name = file
                .strip_prefix(root)
                .unwrap_or(&file)
                .to_string_lossy()
                .to_string();
            let entry =
                async_zip::ZipEntryBuilder::new(name.into(), async_zip::Compression::Deflate)
                    .build();

            let mut writer = zip.write_entry_stream(entry).await?;
            futures::io::copy(
                &mut fs_err::tokio::File::open(&file).await?.compat(),
                &mut writer,
            )
            .await?;
        }
        zip.close().await?;

        Ok(())
    }

    async fn perform_update(
        &self,
        root: &std::path::Path,
        old: &crate::models::Manifest,
        new: &crate::models::Manifest,
        ch: tauri::ipc::Channel<(crate::models::ManifestId, InstallProgress)>,
    ) -> Result<(), anyhow::Error> {
        self.shallow_uninstall(root, old, ch.clone()).await?;
        self.perform_install(root, new, ch.clone()).await
    }

    async fn perform_uninstall(
        &self,
        root: &std::path::Path,
        manifest: &crate::models::Manifest,
        ch: tauri::ipc::Channel<(crate::models::ManifestId, InstallProgress)>,
    ) -> Result<(), anyhow::Error> {
        self.backup_configuration(root, manifest, ch.clone())
            .await?;
        self.full_uninstall(root, manifest, ch.clone()).await?;
        ch.send((manifest.id.clone(), InstallProgress::Complete))?;
        Ok(())
    }

    async fn perform_install(
        &self,
        root: &std::path::Path,
        manifest: &crate::models::Manifest,
        ch: tauri::ipc::Channel<(crate::models::ManifestId, InstallProgress)>,
    ) -> Result<(), anyhow::Error> {
        let temp_dir = tempfile::TempDir::with_prefix_in(
            root.join(DATA_DIR).join(TEMP_DIR),
            format!("{}-", &manifest.id),
        )?;
        fs_err::tokio::create_dir_all(&temp_dir).await?;
        let files_to_download = manifest
            .resources
            .iter()
            .filter(|c| c.source.scheme() == "http" || c.source.scheme() == "https")
            .map(|c| {
                (
                    either::Either::Left(c),
                    temp_dir.path().join(url_to_file_name(&c.source)),
                )
            })
            .chain(manifest.bundles.iter().flat_map(|c| {
                c.iter()
                    .map(|r| (either::Either::Right(r), temp_dir.path().join(&**r.0)))
            }))
            .collect::<Vec<_>>();
        let num_files = files_to_download.len();

        for (i, (file, destination)) in files_to_download.into_iter().enumerate() {
            let source = match file {
                either::Either::Left(r) => &url::Url::clone(&r.source),
                either::Either::Right((_name, url)) => &url::Url::clone(url),
            };
            let name = destination.to_string_lossy().to_string();
            ch.send((
                manifest.id.clone(),
                InstallProgress::Download {
                    file: i,
                    num_files,
                    name: name.clone(),
                    bytes: 0,
                    total_bytes: None,
                },
            ))?;

            assert!(source.scheme() == "http" || source.scheme() == "https");

            let resp = reqwest::get(source.as_str()).await?;
            let total_bytes = resp.content_length();
            let mut stream = resp.bytes_stream();

            let mut file = fs_err::tokio::File::create(&destination).await?;
            let mut downloaded_bytes = 0u64;
            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                file.write_all(&chunk).await?;
                downloaded_bytes += chunk.len() as u64;
                ch.send((
                    manifest.id.clone(),
                    InstallProgress::Download {
                        file: i,
                        num_files,
                        name: name.clone(),
                        bytes: downloaded_bytes,
                        total_bytes,
                    },
                ))?;
            }
        }

        let total_files = manifest.resources.len();
        for (i, resource) in manifest.resources.iter().enumerate() {
            ch.send((
                manifest.id.clone(),
                InstallProgress::Install {
                    file: i,
                    num_files: total_files,
                    name: resource
                        .destination
                        .as_relative_path()
                        .to_string_lossy()
                        .to_string(),
                },
            ))?;
            let abs_path = resource.destination.to_absolute_path(root);
            match resource.source.scheme() {
                "http" | "https" => {
                    let temp_file = temp_dir.path().join(url_to_file_name(&resource.source));
                    if !temp_file.exists() {
                        return Err(anyhow::anyhow!(
                            "Temporary file not found: {}",
                            temp_file.display()
                        ));
                    }
                    if let Some(parent) = abs_path.parent() {
                        fs_err::tokio::create_dir_all(parent).await?;
                    }
                    fs_err::tokio::rename(&temp_file, &abs_path).await?;
                }
                "bundle" => {
                    let bundle_name = resource
                        .source
                        .host_str()
                        .ok_or_else(|| anyhow::anyhow!("Invalid bundle URL: {}", resource.source))?
                        .to_string();
                    let bundle = manifest
                        .bundles
                        .iter()
                        .flatten()
                        .find(|(name, _)| name.as_str() == bundle_name)
                        .ok_or_else(|| {
                            anyhow::anyhow!("Bundle not found for resource: {}", resource.source)
                        })?;
                    let bundle_path = temp_dir.path().join(&**bundle.0);
                    if !bundle_path.exists() {
                        return Err(anyhow::anyhow!(
                            "Temporary bundle file not found: {}",
                            bundle_path.display()
                        ));
                    }

                    if let Some(parent) = abs_path.parent() {
                        fs_err::tokio::create_dir_all(parent).await?;
                    }
                    let mut zip = async_zip::tokio::read::seek::ZipFileReader::new(
                        tokio::io::BufReader::new(fs_err::tokio::File::open(&bundle_path).await?)
                            .compat(),
                    )
                    .await?;
                    let wants_directory_source = resource.source.path().ends_with('/');
                    let wants_directory_destination = resource.destination.path.ends_with('/');
                    if wants_directory_source != wants_directory_destination {
                        return Err(anyhow::anyhow!(
                            "Resource source and destination must both be files or both be directories: source={}, destination={}",
                            resource.source,
                            resource.destination
                        ));
                    }
                    let wants_directory = wants_directory_source;
                    let entries = zip
                        .file()
                        .entries()
                        .iter()
                        .enumerate()
                        .filter_map(|(i, e)| {
                            let entry_path = e.filename().as_str().ok()?;
                            if wants_directory {
                                if !entry_path.starts_with(resource.source.path()) {
                                    return None;
                                }
                                let relative_path = &entry_path[resource.source.path().len()..];
                                Some((i, relative_path.to_owned()))
                            } else if entry_path == resource.source.path() {
                                Some((i, "".to_owned()))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();
                    if entries.is_empty() {
                        return Err(anyhow::anyhow!(
                            "File not found in bundle: {}",
                            resource.source.path()
                        ));
                    }
                    if !wants_directory && entries.len() > 1 {
                        return Err(anyhow::anyhow!(
                            "Multiple files found in bundle for single file resource: {}",
                            resource.source.path()
                        ));
                    }
                    for (index, relative_path) in entries {
                        let out_path = if wants_directory {
                            abs_path.join(relative_path)
                        } else {
                            abs_path.clone()
                        };
                        if out_path.to_string_lossy().ends_with('/') {
                            fs_err::tokio::create_dir_all(&out_path).await?;
                            continue;
                        }
                        if let Some(parent) = out_path.parent() {
                            fs_err::tokio::create_dir_all(parent).await?;
                        }
                        let mut reader = zip.reader_without_entry(index).await?;
                        let mut writer = fs_err::tokio::File::create(&out_path)
                            .await
                            .map(|f| f.compat_write())?;
                        futures::io::copy(&mut reader, &mut writer).await?;
                    }
                }
                _ => unreachable!(),
            }
        }

        ch.send((manifest.id.clone(), InstallProgress::Complete))?;
        Ok(())
    }
}

fn url_to_file_name(name: &url::Url) -> String {
    let mut safe_name = String::new();
    let name = name.as_str();
    for c in name.chars() {
        if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' {
            safe_name.push(c);
        } else {
            safe_name.push('_');
        }
    }
    safe_name.push('_');
    safe_name.push_str(&format!(
        "{:x}",
        xxhash_rust::xxh3::xxh3_64(name.as_bytes())
    ));
    safe_name
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_common_segments() {
        let path1 = std::path::Path::new("a/c");
        let path2 = std::path::Path::new("a/d");
        let path3 = std::path::Path::new("a/d/");
        let path4 = std::path::Path::new("a/d/e");

        assert!(InstallPlan::check_path_conflict(path1, path1));

        assert!(!InstallPlan::check_path_conflict(path1, path2));
        assert!(!InstallPlan::check_path_conflict(path2, path1));
        assert!(InstallPlan::check_path_conflict(path2, path3));
        assert!(InstallPlan::check_path_conflict(path3, path2));
        assert!(InstallPlan::check_path_conflict(path2, path4));
        assert!(InstallPlan::check_path_conflict(path4, path2));
    }
}
