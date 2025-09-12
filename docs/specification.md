# 仕様・定義

> [!WARNING]
> まだドラフト段階です。変更される可能性があります。

## 読み方

この型定義は[TypeSpec](https://typespec.io)で記述されています。
また、この型定義のJSON Schemaは`<TODO>`に存在します。

以下の共通型定義が存在します：

<<< ./specification.tsp#common

## データディレクトリ

AviUtl2の環境設定ディレクトリ。`C:\ProgramData\AviUtl2`や、`aviutl2.exe`と同じディレクトリにある`data`フォルダなどを指します。

## may-follow-link

「may-follow-link」と宣言されているものをURLで指定する場合、以下の条件を満たす場合に限り、リンク先のURLをマニフェストのURLとして扱うことができます。

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
