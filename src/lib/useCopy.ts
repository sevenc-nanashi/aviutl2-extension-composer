import { useI18n } from "vue-i18n";
import { useToast } from "../plugins/toast.ts";

export function useCopy(): (text: string) => void {
  const toast = useToast();
  const { t } = useI18n();

  return (text: string): void => {
    toast.open({
      message: t("copiedToClipboard"),
      type: "success",
    });
    void navigator.clipboard.writeText(text);
  };
}
