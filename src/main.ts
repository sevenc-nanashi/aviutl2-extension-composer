import "@fontsource/noto-sans-jp/japanese";
import "@fontsource/noto-sans-jp/japanese-700.css";
import "virtual:uno.css";
import "./styles/reset.css";
import "./styles/style.scss";
import { createApp } from "vue";
import App from "./App.vue";
import { dialogPlugin } from "./lib/dialog.ts";
import { router } from "./plugins/router.ts";
import { i18n } from "./plugins/i18n.ts";

createApp(App).use(router).use(dialogPlugin).use(i18n).mount("#app");
