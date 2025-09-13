import "@fontsource/noto-sans-jp/japanese";
import "@fontsource/noto-sans-jp/japanese-700.css";
import "virtual:uno.css";
import "./styles/reset.css";
import "./styles/style.scss";
import { createApp } from "vue";
import App from "./App.vue";
import { dialogPlugin } from "./plugins/dialog.ts";
import { i18n } from "./plugins/i18n.ts";
import { router } from "./plugins/router.ts";
import { toastPlugin } from "./plugins/toast.ts";

createApp(App)
  .use(router)
  .use(dialogPlugin)
  .use(toastPlugin)
  .use(i18n)
  .mount("#app");
