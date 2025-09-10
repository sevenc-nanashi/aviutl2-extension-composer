use serde::ser::{SerializeMap, SerializeStruct};

#[derive(Debug, Clone, serde::Serialize)]
pub struct ManifestId(String);

impl std::str::FromStr for ManifestId {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("empty manifest id".to_string());
        }
        let id_pattern =
            lazy_regex::lazy_regex!(r"^(?<author>[a-z0-9_]+)-(?<content_name>[a-z0-9_-]+)$");
        if !id_pattern.is_match(s) {
            return Err("invalid manifest id format".to_string());
        }
        if s.contains("--") {
            return Err("invalid manifest id: consecutive hyphens".to_string());
        }
        Ok(ManifestId(s.to_string()))
    }
}

impl<'de> serde::Deserialize<'de> for ManifestId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            return Err(serde::de::Error::custom("empty manifest id"));
        }

        s.parse().map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataDirRelativePath {
    pub root: String,
    pub path: String,
}

pub static DATA_DIR_PREFIXES: &[&str] = &[
    "$alias",
    "$default",
    "$palette",
    "$figure",
    "$language",
    "$plugin",
    "$script",
    "$transition",
    "$theme",
    "$data",
];

impl std::str::FromStr for DataDirRelativePath {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("empty DataDirRelativePath".to_string());
        }
        if s.contains('\\') {
            return Err("DataDirRelativePath must use forward slashes".to_string());
        }
        if s.starts_with('/') {
            return Err("DataDirRelativePath must be relative".to_string());
        }
        if s.contains("..") {
            return Err("DataDirRelativePath must not contain '..'".to_string());
        }
        for prefix in DATA_DIR_PREFIXES {
            if s.starts_with(prefix) {
                let path = s
                    .strip_prefix(prefix)
                    .unwrap()
                    .trim_start_matches('/')
                    .replace('\\', "/")
                    .replace("//", "/");
                return Ok(DataDirRelativePath {
                    root: prefix.to_string(),
                    path: path.to_string(),
                });
            }
        }
        Err(format!(
            "DataDirRelativePath must start with one of the known prefixes: {:?}",
            DATA_DIR_PREFIXES
        ))
    }
}

impl std::fmt::Display for DataDirRelativePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.path.is_empty() {
            write!(f, "{}", self.root)
        } else {
            write!(f, "{}/{}", self.root, self.path)
        }
    }
}

impl<'de> serde::Deserialize<'de> for DataDirRelativePath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            return Err(serde::de::Error::custom("empty DataDirRelativePath"));
        }
        s.parse().map_err(serde::de::Error::custom)
    }
}
impl serde::Serialize for DataDirRelativePath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum MaybeLocalizedString {
    Unlocalized(String),
    Localized(std::collections::HashMap<String, String>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Manifest {
    pub manifest_version: u64,
    pub manifest_url: Option<url::Url>,
    pub id: ManifestId,
    pub name: String,
    pub summary: MaybeLocalizedString,
    pub tags: Vec<MaybeLocalizedString>,
    pub version: String,
    pub version_number: Option<u64>,
    pub authors: Vec<Author>,
    pub license: Option<License>,
    pub homepage: Option<url::Url>,
    pub description: MaybeLocalizedString,
    pub changelogs: Vec<ChangeLog>,
    pub bundles: std::collections::HashMap<String, url::Url>,
    pub resources: Vec<Resource>,
    pub configurations: Vec<Configuration>,
    pub disposables: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Author {
    pub name: String,
    pub url: Option<url::Url>,
}

#[derive(Debug, Clone)]
pub enum License {
    Free {
        text: MaybeLocalizedString,
    },
    Nicovideo {
        id: String,
        text: MaybeLocalizedString,
    },
    Custom {
        text: MaybeLocalizedString,
    },
    Misc {
        name: String,
        text: MaybeLocalizedString,
        others: std::collections::HashMap<String, serde_json::Value>,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChangeLog {
    pub version: String,
    pub version_number: Option<u64>,
    pub changes: MaybeLocalizedString,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Resource {
    pub source: url::Url,
    pub destination: DataDirRelativePath,
    pub scope: Option<String>,
    pub sha256: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Configuration {
    pub path: DataDirRelativePath,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Disposable {
    pub path: DataDirRelativePath,
}

impl serde::Serialize for License {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            License::Free { text } => {
                let mut state = serializer.serialize_struct("License", 2)?;
                state.serialize_field("name", "free")?;
                state.serialize_field("text", text)?;
                state.end()
            }
            License::Nicovideo { id, text } => {
                let mut state = serializer.serialize_struct("License", 3)?;
                state.serialize_field("name", "nicovideo")?;
                state.serialize_field("id", id)?;
                state.serialize_field("text", text)?;
                state.end()
            }
            License::Custom { text } => {
                let mut state = serializer.serialize_struct("License", 2)?;
                state.serialize_field("name", "custom")?;
                state.serialize_field("text", text)?;
                state.end()
            }
            License::Misc { name, text, others } => {
                let mut state = serializer.serialize_map(Some(2 + others.len()))?;
                state.serialize_entry("name", name)?;
                state.serialize_entry("text", text)?;
                for (k, v) in others {
                    state.serialize_entry(k, v)?;
                }
                state.end()
            }
        }
    }
}
impl<'de> serde::Deserialize<'de> for License {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct LicenseHelper {
            name: String,
            text: MaybeLocalizedString,
            id: Option<String>,
            #[serde(flatten)]
            others: std::collections::HashMap<String, serde_json::Value>,
        }

        let helper = LicenseHelper::deserialize(deserializer)?;
        match helper.name.as_str() {
            "free" => Ok(License::Free { text: helper.text }),
            "nicovideo" => {
                let id = helper.id.ok_or_else(|| {
                    serde::de::Error::custom("missing 'id' field for Nicovideo license")
                })?;
                Ok(License::Nicovideo {
                    id,
                    text: helper.text,
                })
            }
            "custom" => Ok(License::Custom { text: helper.text }),
            _ => Ok(License::Misc {
                name: helper.name,
                text: helper.text,
                others: helper.others,
            }),
        }
    }
}

trait Migratable {
    const CURRENT_VERSION: u64;

    fn version_key() -> &'static str;
    fn migrate(from: u64, to: u64, value: serde_json::Value) -> anyhow::Result<serde_json::Value>;

    fn parse(s: &str) -> anyhow::Result<Self>
    where
        Self: Sized + serde::de::DeserializeOwned,
    {
        let mut value: serde_json::Value = serde_json::from_str(s)?;
        let current_version = Self::CURRENT_VERSION;
        let version_key = Self::version_key();

        let from_version = value.get(version_key).and_then(|v| v.as_u64()).unwrap_or(0);

        if from_version > current_version {
            anyhow::bail!("unsupported version: {from_version}");
        }

        if from_version < current_version {
            value = Self::migrate(from_version, current_version, value)?;
            value[version_key] = serde_json::json!(current_version);
        }

        let result: Self = serde_json::from_value(value)?;
        Ok(result)
    }
}
