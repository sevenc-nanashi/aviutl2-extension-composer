import { Composer } from "vue-i18n";
import { MaybeLocalizedString } from "./models/Registry.ts";

export function useMaybeLocalized(
  i18n: Composer,
): (t: MaybeLocalizedString) => string {
  return (t: MaybeLocalizedString): string => {
    if (typeof t === "string") {
      return t;
    } else {
      return (t[i18n.locale.value] ??
        t["en"] ??
        Object.values(t)[0] ??
        "") as string;
    }
  };
}
