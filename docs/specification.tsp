import "@typespec/json-schema";
using TypeSpec.JsonSchema;

// NOTE: aliasではなくscalar extendsやmodel isを使っているのは型を`$defs`に出力させるため。

// #region common
/**
 * 多言語対応文字列。キーにロケール、値にそのロケールでの文字列を持つオブジェクト。
 * 例：{ ja: "こんにちは", en: "Hello" }
 */
model LocalizedString is Record<string>;

/**
 * 多言語対応文字列または単一言語文字列。
 * 例1："Hello"
 * 例2：{ ja: "こんにちは", en: "Hello" }
 */
union MaybeLocalizedString {
  @extension("title", "Single")
  single: string,

  @extension("title", "Localized")
  localized: LocalizedString,
}

/**
 * 非網羅的なオブジェクトを表す。
 * この型は将来的に拡張される可能性があることを示すために使用します。
 */
model NonExhaustive is Record<unknown>;

/**
 * ファイルパス。
 */
scalar FilePath extends string;

/**
 * Globパターン。
 * `*`、`**`に対応しています。
 */
scalar GlobPattern extends string;

/**
 * データディレクトリ下の特定のサブディレクトリからの相対パス。
 *
 * 以下のいずれかのディレクトリからの相対パスである必要があります：
 * - `$alias`：データディレクトリ/Alias 。
 * - `$default`：データディレクトリ/Default 。
 * - `$palette`：データディレクトリ/Default 。$defaultのエイリアスです。
 * - `$figure`：データディレクトリ/Figure 。
 * - `$language`：データディレクトリ/Language 。
 * - `$plugin`：データディレクトリ/Plugin 。
 * - `$script`：データディレクトリ/Script 。
 * - `$transition`：データディレクトリ/Transition 。
 * - `$theme`：データディレクトリ/au2ec/themes 。
 * - `$data`：データディレクトリ自体。これは最終手段としてのみ使用してください。
 *
 * また、`..`は含めることができません。
 */
@extension(
  "pattern",
  "^\\$(alias|default|palette|figure|language|plugin|script|transition|theme|data)/(?!.*\\.\\./).*$"
)
scalar DataDirRelativePath extends string;

/**
 * GlobPatternのDataDirRelativePath版。
 */
scalar DataDirRelativeGlobPattern extends string;

/**
 * http・httpsのURL。
 */
scalar HttpUrl extends url;

// #endregion common

// NOTE: `@extension("title", "...")`がないとルートのモデルをtypifyが出力してくれないので治す

// #region manifest
@jsonSchema
@extension("title", "Manifest")
model Manifest {
  /**
   * マニフェストのバージョン。現在は1のみサポートしています。
   * 将来的にマニフェストの仕様が変わる可能性があるため、このフィールドを使用してバージョン管理を行います。
   * 破壊的変更が行われた場合にのみ、この値が増加します。
   */
  @extension("x-rust-type", #{ crate: "super", version: "*", path: "super::num::u64" })
  manifest_version: 1;

  /**
   * このマニフェストへのURL。 ない場合は、アップデートチェックが行われません。
   */
  manifest_url?: HttpUrl;

  /**
   * ユーザーコンテンツの一意な識別子。/^(?<author>[a-z0-9_]+)-(?<content_name>[a-z0-9_-]+)$/ にマッチし、かつ、
   * `-`が連続しない文字列である必要があります。
   * ここで、authorは作者名、content_nameはユーザーコンテンツの名前を表します。
   * コンテンツ名では`-`と`_`をどちらも使用することができ、それらは以下のように使用するべきです。
   * - `-`は概念の区切りに使用する。
   * - `_`は単語の一部に使用する。
   * 例えば、`sevenc_nanashi-aviutl2_rs-ffmpeg_output`は：
   * - 「sevenc_nanashi」という作者の、
   * - 「aviutl2_rs」というものに関する、
   * - 「ffmpeg_output」というユーザーコンテンツ
   * を表します。
   * なお、`content_name`は1単語でも構いません。
   */
  id: ManifestId;

  /**
   * ユーザーコンテンツの名前。任意の文字列を指定できます。
   */
  name: string;

  /**
   * ユーザーコンテンツの簡易的な説明。1行で収まるようにしてください。
   */
  summary: MaybeLocalizedString;

  /**
   * ユーザーコンテンツのタグ。
   * タグは自由に追加できますが、以下のタグはUIで特別に扱われます：
   * - `#alias`：エイリアス
   * - `#alias-effect`：フィルタ効果エイリアス
   * - `#alias-object`：オブジェクトエイリアス
   * - `#default`：パレット
   * - `#palette`：パレット（#defaultのエイリアス）
   * - `#figure`：図形
   * - `#language`：言語ファイル
   * - `#plugin`：プラグイン
   * - `#plugin-input`：入力プラグイン
   * - `#plugin-output`：出力プラグイン
   * - `#script`：スクリプト
   * - `#script-anm`：アニメーション効果スクリプト
   * - `#script-obj`：カスタムオブジェクトスクリプト
   * - `#script-cam`：カメラ制御スクリプト
   * - `#script-scn`：シーンチェンジスクリプト
   * - `#script-tra`：トラックバー移動方法スクリプト
   * - `#theme`：テーマ
   * - `#transition`：トランジション
   */
  tags?: Array<MaybeLocalizedString>;

  /**
   * ユーザーコンテンツのバージョン。`X.Y.Z(-.+)?`の形式に従う必要があります。
   * 例: "1.0.0", "0.1.0", "2.3.4-beta"
   */
  version: string;

  /**
   * ユーザーコンテンツのバージョン番号。
   * バージョンを比較するために使用します。必ずしも連番である必要はありませんが、バージョンが新しくなるほど大きな値になっている必要があります。
   * 同じversion_numberを持つバージョンが存在した場合は未定義です。
   *
   * このフィールドがないバージョンは常にversion_numberがあるバージョンよりも古いものとして扱われます。
   */
  @extension("x-rust-type", #{ crate: "super", version: "*", path: "super::num::u64" })
  version_number?: uint64;

  /**
   * 過去のバージョンのマニフェストへのURL。
   */
  previous_versions?: Array<{
    /** マニフェストのバージョン。 */
    version: string;

    /** マニフェストのバージョン番号。 */
    @extension("x-rust-type", #{ crate: "super", version: "*", path: "super::num::u64" })
    version_number?: uint64;

    /** マニフェストへのURL。 */
    url: HttpUrl;
  }>;

  /**
   * ユーザーコンテンツの作者。
   */
  authors: Array<{
    name: MaybeLocalizedString;
    url?: HttpUrl;
  }>;

  /**
   * ユーザーコンテンツを使用する際の利用規約。
   * このユーザーコンテンツを使用したときの規約のみを記述してください。（例えば、再配布の規約などは含めないでください）
   */
  license: License;

  /**
   * ユーザーコンテンツのホームページ。
   */
  homepage?: string;

  /**
   * ユーザーコンテンツの説明。Markdown形式で記述できます。
   */
  description?: MaybeLocalizedString;

  /**
   * 過去のバージョンの変更履歴。
   */
  changelogs?: Array<{
    /** 変更履歴のバージョン。`version`と同じ形式である必要があります。 */
    version: string;

    /** 変更履歴のバージョン番号。`version_number`と同じ形式である必要があります。 */
    @extension("x-rust-type", #{ crate: "std", version: "*", path: "std::u64" })
    version_number?: uint64;

    /** 変更内容。Markdown形式で記述できます。 */
    changes: MaybeLocalizedString;
  }>;

  /**
   * バンドルを定義します。
   * 複数ファイルをまとめたアーカイブをダウンロードし、その中から必要なファイルを取り出すために使用します。
   * バンドル名をキー、そのバンドルのURLを値とするオブジェクトです。
   * バンドル名は`[a-z0-9_]+`にマッチする必要があります。
   * 今現在、以下のアーカイブ形式がサポートされています：
   * - zip（`.zip`）
   * - tar.gz（`.tar.gz`または`.tgz`）
   */
  bundles?: Record<HttpUrl>;

  /**
   * ユーザーコンテンツのダウンロードURL。
   */
  resources: Array<{
    /**
     * このファイルの取得元。
     *
     * - `http://`または`https://` で始まるURL。
     * - `bundle://{bundle_name}/{path_or_glob}` の形式のURL。{bundle_name}はバンドル名、{path_or_glob}はバンドル内のファイルパスまたはGlobパターンを表します。
     *  globパターンを使用した場合、destinationはディレクトリとして扱われ、その中にマッチしたファイルがすべてダウンロードされます。
     */
    source: url;

    /**
     * ダウンロードしたファイルのインストール先。
     */
    destination: DataDirRelativePath;

    /**
     * このファイルのスコープを指定します。
     * スコープは`[a-z0-9_-]+`にマッチする必要があります。
     *
     * 通常、別のユーザーコンテンツと同じファイルが存在した場合、コンテンツの競合として扱われ、インストールが失敗します。
     * しかし、スコープを指定することで、同じファイルが存在ても、同じスコープである場合に限り、競合を無視してインストールできるようになります。
     * 例えば、スコープを`ffmpeg`に設定した場合、他のユーザーコンテンツに同じファイルが存在しても、そのファイルのスコープが`ffmpeg`であれば競合を無視します。
     */
    scope?: string;

    /**
     * ダウンロードしたファイルのSHA256ハッシュ値。省略可能ですが、指定することを推奨します。
     * 例: "1e9211b2f7152fe7f1b4f4a3c972c8fb56845acd258f03694625d14ee516ec30"
     */
    sha256?: string;
  }>;

  /**
   * このユーザーコンテンツが設定の永続化に使用するファイル。
   * 環境のエクスポート時にこれらのファイルが含まれます。
   */
  configurations?: Array<{
    /** この設定ファイルのパスまたはGlobパターン。 */
    path: DataDirRelativeGlobPattern;
  }>;

  /**
   * このユーザーコンテンツに関する、削除可能なファイルの一覧。
   * ユーザーがこのユーザーコンテンツを削除したときに、これらのファイルも削除されます。
   * また、環境のエクスポート時にはこれらのファイルは含まれません。
   *
   * 例えば、キャッシュファイルや一時ファイルなどを指定します。
   */
  disposables?: Array<{
    /** このファイルのパスまたはGlobパターン。 */
    path: DataDirRelativeGlobPattern;
  }>;
}

@extension(
  "pattern",
  "^(?<author>[a-z0-9_]+)-(?<content_name>(?:[a-z0-9_]+-)*[a-z0-9_]+)$"
)
scalar ManifestId extends string;

@oneOf
union License {
  @extension("title", "Free")
  free: {
    /** このユーザーコンテンツは商用利用など含めて自由に使用できることを示します。 */
    @extension(
      "x-rust-type",
      #{ crate: "super", version: "*", path: "super::license::Free" }
    )
    name: "free",

    /** 利用規約の詳細。 */
    text?: MaybeLocalizedString,
  },

  @extension("title", "Nicovideo")
  nicovideo: {
    /** このユーザーコンテンツはニコニコ動画での親作品登録が必要であることを示します。 */
    @extension(
      "x-rust-type",
      #{ crate: "super", version: "*", path: "super::license::Nicovideo" }
    )
    name: "nicovideo",

    /** 親作品登録が必須ではないことを示します。省略した場合、親作品登録は必須です。 */
    omittable?: boolean,

    /** 親作品登録に使うID。`smXXXXXX`や`nmXXXXXX`の形式で指定してください。 */
    id: string,

    /** 利用規約の詳細。 */
    text?: MaybeLocalizedString,
  },

  @extension("title", "Custom")
  custom: {
    /** このユーザーコンテンツは独自の利用規約を持つことを示します。 */
    @extension(
      "x-rust-type",
      #{ crate: "super", version: "*", path: "super::license::Custom" }
    )
    name: "custom",

    /** 利用規約の詳細。必須です。 */
    text: MaybeLocalizedString,
  },

  @extension("title", "Unknown")
  `unknown`: {
    /** 将来の拡張用。 */
    name: string,

    /** 少なくとも、`type`フィールドはstring?である必要があることが保証されます。 */
    text?: MaybeLocalizedString,

    ...NonExhaustive,
  },
}

// #endregion manifest

// #region registry
@jsonSchema
@extension("title", "Registry")
model Registry {
  /**
   * レジストリのバージョン。現在は1のみサポートしています。
   * 破壊的変更が行われた場合にのみ、この値が増加します。
   */
  @extension("x-rust-type", #{ crate: "std", version: "*", path: "std::u64" })
  registry_version: 1;

  /**
   * レジストリの名前。
   */
  name: string;

  /**
   * このレジストリのホームページ。
   */
  homepage?: string;

  /**
   * このレジストリに含まれるユーザーコンテンツの一覧。
   */
  contents: Array<PickProperties<
    Manifest,

      | "manifest_version"
      | "id"
      | "name"
      | "summary"
      | "tags"
      | "version"
      | "version_number"
      | "authors"
      | "license"
      | "homepage"
      | "description"
  > & {
    /**
     * このユーザーコンテンツのマニフェストへのURL。
     * 単体のマニフェストとは違い、このフィールドは必須です。
     */
    manifest_url: string;
  }>;
}
// #endregion registry
