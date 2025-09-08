import "@fontsource/noto-sans-jp/japanese";
import "@fontsource/noto-sans-jp/japanese-700.css";
import "virtual:uno.css";
import "./styles/reset.css";
import "./styles/style.scss";
import { createApp } from "vue";
import App from "./App.vue";
import { router } from "./router.ts";

createApp(App).use(router).mount("#app");
