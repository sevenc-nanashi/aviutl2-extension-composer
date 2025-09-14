mod generated {
    include!(env!("SCHEMA_RS_PATH"));
}
pub use generated::*;
pub use overrides::*;

mod overrides {
    #[allow(non_camel_case_types)]
    pub type u64 = ::std::primitive::u64;

    pub type Url = ::url::Url;

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
