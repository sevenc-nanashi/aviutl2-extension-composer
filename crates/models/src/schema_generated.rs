/// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
/**データディレクトリ下の特定のサブディレクトリからの相対パス。

以下のいずれかのディレクトリからの相対パスである必要があります：
- `$alias`：データディレクトリ/Alias 。
- `$default`：データディレクトリ/Default 。
- `$palette`：データディレクトリ/Default 。$defaultのエイリアスです。
- `$figure`：データディレクトリ/Figure 。
- `$language`：データディレクトリ/Language 。
- `$plugin`：データディレクトリ/Plugin 。
- `$script`：データディレクトリ/Script 。
- `$transition`：データディレクトリ/Transition 。
- `$theme`：データディレクトリ/au2ec/themes 。
- `$data`：データディレクトリ自体。これは最終手段としてのみ使用してください。

また、`..`は含めることができません。*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "データディレクトリ下の特定のサブディレクトリからの相対パス。\n\n以下のいずれかのディレクトリからの相対パスである必要があります：\n- `$alias`：データディレクトリ/Alias 。\n- `$default`：データディレクトリ/Default 。\n- `$palette`：データディレクトリ/Default 。$defaultのエイリアスです。\n- `$figure`：データディレクトリ/Figure 。\n- `$language`：データディレクトリ/Language 。\n- `$plugin`：データディレクトリ/Plugin 。\n- `$script`：データディレクトリ/Script 。\n- `$transition`：データディレクトリ/Transition 。\n- `$theme`：データディレクトリ/au2ec/themes 。\n- `$data`：データディレクトリ自体。これは最終手段としてのみ使用してください。\n\nまた、`..`は含めることができません。",
///  "type": "string",
///  "pattern": "^\\$(alias|default|palette|figure|language|plugin|script|transition|theme|data)/(?!.*\\.\\./).*$",
///  "x-rust-type": {
///    "crate": "super",
///    "path": "super::overrides::DataDirRelativePathInner",
///    "version": "*"
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct DataDirRelativePath(pub super::overrides::DataDirRelativePathInner);
impl ::std::ops::Deref for DataDirRelativePath {
    type Target = super::overrides::DataDirRelativePathInner;
    fn deref(&self) -> &super::overrides::DataDirRelativePathInner {
        &self.0
    }
}
impl ::std::convert::From<DataDirRelativePath>
for super::overrides::DataDirRelativePathInner {
    fn from(value: DataDirRelativePath) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DataDirRelativePath> for DataDirRelativePath {
    fn from(value: &DataDirRelativePath) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<super::overrides::DataDirRelativePathInner>
for DataDirRelativePath {
    fn from(value: super::overrides::DataDirRelativePathInner) -> Self {
        Self(value)
    }
}
///`License`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "title": "Free",
///      "type": "object",
///      "required": [
///        "name"
///      ],
///      "properties": {
///        "name": {
///          "description": "このユーザーコンテンツは商用利用など含めて自由に使用できることを示します。",
///          "type": "string",
///          "const": "free",
///          "x-rust-type": {
///            "crate": "super",
///            "path": "super::overrides::license::Free",
///            "version": "*"
///          }
///        },
///        "text": {
///          "description": "利用規約の詳細。",
///          "$ref": "#/$defs/MaybeLocalizedString"
///        }
///      }
///    },
///    {
///      "title": "Nicovideo",
///      "type": "object",
///      "required": [
///        "id",
///        "name"
///      ],
///      "properties": {
///        "name": {
///          "description": "このユーザーコンテンツはニコニコ動画での親作品登録が必要であることを示します。",
///          "type": "string",
///          "const": "nicovideo",
///          "x-rust-type": {
///            "crate": "super",
///            "path": "super::overrides::license::Nicovideo",
///            "version": "*"
///          }
///        },
///        "omittable": {
///          "description": "親作品登録が必須ではないことを示します。省略した場合、親作品登録は必須です。",
///          "type": "boolean"
///        },
///        "id": {
///          "description": "親作品登録に使うID。`smXXXXXX`や`nmXXXXXX`の形式で指定してください。",
///          "type": "string"
///        },
///        "text": {
///          "description": "利用規約の詳細。",
///          "$ref": "#/$defs/MaybeLocalizedString"
///        }
///      }
///    },
///    {
///      "title": "Custom",
///      "type": "object",
///      "required": [
///        "name",
///        "text"
///      ],
///      "properties": {
///        "name": {
///          "description": "このユーザーコンテンツは独自の利用規約を持つことを示します。",
///          "type": "string",
///          "const": "custom",
///          "x-rust-type": {
///            "crate": "super",
///            "path": "super::overrides::license::Custom",
///            "version": "*"
///          }
///        },
///        "text": {
///          "description": "利用規約の詳細。必須です。",
///          "$ref": "#/$defs/MaybeLocalizedString"
///        }
///      }
///    },
///    {
///      "title": "Unknown",
///      "type": "object",
///      "required": [
///        "name"
///      ],
///      "properties": {
///        "name": {
///          "description": "将来の拡張用。",
///          "type": "string",
///          "pattern": "^(?!free$)(?!nicovideo$)(?!custom$)[a-z0-9_]+$"
///        },
///        "text": {
///          "description": "少なくとも、`type`フィールドはstring?である必要があることが保証されます。",
///          "$ref": "#/$defs/MaybeLocalizedString"
///        }
///      },
///      "unevaluatedProperties": {}
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum License {
    Free {
        ///このユーザーコンテンツは商用利用など含めて自由に使用できることを示します。
        name: super::overrides::license::Free,
        ///利用規約の詳細。
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        text: ::std::option::Option<MaybeLocalizedString>,
    },
    Nicovideo {
        ///親作品登録に使うID。`smXXXXXX`や`nmXXXXXX`の形式で指定してください。
        id: ::std::string::String,
        ///このユーザーコンテンツはニコニコ動画での親作品登録が必要であることを示します。
        name: super::overrides::license::Nicovideo,
        ///親作品登録が必須ではないことを示します。省略した場合、親作品登録は必須です。
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        omittable: ::std::option::Option<bool>,
        ///利用規約の詳細。
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        text: ::std::option::Option<MaybeLocalizedString>,
    },
    Custom {
        ///このユーザーコンテンツは独自の利用規約を持つことを示します。
        name: super::overrides::license::Custom,
        ///利用規約の詳細。必須です。
        text: MaybeLocalizedString,
    },
    Unknown {
        ///将来の拡張用。
        name: UnknownName,
        ///少なくとも、`type`フィールドはstring?である必要があることが保証されます。
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        text: ::std::option::Option<MaybeLocalizedString>,
    },
}
impl ::std::convert::From<&Self> for License {
    fn from(value: &License) -> Self {
        value.clone()
    }
}
/**多言語対応文字列。キーにロケール、値にそのロケールでの文字列を持つオブジェクト。
例：{ ja: "こんにちは", en: "Hello" }*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "多言語対応文字列。キーにロケール、値にそのロケールでの文字列を持つオブジェクト。\n例：{ ja: \"こんにちは\", en: \"Hello\" }",
///  "type": "object",
///  "unevaluatedProperties": {
///    "type": "string"
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct LocalizedString(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for LocalizedString {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<LocalizedString>
for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
    fn from(value: LocalizedString) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LocalizedString> for LocalizedString {
    fn from(value: &LocalizedString) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
for LocalizedString {
    fn from(
        value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Self {
        Self(value)
    }
}
///`Manifest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$id": "Manifest.json",
///  "title": "Manifest",
///  "type": "object",
///  "required": [
///    "authors",
///    "id",
///    "license",
///    "manifest_version",
///    "name",
///    "resources",
///    "summary",
///    "version"
///  ],
///  "properties": {
///    "manifest_version": {
///      "description": "マニフェストのバージョン。現在は1のみサポートしています。\n将来的にマニフェストの仕様が変わる可能性があるため、このフィールドを使用してバージョン管理を行います。\n破壊的変更が行われた場合にのみ、この値が増加します。",
///      "type": "number",
///      "const": 1,
///      "x-rust-type": {
///        "crate": "super",
///        "path": "super::overrides::u64",
///        "version": "*"
///      }
///    },
///    "manifest_url": {
///      "description": "このマニフェストへのURL。 ない場合は、アップデートチェックが行われません。",
///      "$ref": "#/$defs/HttpUrl"
///    },
///    "id": {
///      "description": "ユーザーコンテンツの一意な識別子。/^(?<author>[a-z0-9_]+)-(?<content_name>[a-z0-9_-]+)$/ にマッチし、かつ、\n`-`が連続しない文字列である必要があります。\nここで、authorは作者名、content_nameはユーザーコンテンツの名前を表します。\nコンテンツ名では`-`と`_`をどちらも使用することができ、それらは以下のように使用するべきです。\n- `-`は概念の区切りに使用する。\n- `_`は単語の一部に使用する。\n例えば、`sevenc_nanashi-aviutl2_rs-ffmpeg_output`は：\n- 「sevenc_nanashi」という作者の、\n- 「aviutl2_rs」というものに関する、\n- 「ffmpeg_output」というユーザーコンテンツ\nを表します。\nなお、`content_name`は1単語でも構いません。",
///      "$ref": "#/$defs/ManifestId"
///    },
///    "name": {
///      "description": "ユーザーコンテンツの名前。任意の文字列を指定できます。",
///      "type": "string"
///    },
///    "summary": {
///      "description": "ユーザーコンテンツの簡易的な説明。1行で収まるようにしてください。",
///      "$ref": "#/$defs/MaybeLocalizedString"
///    },
///    "tags": {
///      "description": "ユーザーコンテンツのタグ。\nタグは自由に追加できますが、以下のタグはUIで特別に扱われます：\n- `#alias`：エイリアス\n- `#alias-effect`：フィルタ効果エイリアス\n- `#alias-object`：オブジェクトエイリアス\n- `#default`：パレット\n- `#palette`：パレット（#defaultのエイリアス）\n- `#figure`：図形\n- `#language`：言語ファイル\n- `#plugin`：プラグイン\n- `#plugin-input`：入力プラグイン\n- `#plugin-output`：出力プラグイン\n- `#script`：スクリプト\n- `#script-anm`：アニメーション効果スクリプト\n- `#script-obj`：カスタムオブジェクトスクリプト\n- `#script-cam`：カメラ制御スクリプト\n- `#script-scn`：シーンチェンジスクリプト\n- `#script-tra`：トラックバー移動方法スクリプト\n- `#theme`：テーマ\n- `#transition`：トランジション",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/MaybeLocalizedString"
///      }
///    },
///    "version": {
///      "description": "ユーザーコンテンツのバージョン。`X.Y.Z(-.+)?`の形式に従う必要があります。\n例: \"1.0.0\", \"0.1.0\", \"2.3.4-beta\"",
///      "$ref": "#/$defs/Version"
///    },
///    "version_number": {
///      "description": "ユーザーコンテンツのバージョン番号。",
///      "$ref": "#/$defs/Uint"
///    },
///    "previous_versions": {
///      "description": "過去のバージョンのマニフェストへのURL。",
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "url",
///          "version"
///        ],
///        "properties": {
///          "version": {
///            "description": "このバージョンのバージョン。",
///            "$ref": "#/$defs/Version"
///          },
///          "version_number": {
///            "description": "このバージョンのバージョン番号。",
///            "$ref": "#/$defs/Uint"
///          },
///          "url": {
///            "description": "マニフェストへのURL。",
///            "$ref": "#/$defs/HttpUrl"
///          }
///        }
///      }
///    },
///    "authors": {
///      "description": "ユーザーコンテンツの作者。",
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "name"
///        ],
///        "properties": {
///          "name": {
///            "$ref": "#/$defs/MaybeLocalizedString"
///          },
///          "url": {
///            "$ref": "#/$defs/HttpUrl"
///          }
///        }
///      },
///      "minItems": 1
///    },
///    "license": {
///      "description": "ユーザーコンテンツを使用する際の利用規約。\nこのユーザーコンテンツを使用したときの規約のみを記述してください。（例えば、再配布の規約などは含めないでください）",
///      "$ref": "#/$defs/License"
///    },
///    "homepage": {
///      "description": "ユーザーコンテンツのホームページ。",
///      "type": "string"
///    },
///    "description": {
///      "description": "ユーザーコンテンツの説明。Markdown形式で記述できます。",
///      "$ref": "#/$defs/MaybeLocalizedString"
///    },
///    "changelogs": {
///      "description": "過去のバージョンの変更履歴。",
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "changes",
///          "version"
///        ],
///        "properties": {
///          "version": {
///            "description": "変更履歴のバージョン。`version`と同じ形式である必要があります。",
///            "$ref": "#/$defs/Version"
///          },
///          "version_number": {
///            "description": "変更履歴のバージョン番号。`version_number`と同じ形式である必要があります。",
///            "$ref": "#/$defs/Uint"
///          },
///          "changes": {
///            "description": "変更内容。Markdown形式で記述できます。",
///            "$ref": "#/$defs/MaybeLocalizedString"
///          }
///        }
///      }
///    },
///    "bundles": {
///      "description": "バンドルを定義します。\n複数ファイルをまとめたアーカイブをダウンロードし、その中から必要なファイルを取り出すために使用します。\nバンドル名をキー、そのバンドルのURLを値とするオブジェクトです。\nバンドル名は`[a-z0-9_]+`にマッチする必要があります。\n今現在、以下のアーカイブ形式がサポートされています：\n- zip（`.zip`）\n- tar.gz（`.tar.gz`または`.tgz`）",
///      "$ref": "#/$defs/RecordHttpUrl",
///      "x-rust-type": {
///        "crate": "super",
///        "path": "super::overrides::Bundles",
///        "version": "*"
///      }
///    },
///    "resources": {
///      "description": "ユーザーコンテンツのダウンロードURLと、そのインストール先の一覧。",
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "destination",
///          "source"
///        ],
///        "properties": {
///          "source": {
///            "description": "このファイルの取得元。\n\n- `http://`または`https://` で始まるURL。\n- `bundle://{bundle_name}/{path}` の形式のURL。{bundle_name}はバンドル名、{path}はそのバンドル内のパスを表します。\nディレクトリを展開する場合はURLの末尾が`/`で終わる必要があります。これが一致していない場合はエラーになります。\nディレクトリを指定した場合は、そのディレクトリ内が再帰的に展開されます。",
///            "$ref": "#/$defs/SourceUrl"
///          },
///          "destination": {
///            "description": "ダウンロードしたファイルのインストール先。",
///            "$ref": "#/$defs/DataDirRelativePath"
///          },
///          "scope": {
///            "description": "このファイルのスコープを指定します。",
///            "type": "string",
///            "pattern": "^[a-z0-9_-]+$"
///          },
///          "sha256": {
///            "description": "ダウンロードしたファイルのSHA256ハッシュ値。省略可能ですが、指定することを推奨します。\n例: \"1e9211b2f7152fe7f1b4f4a3c972c8fb56845acd258f03694625d14ee516ec30\"",
///            "type": "string"
///          }
///        }
///      }
///    },
///    "configurations": {
///      "description": "このユーザーコンテンツが設定の永続化に使用するファイル。\n環境のエクスポート時にこれらのファイルが含まれます。",
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "path"
///        ],
///        "properties": {
///          "path": {
///            "description": "このファイルのパス。ディレクトリを指定することもできます。",
///            "$ref": "#/$defs/DataDirRelativePath"
///          },
///          "scope": {
///            "description": "このファイルのスコープを指定します。",
///            "type": "string",
///            "pattern": "^[a-z0-9_-]+$"
///          }
///        }
///      }
///    },
///    "disposables": {
///      "description": "このユーザーコンテンツに関する、削除可能なファイルの一覧。\nユーザーがこのユーザーコンテンツを削除したときに、これらのファイルも削除されます。\nまた、環境のエクスポート時にはこれらのファイルは含まれません。\n\nキャッシュファイルやログファイルなど、一時的なファイルを指定してください。",
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "path"
///        ],
///        "properties": {
///          "path": {
///            "description": "このファイルのパス。ディレクトリを指定することもできます。",
///            "$ref": "#/$defs/DataDirRelativePath"
///          }
///        }
///      }
///    }
///  },
///  "$schema": "https://json-schema.org/draft/2020-12/schema",
///  "$defs": {
///    "DataDirRelativePath": {
///      "description": "データディレクトリ下の特定のサブディレクトリからの相対パス。\n\n以下のいずれかのディレクトリからの相対パスである必要があります：\n- `$alias`：データディレクトリ/Alias 。\n- `$default`：データディレクトリ/Default 。\n- `$palette`：データディレクトリ/Default 。$defaultのエイリアスです。\n- `$figure`：データディレクトリ/Figure 。\n- `$language`：データディレクトリ/Language 。\n- `$plugin`：データディレクトリ/Plugin 。\n- `$script`：データディレクトリ/Script 。\n- `$transition`：データディレクトリ/Transition 。\n- `$theme`：データディレクトリ/au2ec/themes 。\n- `$data`：データディレクトリ自体。これは最終手段としてのみ使用してください。\n\nまた、`..`は含めることができません。",
///      "pattern": "^\\$(alias|default|palette|figure|language|plugin|script|transition|theme|data)/(?!.*\\.\\./).*$",
///      "type": "string",
///      "x-rust-type": {
///        "crate": "super",
///        "path": "super::overrides::DataDirRelativePathInner",
///        "version": "*"
///      }
///    },
///    "HttpUrl": {
///      "description": "http・httpsのURL。",
///      "format": "uri",
///      "type": "string",
///      "x-rust-type": {
///        "crate": "super",
///        "path": "super::overrides::HttpUrl",
///        "version": "*"
///      }
///    },
///    "License": {
///      "oneOf": [
///        {
///          "properties": {
///            "name": {
///              "const": "free",
///              "description": "このユーザーコンテンツは商用利用など含めて自由に使用できることを示します。",
///              "type": "string",
///              "x-rust-type": {
///                "crate": "super",
///                "path": "super::overrides::license::Free",
///                "version": "*"
///              }
///            },
///            "text": {
///              "$ref": "#/$defs/MaybeLocalizedString",
///              "description": "利用規約の詳細。"
///            }
///          },
///          "required": [
///            "name"
///          ],
///          "title": "Free",
///          "type": "object"
///        },
///        {
///          "properties": {
///            "id": {
///              "description": "親作品登録に使うID。`smXXXXXX`や`nmXXXXXX`の形式で指定してください。",
///              "type": "string"
///            },
///            "name": {
///              "const": "nicovideo",
///              "description": "このユーザーコンテンツはニコニコ動画での親作品登録が必要であることを示します。",
///              "type": "string",
///              "x-rust-type": {
///                "crate": "super",
///                "path": "super::overrides::license::Nicovideo",
///                "version": "*"
///              }
///            },
///            "omittable": {
///              "description": "親作品登録が必須ではないことを示します。省略した場合、親作品登録は必須です。",
///              "type": "boolean"
///            },
///            "text": {
///              "$ref": "#/$defs/MaybeLocalizedString",
///              "description": "利用規約の詳細。"
///            }
///          },
///          "required": [
///            "name",
///            "id"
///          ],
///          "title": "Nicovideo",
///          "type": "object"
///        },
///        {
///          "properties": {
///            "name": {
///              "const": "custom",
///              "description": "このユーザーコンテンツは独自の利用規約を持つことを示します。",
///              "type": "string",
///              "x-rust-type": {
///                "crate": "super",
///                "path": "super::overrides::license::Custom",
///                "version": "*"
///              }
///            },
///            "text": {
///              "$ref": "#/$defs/MaybeLocalizedString",
///              "description": "利用規約の詳細。必須です。"
///            }
///          },
///          "required": [
///            "name",
///            "text"
///          ],
///          "title": "Custom",
///          "type": "object"
///        },
///        {
///          "properties": {
///            "name": {
///              "description": "将来の拡張用。",
///              "pattern": "^(?!free$)(?!nicovideo$)(?!custom$)[a-z0-9_]+$",
///              "type": "string"
///            },
///            "text": {
///              "$ref": "#/$defs/MaybeLocalizedString",
///              "description": "少なくとも、`type`フィールドはstring?である必要があることが保証されます。"
///            }
///          },
///          "required": [
///            "name"
///          ],
///          "title": "Unknown",
///          "type": "object",
///          "unevaluatedProperties": {}
///        }
///      ]
///    },
///    "LocalizedString": {
///      "description": "多言語対応文字列。キーにロケール、値にそのロケールでの文字列を持つオブジェクト。\n例：{ ja: \"こんにちは\", en: \"Hello\" }",
///      "properties": {},
///      "type": "object",
///      "unevaluatedProperties": {
///        "type": "string"
///      }
///    },
///    "ManifestId": {
///      "pattern": "^(?<author>[a-z0-9_]+)-(?<content_name>(?:[a-z0-9_]+-)*[a-z0-9_]+)$",
///      "type": "string"
///    },
///    "MaybeLocalizedString": {
///      "anyOf": [
///        {
///          "title": "Single",
///          "type": "string"
///        },
///        {
///          "$ref": "#/$defs/LocalizedString",
///          "title": "Localized"
///        }
///      ],
///      "description": "多言語対応文字列または単一言語文字列。\n例1：\"Hello\"\n例2：{ ja: \"こんにちは\", en: \"Hello\" }"
///    },
///    "RecordHttpUrl": {
///      "properties": {},
///      "type": "object",
///      "unevaluatedProperties": {
///        "$ref": "#/$defs/HttpUrl"
///      }
///    },
///    "SourceUrl": {
///      "description": "http・https、またはbundleスキームのURL。",
///      "format": "uri",
///      "pattern": "^(https?://|bundle://[a-z0-9_]+/).*$",
///      "type": "string",
///      "x-rust-type": {
///        "crate": "super",
///        "path": "super::overrides::SourceUrl",
///        "version": "*"
///      }
///    },
///    "Uint": {
///      "description": "非負整数。",
///      "type": "integer",
///      "x-rust-type": {
///        "crate": "super",
///        "path": "super::overrides::u64",
///        "version": "*"
///      }
///    },
///    "Version": {
///      "description": "バージョン。`X.Y.Z(-.+)?`の形式に従う必要があります。\n例: \"1.0.0\", \"0.1.0\", \"2.3.4-beta\"",
///      "pattern": "^([0-9]+)\\.([0-9]+)\\.([0-9]+)(-[0-9A-Za-z-.]+)?$",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Manifest {
    ///ユーザーコンテンツの作者。
    pub authors: ::std::vec::Vec<ManifestAuthorsItem>,
    /**バンドルを定義します。
複数ファイルをまとめたアーカイブをダウンロードし、その中から必要なファイルを取り出すために使用します。
バンドル名をキー、そのバンドルのURLを値とするオブジェクトです。
バンドル名は`[a-z0-9_]+`にマッチする必要があります。
今現在、以下のアーカイブ形式がサポートされています：
- zip（`.zip`）
- tar.gz（`.tar.gz`または`.tgz`）*/
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bundles: ::std::option::Option<super::overrides::Bundles>,
    ///過去のバージョンの変更履歴。
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub changelogs: ::std::vec::Vec<ManifestChangelogsItem>,
    /**このユーザーコンテンツが設定の永続化に使用するファイル。
環境のエクスポート時にこれらのファイルが含まれます。*/
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub configurations: ::std::vec::Vec<ManifestConfigurationsItem>,
    ///ユーザーコンテンツの説明。Markdown形式で記述できます。
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<MaybeLocalizedString>,
    /**このユーザーコンテンツに関する、削除可能なファイルの一覧。
ユーザーがこのユーザーコンテンツを削除したときに、これらのファイルも削除されます。
また、環境のエクスポート時にはこれらのファイルは含まれません。

キャッシュファイルやログファイルなど、一時的なファイルを指定してください。*/
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub disposables: ::std::vec::Vec<ManifestDisposablesItem>,
    ///ユーザーコンテンツのホームページ。
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub homepage: ::std::option::Option<::std::string::String>,
    /**ユーザーコンテンツの一意な識別子。/^(?<author>[a-z0-9_]+)-(?<content_name>[a-z0-9_-]+)$/ にマッチし、かつ、
`-`が連続しない文字列である必要があります。
ここで、authorは作者名、content_nameはユーザーコンテンツの名前を表します。
コンテンツ名では`-`と`_`をどちらも使用することができ、それらは以下のように使用するべきです。
- `-`は概念の区切りに使用する。
- `_`は単語の一部に使用する。
例えば、`sevenc_nanashi-aviutl2_rs-ffmpeg_output`は：
- 「sevenc_nanashi」という作者の、
- 「aviutl2_rs」というものに関する、
- 「ffmpeg_output」というユーザーコンテンツ
を表します。
なお、`content_name`は1単語でも構いません。*/
    pub id: ManifestId,
    /**ユーザーコンテンツを使用する際の利用規約。
このユーザーコンテンツを使用したときの規約のみを記述してください。（例えば、再配布の規約などは含めないでください）*/
    pub license: License,
    ///このマニフェストへのURL。 ない場合は、アップデートチェックが行われません。
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub manifest_url: ::std::option::Option<super::overrides::HttpUrl>,
    /**マニフェストのバージョン。現在は1のみサポートしています。
将来的にマニフェストの仕様が変わる可能性があるため、このフィールドを使用してバージョン管理を行います。
破壊的変更が行われた場合にのみ、この値が増加します。*/
    pub manifest_version: super::overrides::u64,
    ///ユーザーコンテンツの名前。任意の文字列を指定できます。
    pub name: ::std::string::String,
    ///過去のバージョンのマニフェストへのURL。
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub previous_versions: ::std::vec::Vec<ManifestPreviousVersionsItem>,
    ///ユーザーコンテンツのダウンロードURLと、そのインストール先の一覧。
    pub resources: ::std::vec::Vec<ManifestResourcesItem>,
    ///ユーザーコンテンツの簡易的な説明。1行で収まるようにしてください。
    pub summary: MaybeLocalizedString,
    /**ユーザーコンテンツのタグ。
タグは自由に追加できますが、以下のタグはUIで特別に扱われます：
- `#alias`：エイリアス
- `#alias-effect`：フィルタ効果エイリアス
- `#alias-object`：オブジェクトエイリアス
- `#default`：パレット
- `#palette`：パレット（#defaultのエイリアス）
- `#figure`：図形
- `#language`：言語ファイル
- `#plugin`：プラグイン
- `#plugin-input`：入力プラグイン
- `#plugin-output`：出力プラグイン
- `#script`：スクリプト
- `#script-anm`：アニメーション効果スクリプト
- `#script-obj`：カスタムオブジェクトスクリプト
- `#script-cam`：カメラ制御スクリプト
- `#script-scn`：シーンチェンジスクリプト
- `#script-tra`：トラックバー移動方法スクリプト
- `#theme`：テーマ
- `#transition`：トランジション*/
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tags: ::std::vec::Vec<MaybeLocalizedString>,
    /**ユーザーコンテンツのバージョン。`X.Y.Z(-.+)?`の形式に従う必要があります。
例: "1.0.0", "0.1.0", "2.3.4-beta"*/
    pub version: Version,
    ///ユーザーコンテンツのバージョン番号。
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub version_number: ::std::option::Option<Uint>,
}
impl ::std::convert::From<&Manifest> for Manifest {
    fn from(value: &Manifest) -> Self {
        value.clone()
    }
}
///`ManifestAuthorsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "name"
///  ],
///  "properties": {
///    "name": {
///      "$ref": "#/$defs/MaybeLocalizedString"
///    },
///    "url": {
///      "$ref": "#/$defs/HttpUrl"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ManifestAuthorsItem {
    pub name: MaybeLocalizedString,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<super::overrides::HttpUrl>,
}
impl ::std::convert::From<&ManifestAuthorsItem> for ManifestAuthorsItem {
    fn from(value: &ManifestAuthorsItem) -> Self {
        value.clone()
    }
}
///`ManifestChangelogsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "changes",
///    "version"
///  ],
///  "properties": {
///    "version": {
///      "description": "変更履歴のバージョン。`version`と同じ形式である必要があります。",
///      "$ref": "#/$defs/Version"
///    },
///    "version_number": {
///      "description": "変更履歴のバージョン番号。`version_number`と同じ形式である必要があります。",
///      "$ref": "#/$defs/Uint"
///    },
///    "changes": {
///      "description": "変更内容。Markdown形式で記述できます。",
///      "$ref": "#/$defs/MaybeLocalizedString"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ManifestChangelogsItem {
    ///変更内容。Markdown形式で記述できます。
    pub changes: MaybeLocalizedString,
    ///変更履歴のバージョン。`version`と同じ形式である必要があります。
    pub version: Version,
    ///変更履歴のバージョン番号。`version_number`と同じ形式である必要があります。
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub version_number: ::std::option::Option<Uint>,
}
impl ::std::convert::From<&ManifestChangelogsItem> for ManifestChangelogsItem {
    fn from(value: &ManifestChangelogsItem) -> Self {
        value.clone()
    }
}
///`ManifestConfigurationsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "path"
///  ],
///  "properties": {
///    "path": {
///      "description": "このファイルのパス。ディレクトリを指定することもできます。",
///      "$ref": "#/$defs/DataDirRelativePath"
///    },
///    "scope": {
///      "description": "このファイルのスコープを指定します。",
///      "type": "string",
///      "pattern": "^[a-z0-9_-]+$"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ManifestConfigurationsItem {
    ///このファイルのパス。ディレクトリを指定することもできます。
    pub path: DataDirRelativePath,
    ///このファイルのスコープを指定します。
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub scope: ::std::option::Option<ManifestConfigurationsItemScope>,
}
impl ::std::convert::From<&ManifestConfigurationsItem> for ManifestConfigurationsItem {
    fn from(value: &ManifestConfigurationsItem) -> Self {
        value.clone()
    }
}
///このファイルのスコープを指定します。
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "このファイルのスコープを指定します。",
///  "type": "string",
///  "pattern": "^[a-z0-9_-]+$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ManifestConfigurationsItemScope(::std::string::String);
impl ::std::ops::Deref for ManifestConfigurationsItemScope {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ManifestConfigurationsItemScope> for ::std::string::String {
    fn from(value: ManifestConfigurationsItemScope) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ManifestConfigurationsItemScope>
for ManifestConfigurationsItemScope {
    fn from(value: &ManifestConfigurationsItemScope) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ManifestConfigurationsItemScope {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[a-z0-9_-]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-z0-9_-]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ManifestConfigurationsItemScope {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for ManifestConfigurationsItemScope {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ManifestConfigurationsItemScope {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ManifestConfigurationsItemScope {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`ManifestDisposablesItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "path"
///  ],
///  "properties": {
///    "path": {
///      "description": "このファイルのパス。ディレクトリを指定することもできます。",
///      "$ref": "#/$defs/DataDirRelativePath"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ManifestDisposablesItem {
    ///このファイルのパス。ディレクトリを指定することもできます。
    pub path: DataDirRelativePath,
}
impl ::std::convert::From<&ManifestDisposablesItem> for ManifestDisposablesItem {
    fn from(value: &ManifestDisposablesItem) -> Self {
        value.clone()
    }
}
///`ManifestId`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "pattern": "^(?<author>[a-z0-9_]+)-(?<content_name>(?:[a-z0-9_]+-)*[a-z0-9_]+)$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ManifestId(::std::string::String);
impl ::std::ops::Deref for ManifestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ManifestId> for ::std::string::String {
    fn from(value: ManifestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ManifestId> for ManifestId {
    fn from(value: &ManifestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ManifestId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new(
                    "^(?<author>[a-z0-9_]+)-(?<content_name>(?:[a-z0-9_]+-)*[a-z0-9_]+)$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^(?<author>[a-z0-9_]+)-(?<content_name>(?:[a-z0-9_]+-)*[a-z0-9_]+)$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ManifestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ManifestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ManifestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ManifestId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`ManifestPreviousVersionsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "url",
///    "version"
///  ],
///  "properties": {
///    "version": {
///      "description": "このバージョンのバージョン。",
///      "$ref": "#/$defs/Version"
///    },
///    "version_number": {
///      "description": "このバージョンのバージョン番号。",
///      "$ref": "#/$defs/Uint"
///    },
///    "url": {
///      "description": "マニフェストへのURL。",
///      "$ref": "#/$defs/HttpUrl"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ManifestPreviousVersionsItem {
    ///マニフェストへのURL。
    pub url: super::overrides::HttpUrl,
    ///このバージョンのバージョン。
    pub version: Version,
    ///このバージョンのバージョン番号。
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub version_number: ::std::option::Option<Uint>,
}
impl ::std::convert::From<&ManifestPreviousVersionsItem>
for ManifestPreviousVersionsItem {
    fn from(value: &ManifestPreviousVersionsItem) -> Self {
        value.clone()
    }
}
///`ManifestResourcesItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "destination",
///    "source"
///  ],
///  "properties": {
///    "source": {
///      "description": "このファイルの取得元。\n\n- `http://`または`https://` で始まるURL。\n- `bundle://{bundle_name}/{path}` の形式のURL。{bundle_name}はバンドル名、{path}はそのバンドル内のパスを表します。\nディレクトリを展開する場合はURLの末尾が`/`で終わる必要があります。これが一致していない場合はエラーになります。\nディレクトリを指定した場合は、そのディレクトリ内が再帰的に展開されます。",
///      "$ref": "#/$defs/SourceUrl"
///    },
///    "destination": {
///      "description": "ダウンロードしたファイルのインストール先。",
///      "$ref": "#/$defs/DataDirRelativePath"
///    },
///    "scope": {
///      "description": "このファイルのスコープを指定します。",
///      "type": "string",
///      "pattern": "^[a-z0-9_-]+$"
///    },
///    "sha256": {
///      "description": "ダウンロードしたファイルのSHA256ハッシュ値。省略可能ですが、指定することを推奨します。\n例: \"1e9211b2f7152fe7f1b4f4a3c972c8fb56845acd258f03694625d14ee516ec30\"",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ManifestResourcesItem {
    ///ダウンロードしたファイルのインストール先。
    pub destination: DataDirRelativePath,
    ///このファイルのスコープを指定します。
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub scope: ::std::option::Option<ManifestResourcesItemScope>,
    /**ダウンロードしたファイルのSHA256ハッシュ値。省略可能ですが、指定することを推奨します。
例: "1e9211b2f7152fe7f1b4f4a3c972c8fb56845acd258f03694625d14ee516ec30"*/
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sha256: ::std::option::Option<::std::string::String>,
    /**このファイルの取得元。

- `http://`または`https://` で始まるURL。
- `bundle://{bundle_name}/{path}` の形式のURL。{bundle_name}はバンドル名、{path}はそのバンドル内のパスを表します。
ディレクトリを展開する場合はURLの末尾が`/`で終わる必要があります。これが一致していない場合はエラーになります。
ディレクトリを指定した場合は、そのディレクトリ内が再帰的に展開されます。*/
    pub source: super::overrides::SourceUrl,
}
impl ::std::convert::From<&ManifestResourcesItem> for ManifestResourcesItem {
    fn from(value: &ManifestResourcesItem) -> Self {
        value.clone()
    }
}
///このファイルのスコープを指定します。
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "このファイルのスコープを指定します。",
///  "type": "string",
///  "pattern": "^[a-z0-9_-]+$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ManifestResourcesItemScope(::std::string::String);
impl ::std::ops::Deref for ManifestResourcesItemScope {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ManifestResourcesItemScope> for ::std::string::String {
    fn from(value: ManifestResourcesItemScope) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ManifestResourcesItemScope> for ManifestResourcesItemScope {
    fn from(value: &ManifestResourcesItemScope) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ManifestResourcesItemScope {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[a-z0-9_-]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-z0-9_-]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ManifestResourcesItemScope {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ManifestResourcesItemScope {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ManifestResourcesItemScope {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ManifestResourcesItemScope {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
/**多言語対応文字列または単一言語文字列。
例1："Hello"
例2：{ ja: "こんにちは", en: "Hello" }*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "多言語対応文字列または単一言語文字列。\n例1：\"Hello\"\n例2：{ ja: \"こんにちは\", en: \"Hello\" }",
///  "anyOf": [
///    {
///      "title": "Single",
///      "type": "string"
///    },
///    {
///      "title": "Localized",
///      "$ref": "#/$defs/LocalizedString"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum MaybeLocalizedString {
    Single(::std::string::String),
    LocalizedString(LocalizedString),
}
impl ::std::convert::From<&Self> for MaybeLocalizedString {
    fn from(value: &MaybeLocalizedString) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<LocalizedString> for MaybeLocalizedString {
    fn from(value: LocalizedString) -> Self {
        Self::LocalizedString(value)
    }
}
///`RecordHttpUrl`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "unevaluatedProperties": {
///    "$ref": "#/$defs/HttpUrl"
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RecordHttpUrl(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for RecordHttpUrl {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<RecordHttpUrl>
for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
    fn from(value: RecordHttpUrl) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RecordHttpUrl> for RecordHttpUrl {
    fn from(value: &RecordHttpUrl) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
for RecordHttpUrl {
    fn from(
        value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Self {
        Self(value)
    }
}
///`Registry`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$id": "Registry.json",
///  "title": "Registry",
///  "type": "object",
///  "required": [
///    "contents",
///    "name",
///    "registry_version",
///    "summary"
///  ],
///  "properties": {
///    "registry_version": {
///      "description": "レジストリのバージョン。現在は1のみサポートしています。\n破壊的変更が行われた場合にのみ、この値が増加します。",
///      "type": "number",
///      "const": 1,
///      "x-rust-type": {
///        "crate": "std",
///        "path": "std::u64",
///        "version": "*"
///      }
///    },
///    "name": {
///      "description": "レジストリの名前。",
///      "type": "string"
///    },
///    "summary": {
///      "description": "このレジストリの説明。",
///      "$ref": "#/$defs/MaybeLocalizedString"
///    },
///    "homepage": {
///      "description": "このレジストリのホームページ。",
///      "$ref": "#/$defs/HttpUrl"
///    },
///    "contents": {
///      "description": "このレジストリに含まれるユーザーコンテンツの一覧。",
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "authors",
///          "id",
///          "license",
///          "manifest_url",
///          "manifest_version",
///          "name",
///          "summary",
///          "version"
///        ],
///        "properties": {
///          "manifest_version": {
///            "description": "マニフェストのバージョン。現在は1のみサポートしています。\n将来的にマニフェストの仕様が変わる可能性があるため、このフィールドを使用してバージョン管理を行います。\n破壊的変更が行われた場合にのみ、この値が増加します。",
///            "type": "number",
///            "const": 1,
///            "x-rust-type": {
///              "crate": "super",
///              "path": "super::overrides::u64",
///              "version": "*"
///            }
///          },
///          "id": {
///            "description": "ユーザーコンテンツの一意な識別子。/^(?<author>[a-z0-9_]+)-(?<content_name>[a-z0-9_-]+)$/ にマッチし、かつ、\n`-`が連続しない文字列である必要があります。\nここで、authorは作者名、content_nameはユーザーコンテンツの名前を表します。\nコンテンツ名では`-`と`_`をどちらも使用することができ、それらは以下のように使用するべきです。\n- `-`は概念の区切りに使用する。\n- `_`は単語の一部に使用する。\n例えば、`sevenc_nanashi-aviutl2_rs-ffmpeg_output`は：\n- 「sevenc_nanashi」という作者の、\n- 「aviutl2_rs」というものに関する、\n- 「ffmpeg_output」というユーザーコンテンツ\nを表します。\nなお、`content_name`は1単語でも構いません。",
///            "$ref": "#/$defs/ManifestId"
///          },
///          "name": {
///            "description": "ユーザーコンテンツの名前。任意の文字列を指定できます。",
///            "type": "string"
///          },
///          "summary": {
///            "description": "ユーザーコンテンツの簡易的な説明。1行で収まるようにしてください。",
///            "$ref": "#/$defs/MaybeLocalizedString"
///          },
///          "tags": {
///            "description": "ユーザーコンテンツのタグ。\nタグは自由に追加できますが、以下のタグはUIで特別に扱われます：\n- `#alias`：エイリアス\n- `#alias-effect`：フィルタ効果エイリアス\n- `#alias-object`：オブジェクトエイリアス\n- `#default`：パレット\n- `#palette`：パレット（#defaultのエイリアス）\n- `#figure`：図形\n- `#language`：言語ファイル\n- `#plugin`：プラグイン\n- `#plugin-input`：入力プラグイン\n- `#plugin-output`：出力プラグイン\n- `#script`：スクリプト\n- `#script-anm`：アニメーション効果スクリプト\n- `#script-obj`：カスタムオブジェクトスクリプト\n- `#script-cam`：カメラ制御スクリプト\n- `#script-scn`：シーンチェンジスクリプト\n- `#script-tra`：トラックバー移動方法スクリプト\n- `#theme`：テーマ\n- `#transition`：トランジション",
///            "type": "array",
///            "items": {
///              "$ref": "#/$defs/MaybeLocalizedString"
///            }
///          },
///          "version": {
///            "description": "ユーザーコンテンツのバージョン。`X.Y.Z(-.+)?`の形式に従う必要があります。\n例: \"1.0.0\", \"0.1.0\", \"2.3.4-beta\"",
///            "$ref": "#/$defs/Version"
///          },
///          "version_number": {
///            "description": "ユーザーコンテンツのバージョン番号。",
///            "$ref": "#/$defs/Uint"
///          },
///          "authors": {
///            "description": "ユーザーコンテンツの作者。",
///            "type": "array",
///            "items": {
///              "type": "object",
///              "required": [
///                "name"
///              ],
///              "properties": {
///                "name": {
///                  "$ref": "#/$defs/MaybeLocalizedString"
///                },
///                "url": {
///                  "$ref": "#/$defs/HttpUrl"
///                }
///              }
///            },
///            "minItems": 1
///          },
///          "license": {
///            "description": "ユーザーコンテンツを使用する際の利用規約。\nこのユーザーコンテンツを使用したときの規約のみを記述してください。（例えば、再配布の規約などは含めないでください）",
///            "$ref": "#/$defs/License"
///          },
///          "homepage": {
///            "description": "ユーザーコンテンツのホームページ。",
///            "type": "string"
///          },
///          "description": {
///            "description": "ユーザーコンテンツの説明。Markdown形式で記述できます。",
///            "$ref": "#/$defs/MaybeLocalizedString"
///          },
///          "manifest_url": {
///            "description": "このユーザーコンテンツのマニフェストへのURL。\n単体のマニフェストとは違い、このフィールドは必須です。",
///            "type": "string"
///          }
///        }
///      }
///    }
///  },
///  "$schema": "https://json-schema.org/draft/2020-12/schema",
///  "$defs": {
///    "HttpUrl": {
///      "description": "http・httpsのURL。",
///      "format": "uri",
///      "type": "string",
///      "x-rust-type": {
///        "crate": "super",
///        "path": "super::overrides::HttpUrl",
///        "version": "*"
///      }
///    },
///    "License": {
///      "oneOf": [
///        {
///          "properties": {
///            "name": {
///              "const": "free",
///              "description": "このユーザーコンテンツは商用利用など含めて自由に使用できることを示します。",
///              "type": "string",
///              "x-rust-type": {
///                "crate": "super",
///                "path": "super::overrides::license::Free",
///                "version": "*"
///              }
///            },
///            "text": {
///              "$ref": "#/$defs/MaybeLocalizedString",
///              "description": "利用規約の詳細。"
///            }
///          },
///          "required": [
///            "name"
///          ],
///          "title": "Free",
///          "type": "object"
///        },
///        {
///          "properties": {
///            "id": {
///              "description": "親作品登録に使うID。`smXXXXXX`や`nmXXXXXX`の形式で指定してください。",
///              "type": "string"
///            },
///            "name": {
///              "const": "nicovideo",
///              "description": "このユーザーコンテンツはニコニコ動画での親作品登録が必要であることを示します。",
///              "type": "string",
///              "x-rust-type": {
///                "crate": "super",
///                "path": "super::overrides::license::Nicovideo",
///                "version": "*"
///              }
///            },
///            "omittable": {
///              "description": "親作品登録が必須ではないことを示します。省略した場合、親作品登録は必須です。",
///              "type": "boolean"
///            },
///            "text": {
///              "$ref": "#/$defs/MaybeLocalizedString",
///              "description": "利用規約の詳細。"
///            }
///          },
///          "required": [
///            "name",
///            "id"
///          ],
///          "title": "Nicovideo",
///          "type": "object"
///        },
///        {
///          "properties": {
///            "name": {
///              "const": "custom",
///              "description": "このユーザーコンテンツは独自の利用規約を持つことを示します。",
///              "type": "string",
///              "x-rust-type": {
///                "crate": "super",
///                "path": "super::overrides::license::Custom",
///                "version": "*"
///              }
///            },
///            "text": {
///              "$ref": "#/$defs/MaybeLocalizedString",
///              "description": "利用規約の詳細。必須です。"
///            }
///          },
///          "required": [
///            "name",
///            "text"
///          ],
///          "title": "Custom",
///          "type": "object"
///        },
///        {
///          "properties": {
///            "name": {
///              "description": "将来の拡張用。",
///              "pattern": "^(?!free$)(?!nicovideo$)(?!custom$)[a-z0-9_]+$",
///              "type": "string"
///            },
///            "text": {
///              "$ref": "#/$defs/MaybeLocalizedString",
///              "description": "少なくとも、`type`フィールドはstring?である必要があることが保証されます。"
///            }
///          },
///          "required": [
///            "name"
///          ],
///          "title": "Unknown",
///          "type": "object",
///          "unevaluatedProperties": {}
///        }
///      ]
///    },
///    "LocalizedString": {
///      "description": "多言語対応文字列。キーにロケール、値にそのロケールでの文字列を持つオブジェクト。\n例：{ ja: \"こんにちは\", en: \"Hello\" }",
///      "properties": {},
///      "type": "object",
///      "unevaluatedProperties": {
///        "type": "string"
///      }
///    },
///    "ManifestId": {
///      "pattern": "^(?<author>[a-z0-9_]+)-(?<content_name>(?:[a-z0-9_]+-)*[a-z0-9_]+)$",
///      "type": "string"
///    },
///    "MaybeLocalizedString": {
///      "anyOf": [
///        {
///          "title": "Single",
///          "type": "string"
///        },
///        {
///          "$ref": "#/$defs/LocalizedString",
///          "title": "Localized"
///        }
///      ],
///      "description": "多言語対応文字列または単一言語文字列。\n例1：\"Hello\"\n例2：{ ja: \"こんにちは\", en: \"Hello\" }"
///    },
///    "Uint": {
///      "description": "非負整数。",
///      "type": "integer",
///      "x-rust-type": {
///        "crate": "super",
///        "path": "super::overrides::u64",
///        "version": "*"
///      }
///    },
///    "Version": {
///      "description": "バージョン。`X.Y.Z(-.+)?`の形式に従う必要があります。\n例: \"1.0.0\", \"0.1.0\", \"2.3.4-beta\"",
///      "pattern": "^([0-9]+)\\.([0-9]+)\\.([0-9]+)(-[0-9A-Za-z-.]+)?$",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Registry {
    ///このレジストリに含まれるユーザーコンテンツの一覧。
    pub contents: ::std::vec::Vec<RegistryContentsItem>,
    ///このレジストリのホームページ。
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub homepage: ::std::option::Option<super::overrides::HttpUrl>,
    ///レジストリの名前。
    pub name: ::std::string::String,
    pub registry_version: f64,
    ///このレジストリの説明。
    pub summary: MaybeLocalizedString,
}
impl ::std::convert::From<&Registry> for Registry {
    fn from(value: &Registry) -> Self {
        value.clone()
    }
}
///`RegistryContentsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "authors",
///    "id",
///    "license",
///    "manifest_url",
///    "manifest_version",
///    "name",
///    "summary",
///    "version"
///  ],
///  "properties": {
///    "manifest_version": {
///      "description": "マニフェストのバージョン。現在は1のみサポートしています。\n将来的にマニフェストの仕様が変わる可能性があるため、このフィールドを使用してバージョン管理を行います。\n破壊的変更が行われた場合にのみ、この値が増加します。",
///      "type": "number",
///      "const": 1,
///      "x-rust-type": {
///        "crate": "super",
///        "path": "super::overrides::u64",
///        "version": "*"
///      }
///    },
///    "id": {
///      "description": "ユーザーコンテンツの一意な識別子。/^(?<author>[a-z0-9_]+)-(?<content_name>[a-z0-9_-]+)$/ にマッチし、かつ、\n`-`が連続しない文字列である必要があります。\nここで、authorは作者名、content_nameはユーザーコンテンツの名前を表します。\nコンテンツ名では`-`と`_`をどちらも使用することができ、それらは以下のように使用するべきです。\n- `-`は概念の区切りに使用する。\n- `_`は単語の一部に使用する。\n例えば、`sevenc_nanashi-aviutl2_rs-ffmpeg_output`は：\n- 「sevenc_nanashi」という作者の、\n- 「aviutl2_rs」というものに関する、\n- 「ffmpeg_output」というユーザーコンテンツ\nを表します。\nなお、`content_name`は1単語でも構いません。",
///      "$ref": "#/$defs/ManifestId"
///    },
///    "name": {
///      "description": "ユーザーコンテンツの名前。任意の文字列を指定できます。",
///      "type": "string"
///    },
///    "summary": {
///      "description": "ユーザーコンテンツの簡易的な説明。1行で収まるようにしてください。",
///      "$ref": "#/$defs/MaybeLocalizedString"
///    },
///    "tags": {
///      "description": "ユーザーコンテンツのタグ。\nタグは自由に追加できますが、以下のタグはUIで特別に扱われます：\n- `#alias`：エイリアス\n- `#alias-effect`：フィルタ効果エイリアス\n- `#alias-object`：オブジェクトエイリアス\n- `#default`：パレット\n- `#palette`：パレット（#defaultのエイリアス）\n- `#figure`：図形\n- `#language`：言語ファイル\n- `#plugin`：プラグイン\n- `#plugin-input`：入力プラグイン\n- `#plugin-output`：出力プラグイン\n- `#script`：スクリプト\n- `#script-anm`：アニメーション効果スクリプト\n- `#script-obj`：カスタムオブジェクトスクリプト\n- `#script-cam`：カメラ制御スクリプト\n- `#script-scn`：シーンチェンジスクリプト\n- `#script-tra`：トラックバー移動方法スクリプト\n- `#theme`：テーマ\n- `#transition`：トランジション",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/MaybeLocalizedString"
///      }
///    },
///    "version": {
///      "description": "ユーザーコンテンツのバージョン。`X.Y.Z(-.+)?`の形式に従う必要があります。\n例: \"1.0.0\", \"0.1.0\", \"2.3.4-beta\"",
///      "$ref": "#/$defs/Version"
///    },
///    "version_number": {
///      "description": "ユーザーコンテンツのバージョン番号。",
///      "$ref": "#/$defs/Uint"
///    },
///    "authors": {
///      "description": "ユーザーコンテンツの作者。",
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "name"
///        ],
///        "properties": {
///          "name": {
///            "$ref": "#/$defs/MaybeLocalizedString"
///          },
///          "url": {
///            "$ref": "#/$defs/HttpUrl"
///          }
///        }
///      },
///      "minItems": 1
///    },
///    "license": {
///      "description": "ユーザーコンテンツを使用する際の利用規約。\nこのユーザーコンテンツを使用したときの規約のみを記述してください。（例えば、再配布の規約などは含めないでください）",
///      "$ref": "#/$defs/License"
///    },
///    "homepage": {
///      "description": "ユーザーコンテンツのホームページ。",
///      "type": "string"
///    },
///    "description": {
///      "description": "ユーザーコンテンツの説明。Markdown形式で記述できます。",
///      "$ref": "#/$defs/MaybeLocalizedString"
///    },
///    "manifest_url": {
///      "description": "このユーザーコンテンツのマニフェストへのURL。\n単体のマニフェストとは違い、このフィールドは必須です。",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RegistryContentsItem {
    ///ユーザーコンテンツの作者。
    pub authors: ::std::vec::Vec<RegistryContentsItemAuthorsItem>,
    ///ユーザーコンテンツの説明。Markdown形式で記述できます。
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<MaybeLocalizedString>,
    ///ユーザーコンテンツのホームページ。
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub homepage: ::std::option::Option<::std::string::String>,
    /**ユーザーコンテンツの一意な識別子。/^(?<author>[a-z0-9_]+)-(?<content_name>[a-z0-9_-]+)$/ にマッチし、かつ、
`-`が連続しない文字列である必要があります。
ここで、authorは作者名、content_nameはユーザーコンテンツの名前を表します。
コンテンツ名では`-`と`_`をどちらも使用することができ、それらは以下のように使用するべきです。
- `-`は概念の区切りに使用する。
- `_`は単語の一部に使用する。
例えば、`sevenc_nanashi-aviutl2_rs-ffmpeg_output`は：
- 「sevenc_nanashi」という作者の、
- 「aviutl2_rs」というものに関する、
- 「ffmpeg_output」というユーザーコンテンツ
を表します。
なお、`content_name`は1単語でも構いません。*/
    pub id: ManifestId,
    /**ユーザーコンテンツを使用する際の利用規約。
このユーザーコンテンツを使用したときの規約のみを記述してください。（例えば、再配布の規約などは含めないでください）*/
    pub license: License,
    /**このユーザーコンテンツのマニフェストへのURL。
単体のマニフェストとは違い、このフィールドは必須です。*/
    pub manifest_url: ::std::string::String,
    /**マニフェストのバージョン。現在は1のみサポートしています。
将来的にマニフェストの仕様が変わる可能性があるため、このフィールドを使用してバージョン管理を行います。
破壊的変更が行われた場合にのみ、この値が増加します。*/
    pub manifest_version: super::overrides::u64,
    ///ユーザーコンテンツの名前。任意の文字列を指定できます。
    pub name: ::std::string::String,
    ///ユーザーコンテンツの簡易的な説明。1行で収まるようにしてください。
    pub summary: MaybeLocalizedString,
    /**ユーザーコンテンツのタグ。
タグは自由に追加できますが、以下のタグはUIで特別に扱われます：
- `#alias`：エイリアス
- `#alias-effect`：フィルタ効果エイリアス
- `#alias-object`：オブジェクトエイリアス
- `#default`：パレット
- `#palette`：パレット（#defaultのエイリアス）
- `#figure`：図形
- `#language`：言語ファイル
- `#plugin`：プラグイン
- `#plugin-input`：入力プラグイン
- `#plugin-output`：出力プラグイン
- `#script`：スクリプト
- `#script-anm`：アニメーション効果スクリプト
- `#script-obj`：カスタムオブジェクトスクリプト
- `#script-cam`：カメラ制御スクリプト
- `#script-scn`：シーンチェンジスクリプト
- `#script-tra`：トラックバー移動方法スクリプト
- `#theme`：テーマ
- `#transition`：トランジション*/
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tags: ::std::vec::Vec<MaybeLocalizedString>,
    /**ユーザーコンテンツのバージョン。`X.Y.Z(-.+)?`の形式に従う必要があります。
例: "1.0.0", "0.1.0", "2.3.4-beta"*/
    pub version: Version,
    ///ユーザーコンテンツのバージョン番号。
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub version_number: ::std::option::Option<Uint>,
}
impl ::std::convert::From<&RegistryContentsItem> for RegistryContentsItem {
    fn from(value: &RegistryContentsItem) -> Self {
        value.clone()
    }
}
///`RegistryContentsItemAuthorsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "name"
///  ],
///  "properties": {
///    "name": {
///      "$ref": "#/$defs/MaybeLocalizedString"
///    },
///    "url": {
///      "$ref": "#/$defs/HttpUrl"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RegistryContentsItemAuthorsItem {
    pub name: MaybeLocalizedString,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<super::overrides::HttpUrl>,
}
impl ::std::convert::From<&RegistryContentsItemAuthorsItem>
for RegistryContentsItemAuthorsItem {
    fn from(value: &RegistryContentsItemAuthorsItem) -> Self {
        value.clone()
    }
}
///非負整数。
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "非負整数。",
///  "type": "integer",
///  "x-rust-type": {
///    "crate": "super",
///    "path": "super::overrides::u64",
///    "version": "*"
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Uint(pub super::overrides::u64);
impl ::std::ops::Deref for Uint {
    type Target = super::overrides::u64;
    fn deref(&self) -> &super::overrides::u64 {
        &self.0
    }
}
impl ::std::convert::From<Uint> for super::overrides::u64 {
    fn from(value: Uint) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Uint> for Uint {
    fn from(value: &Uint) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<super::overrides::u64> for Uint {
    fn from(value: super::overrides::u64) -> Self {
        Self(value)
    }
}
///将来の拡張用。
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "将来の拡張用。",
///  "type": "string",
///  "pattern": "^(?!free$)(?!nicovideo$)(?!custom$)[a-z0-9_]+$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct UnknownName(::std::string::String);
impl ::std::ops::Deref for UnknownName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<UnknownName> for ::std::string::String {
    fn from(value: UnknownName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UnknownName> for UnknownName {
    fn from(value: &UnknownName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for UnknownName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new("^(?!free$)(?!nicovideo$)(?!custom$)[a-z0-9_]+$")
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^(?!free$)(?!nicovideo$)(?!custom$)[a-z0-9_]+$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for UnknownName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for UnknownName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UnknownName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for UnknownName {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
/**バージョン。`X.Y.Z(-.+)?`の形式に従う必要があります。
例: "1.0.0", "0.1.0", "2.3.4-beta"*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "バージョン。`X.Y.Z(-.+)?`の形式に従う必要があります。\n例: \"1.0.0\", \"0.1.0\", \"2.3.4-beta\"",
///  "type": "string",
///  "pattern": "^([0-9]+)\\.([0-9]+)\\.([0-9]+)(-[0-9A-Za-z-.]+)?$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Version(::std::string::String);
impl ::std::ops::Deref for Version {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Version> for ::std::string::String {
    fn from(value: Version) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Version> for Version {
    fn from(value: &Version) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Version {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new("^([0-9]+)\\.([0-9]+)\\.([0-9]+)(-[0-9A-Za-z-.]+)?$")
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^([0-9]+)\\.([0-9]+)\\.([0-9]+)(-[0-9A-Za-z-.]+)?$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Version {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Version {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Version {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
