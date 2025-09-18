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
                crate::models::RootType::Theme => "au2ec/themes",
                crate::models::RootType::Data => unreachable!(),
            })
            .join(&self.path),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InstallPlan {
    pub to_keep: Vec<crate::models::Manifest>,
    pub to_update: Vec<(crate::models::Manifest, crate::models::Manifest)>,
    pub to_install: Vec<crate::models::Manifest>,
}

impl InstallPlan {
    pub fn plan(
        root: &std::path::Path,
        existing: &[crate::models::Manifest],
        desired: &[crate::models::Manifest],
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
            if !desired_ids.contains_key(id) {
                to_keep.push(existing_manifest.clone());
            }
        }

        // TODO: rootをチェックして既存ファイルがあったらエラーにする

        let mut used_paths = std::collections::HashMap::new();
        let mut conflicts = Vec::new();
        for manifest in to_keep
            .iter()
            .chain(to_update.iter().map(|(_, m)| m))
            .chain(to_install.iter())
        {
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
