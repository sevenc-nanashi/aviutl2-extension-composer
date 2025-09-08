import { App, inject, ref, InjectionKey } from "vue";

export type DialogOptions = {
  title: string;
  message: string;
  type?: "info" | "warning" | "error" | "success";
  actions: {
    label: string;
    color?: "primary" | "secondary" | "danger";
    onClick: () => void;
  }[];
};

export type DialogState = DialogOptions & {
  id: number;
};

export type DialogContext = {
  open: (options: DialogOptions) => void;
  list: () => DialogState[];
  close: (id: number) => void;
};

const dialogContextKey = Symbol("dialog") as InjectionKey<DialogContext>;

export function useDialog() {
  const context = inject(dialogContextKey, null);
  if (!context) {
    throw new Error("useDialog must be used within a DialogProvider");
  }
  return context;
}

export function dialogPlugin(app: App) {
  const dialogs = ref<DialogState[]>([]);
  let idCounter = 0;
  app.provide(dialogContextKey, {
    open: (options: DialogOptions) => {
      const id = idCounter++;
      dialogs.value.push({ ...options, id });
      return id;
    },
    list: () => dialogs.value,
    close: (id: number) => {
      dialogs.value = dialogs.value.filter((dialog) => dialog.id !== id);
    },
  });
}
