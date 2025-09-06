import { defineConfig } from "unocss";
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
});
