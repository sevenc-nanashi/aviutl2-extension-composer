# 仕様・定義

> [!WARNING]
> まだドラフト段階です。変更される可能性があります。

## 読み方

この型定義は[TypeSpec](https://typespec.io)で記述されています。
また、この型定義のJSON Schemaは`<TODO>`に存在します。

以下の共通型定義が存在します：

<<< ./specification.tsp#common

### バージョンについて

バージョンは`version`と`version_number`フィールドの二つで表されます。

- `version`は人間が読むための文字列です。`^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z-.]+)?$`の形式である必要があります。
- `version_number`はバージョンを正確に比較するための数値です。非負整数である必要があり、また、数値はすべてのバージョンで一意である必要があります。

比較は以下のように行われます（A、Bというバージョンがあるとします）：

1. Aの`version_number`が`null`でBの`version_number`が非`null`である場合、Bの方が新しい。逆も同様。
2. A、Bの両方の`version_number`が非`null`である場合、`version_number`の数値を比較し、大きい方が新しい。
3. A、Bの両方の`version_number`が`null`である場合、`version`の文字列を以下のように比較する：
   1. `version`を`.`で分割し、各部分を数値として比較する。
   2. 数値が等しい場合、`-`以降（プレリリース部分）を辞書順で比較する。もし片方のみプレリリース部分が存在する場合、プレリリース部分が存在しない方が新しいとみなす。

### データディレクトリ

AviUtl2の環境設定ディレクトリ。`C:\ProgramData\AviUtl2`や、`aviutl2.exe`と同じディレクトリにある`data`フォルダなどを指します。

### 競合・スコープについて

通常、別のユーザーコンテンツと同じファイルパスにインストールしようとした場合、インストールが失敗します：

```yaml
# ユーザーコンテンツA
resources:
  - destination: $plugin/sample.auo2
    source: https://example.com/a/sample.auo2
  - destination: $data/common_lib.dll
    source: https://example.com/a/common_lib.dll

# ユーザーコンテンツB（競合発生）
resources:
  - destination: $plugin/sample2.auo2
    source: https://example.com/b/sample.auo2
  - destination: $data/common_lib.dll
    source: https://example.com/b/common_lib.dll
```

#### スコープによる競合回避

スコープを指定することで、同じファイルパスでも異なるスコープ間では競合を無視し、同じスコープ内でのみ競合をチェックできます。これにより、関連するコンポーネント群を一つのスコープとして管理できます。

```yaml
# ユーザーコンテンツA
resources:
  - destination: $plugin/sample.auo2
    source: https://example.com/a/sample.auo2
  - destination: $data/common_lib.dll
    source: https://example.com/a/common_lib.dll
    scope: me-common_lib

# ユーザーコンテンツB（競合しない）
resources:
  - destination: $plugin/sample2.auo2
    source: https://example.com/b/sample.auo2
  - destination: $data/common_lib.dll
    source: https://example.com/b/common_lib.dll
    scope: me-common_lib
```

ただし、スコープが同じでも、`resources`・`configurations`・`disposables`間では競合します：

```yaml
# ユーザーコンテンツA
resources:
  - destination: $data/common_lib.dll
    source: https://example.com/a/common_lib.dll
    scope: me-common_lib

# ユーザーコンテンツB（競合発生）
configurations:
  - destination: $data/common_lib.dll
    scope: me-common_lib
```

#### バンドルの競合

バンドルを使用してディレクトリを展開する場合、そのディレクトリ下のすべてのファイルパスが競合対象となります：

```yaml
# ユーザーコンテンツA
bundles:
  my_bundle: https://example.com/bundle.zip

resources:
  # バンドルをディレクトリに展開
  - source: bundle://my_bundle/
    destination: $data/some_dir/

# ユーザーコンテンツB
resources:
  - source: https://example.com/other_file.txt
    destination: $data/some_dir/my_file.txt  # 競合する
```

### may-follow-link

「may-follow-link」と宣言されているものをURLで指定する場合、以下の条件を満たす場合に限り、リンク先のURLをたどってもよいものとします：

- リンク先の`Content-Type`ヘッダーが`text/html`である。
- 以下のいずれかの条件を満たす：
  - リンク先のHTMLにて、セレクタ `html > head > link[rel="alternate" type="application/yaml+aviutl2-extension-composer"]` にマッチする`link`要素が存在する。
    この場合、`link`要素の`href`属性のURLの先をマニフェストとして扱います。
  - リンク先のHTMLのいずれかの`pre > code`で表せる、かつそのinnerHTMLが正規表現 `/\s*aviutl2-extension-composer:alternate:(?<url>https?:\/\/[^\s]+)\s*/` にマッチするものが存在する。
    この場合、マッチした`url`グループのURLの先をマニフェストとして扱います。

プラグインの紹介ページやリポジトリのREADMEをマニフェストのURLとして使えるようにすることが目的です。

## ユーザーコンテンツ

プラグイン、スクリプト、カスタムオブジェクト、言語ファイルなどの、AviUtl2の環境設定に入れる、かつ複数持てるもの。
例外として、テーマもユーザーコンテンツに含まれます。本来`style.conf`はAviUtl2の環境設定で一つしか持てませんが、AviUtl2 Extension Composerでは複数のテーマを管理できます。

## マニフェスト

ユーザーコンテンツのインストール先や、バージョンなどを記述したyamlファイル。
may-follow-linkです。

::: code-group

<<<./specification.tsp#manifest [manifest.tsp]
<<<./public/schema/Manifest.json [manifest.schema.json]
<<<./examples/manifest.yml [example_manifest.yaml]

:::

## レジストリ

マニフェストの一部を切り出したものを集めたもの。ユーザーコンテンツの検索や、アップデートチェックに使用します。
may-follow-linkです。

::: code-group

<<<./specification.tsp#registry
<<<./public/schema/Registry.json [registry.schema.json]
<<<./examples/registry.yml [example_registry.yaml]

:::
