pub use aviutl2_extension_composer_models::*;
use serde::ser::SerializeStruct;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub prerelease: Option<String>,
    pub version_number: Option<u64>,
}

impl serde::Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut strukt = serializer.serialize_struct("Version", 5)?;
        strukt.serialize_field("version", &self.to_string())?;
        strukt.serialize_field("version_number", &self.version_number)?;

        strukt.end()
    }
}
impl<'de> serde::Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct Helper {
            version: String,
            version_number: Option<u64>,
        }
        let helper = Helper::deserialize(deserializer)?;
        Version::new(helper.version, helper.version_number)
            .map_err(|e| serde::de::Error::custom(format!("invalid version format: {e}")))
    }
}

static PATTERN: lazy_regex::Lazy<lazy_regex::regex::Regex> = lazy_regex::lazy_regex!(
    r"^(?<major>[0-9]+)\.(?<minor>[0-9]+)(?:\.(?<patch>[0-9]+))?(?:-(?<prerelease>[0-9A-Za-z.-]+))?$"
);

pub trait AsVersion {
    fn as_version(&self) -> Version;
}

#[duplicate::duplicate_item(
    Target;
    [crate::models::Manifest];
    [crate::models::ManifestPreviousVersionsItem];
    [crate::models::RegistryContentsItem];
    [crate::models::ManifestChangelogsItem];
)]
impl AsVersion for Target {
    fn as_version(&self) -> Version {
        Version::new(
            self.version.to_string(),
            self.version_number.as_ref().map(|v| v.0),
        )
        .unwrap()
    }
}

impl Version {
    pub fn new(version: String, version_number: Option<u64>) -> anyhow::Result<Self> {
        let version_parsed = PATTERN
            .captures(&version)
            .ok_or_else(|| anyhow::anyhow!("#invalid_version_format"))?;
        let major: u64 = version_parsed
            .name("major")
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let minor: u64 = version_parsed
            .name("minor")
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let patch: u64 = version_parsed
            .name("patch")
            .map(|m| m.as_str().parse().unwrap())
            .unwrap_or(0);
        let prerelease = version_parsed
            .name("prerelease")
            .map(|m| m.as_str().to_string());
        Ok(Self {
            major,
            minor,
            patch,
            prerelease,
            version_number,
        })
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(prerelease) = &self.prerelease {
            write!(f, "-{}", prerelease)?;
        }
        Ok(())
    }
}

impl std::cmp::Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.version_number, other.version_number) {
            (Some(a), Some(b)) => a.cmp(&b),
            (None, Some(_)) => std::cmp::Ordering::Less,
            (Some(_), None) => std::cmp::Ordering::Greater,
            (None, None) => {
                match self.major.cmp(&other.major) {
                    std::cmp::Ordering::Equal => {}
                    ord => return ord,
                }
                match self.minor.cmp(&other.minor) {
                    std::cmp::Ordering::Equal => {}
                    ord => return ord,
                }
                match self.patch.cmp(&other.patch) {
                    std::cmp::Ordering::Equal => {}
                    ord => return ord,
                }
                match (&self.prerelease, &other.prerelease) {
                    (Some(a), Some(b)) => a.cmp(b),
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, None) => std::cmp::Ordering::Equal,
                }
            }
        }
    }
}

impl std::cmp::PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
