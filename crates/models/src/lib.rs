mod generated {
    include!(env!("SCHEMA_RS_PATH"));

    impl Copy for Uint {}
    impl PartialEq for DataDirRelativePath {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    impl Eq for DataDirRelativePath {}
}
pub use generated::*;
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

    pub type Url = ::url::Url;

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
