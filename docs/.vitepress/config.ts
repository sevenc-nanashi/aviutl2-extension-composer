import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "AviUtl2 Extension Composer",
  description: "AviUtl2のプラグイン・スクリプト・テーマを一括管理するツール",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: "Home", link: "/" },
      { text: "Examples", link: "/markdown-examples" },
    ],

    sidebar: [
      {
        text: "仕様",
        link: "/specification",
      },
    ],

    socialLinks: [
      {
        icon: "github",
        link: "https://github.com/sevenc-nanashi/aviutl2-extension-composer",
      },
    ],

    externalLinkIcon: true,
    skipToContentLabel: "内容にジャンプ",
    langMenuLabel: "言語",
    lastUpdatedText: "最終更新",
    sidebarMenuLabel: "メニュー",
    notFound: {
      title: "ページが見つかりません",
      linkLabel: "トップページに戻る",
      linkText: "トップページへ",
      quote: "お探しのページは見つかりませんでした。",
    },
    lightModeSwitchTitle: "ライトモードに切り替え",
    darkModeSwitchLabel: "外観",
    darkModeSwitchTitle: "ダークモードに切り替え",
    returnToTopLabel: "上までスクロール",
    outline: {
      label: "このページの内容",
      level: [2, 3],
    },

    docFooter: {
      prev: "前へ",
      next: "次へ",
    },
  },
});
