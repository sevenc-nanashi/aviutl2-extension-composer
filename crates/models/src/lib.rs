mod schema_generated;
pub use schema_generated::*;

impl Copy for Uint {}
impl PartialEq for DataDirRelativePath {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for DataDirRelativePath {}
impl std::fmt::Display for DataDirRelativePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())?;
        Ok(())
    }
}
impl std::fmt::Display for ManifestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())?;
        Ok(())
    }
}

pub use overrides::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display, strum::EnumString)]
pub enum RootType {
    #[strum(to_string = "$alias")]
    Alias,
    #[strum(serialize = "$default", to_string = "$palette")]
    Palette,
    #[strum(to_string = "$figure")]
    Figure,
    #[strum(to_string = "$language")]
    Language,
    #[strum(to_string = "$plugin")]
    Plugin,
    #[strum(to_string = "$script")]
    Script,
    #[strum(to_string = "$transition")]
    Transition,
    #[strum(to_string = "$theme")]
    Theme,
    #[strum(to_string = "$data")]
    Data,
}

mod overrides {
    use super::RootType;
    #[allow(non_camel_case_types)]
    pub type u64 = ::std::primitive::u64;

    pub type Bundles = std::collections::BTreeMap<BundleId, HttpUrl>;

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
    pub struct BundleId(pub String);
    impl std::ops::Deref for BundleId {
        type Target = String;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl std::ops::DerefMut for BundleId {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl std::fmt::Display for BundleId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl std::hash::Hash for BundleId {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl<'de> serde::Deserialize<'de> for BundleId {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let ss = String::deserialize(deserializer)?;
            if ss.is_empty() {
                return Err(<D::Error as serde::de::Error>::custom("empty bundle ID"));
            }
            let pattern = lazy_regex::lazy_regex!(r"^[a-z0-9_]+$");
            if !pattern.is_match(&ss) {
                return Err(<D::Error as serde::de::Error>::custom(format!(
                    "invalid bundle ID: {} (allowed: lowercase letters, digits, underscores)",
                    ss
                )));
            }
            Ok(BundleId(ss))
        }
    }

    duplicate::duplicate! {
        [
            StructName  allowed_schemes;
            [HttpUrl]   [["http", "https"]];
            [SourceUrl] [["http", "https", "bundle"]]
        ]

        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
        pub struct StructName(pub url::Url);

        impl<'de> serde::Deserialize<'de> for StructName {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let ss = String::deserialize(deserializer)?;
                let url = ss
                    .parse::<url::Url>()
                    .map_err(|e| <D::Error as serde::de::Error>::custom(format!("invalid URL: {}", e)))?;
                if !allowed_schemes.contains(&url.scheme()) {
                    return Err(<D::Error as serde::de::Error>::custom(format!(
                        "invalid URL scheme: {} (allowed: {:?})",
                        url.scheme(),
                        allowed_schemes
                    )));
                }
                Ok(StructName(url))
            }
        }

        impl std::ops::Deref for StructName {
            type Target = url::Url;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for StructName {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl std::fmt::Display for StructName {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::hash::Hash for StructName {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.as_str().hash(state);
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct DataDirRelativePathInner {
        pub root: RootType,
        pub path: String,
    }
    impl DataDirRelativePathInner {
        pub fn is_directory(&self) -> bool {
            self.path.ends_with('/')
        }
    }
    impl serde::Serialize for DataDirRelativePathInner {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(&format!("{}/{}", self.root, self.path))
        }
    }
    impl std::fmt::Display for DataDirRelativePathInner {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}/{}", self.root, self.path)
        }
    }
    impl<'de> serde::Deserialize<'de> for DataDirRelativePathInner {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let ss = String::deserialize(deserializer)?;
            let mut parts = ss.splitn(2, '/');
            let root_str = parts
                .next()
                .ok_or_else(|| <D::Error as serde::de::Error>::custom("missing root"))?;
            let path = parts
                .next()
                .ok_or_else(|| <D::Error as serde::de::Error>::custom("missing path"))?
                .to_string();
            if path.is_empty() {
                return Err(<D::Error as serde::de::Error>::custom("empty path"));
            }
            if path.contains('\\') {
                return Err(<D::Error as serde::de::Error>::custom(
                    "path contains backslash",
                ));
            }
            if path.contains("..") {
                return Err(<D::Error as serde::de::Error>::custom(
                    "path contains parent directory reference",
                ));
            }
            if path.starts_with('/') {
                return Err(<D::Error as serde::de::Error>::custom(
                    "path starts with a slash",
                ));
            }
            if path.contains("//") {
                return Err(<D::Error as serde::de::Error>::custom(
                    "path contains consecutive slashes",
                ));
            }

            let root = root_str
                .parse::<RootType>()
                .map_err(|_| <D::Error as serde::de::Error>::custom("invalid root"))?;
            Ok(DataDirRelativePathInner { root, path })
        }
    }

    pub mod license {
        macro_rules! create_license_struct {
            ($name:ident, $str_val:expr) => {
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct $name;

                impl serde::Serialize for $name {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where
                        S: serde::Serializer,
                    {
                        serializer.serialize_str($str_val)
                    }
                }

                impl<'de> serde::Deserialize<'de> for $name {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        let ss = String::deserialize(deserializer)?;
                        if ss == $str_val {
                            Ok($name)
                        } else {
                            Err(<D::Error as serde::de::Error>::invalid_value(
                                serde::de::Unexpected::Str(&ss),
                                &concat!("`", $str_val, "`"),
                            ))
                        }
                    }
                }
            };
        }
        create_license_struct!(Free, "free");
        create_license_struct!(Nicovideo, "nicovideo");
        create_license_struct!(Custom, "custom");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_example_manifest() {
        insta::assert_debug_snapshot!(
            serde_yml::from_str::<Manifest>(include_str!("../../../docs/examples/manifest.yml"))
                .unwrap()
        );
    }

    #[test]
    fn test_parsing_example_registry() {
        insta::assert_debug_snapshot!(
            serde_yml::from_str::<Registry>(include_str!("../../../docs/examples/registry.yml"))
                .unwrap()
        );
    }
}
