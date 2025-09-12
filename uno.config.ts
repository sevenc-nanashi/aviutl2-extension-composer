import { defineConfig, transformerDirectives } from "unocss";
import presetAttributify from "unocss/preset-attributify";
import presetIcons from "unocss/preset-icons";
import presetWind3 from "unocss/preset-wind3";

export default defineConfig({
  presets: [
    presetAttributify({
      prefix: "un-",
      prefixedOnly: true,
    }),
    presetIcons(),
    presetWind3(),
  ],
  theme: {
    fontFamily: {
      sans: ['"Noto Sans JP"', "serif"],
    },
  },
  transformers: [transformerDirectives()],
  rules: [
    ["align-content-center", { "align-content": "center" }],
    ["w-main", { width: "clamp(60vw,600px,90vw)" }],
  ],
});
