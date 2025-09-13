import { Composer } from "vue-i18n";
import { useToast } from "../plugins/toast.ts";

export function useCopy(t: Composer["t"]): (text: string) => void {
  const toast = useToast();

  return (text: string): void => {
    toast.open({
      message: t("copiedToClipboard"),
      type: "success",
    });
    void navigator.clipboard.writeText(text);
  };
}
