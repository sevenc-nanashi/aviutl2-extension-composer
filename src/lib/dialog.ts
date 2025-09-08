import { App, inject, ref, InjectionKey } from "vue";

export type DialogOptions = {
  title: string | undefined;
  message: string;
  type?: "info" | "warning" | "error" | "success";
  actions: {
    label: string;
    color?: "primary" | "secondary" | "danger";
    onClick?: () => void;
  }[];
};

export type DialogState = DialogOptions & {
  closing: boolean;
  id: number;
};

export type DialogContext = {
  open: (options: DialogOptions) => number;
  list: () => DialogState[];
  close: (id: number) => void;
  remove: (id: number) => void;

  loading: (message: string) => { [Symbol.dispose](): void };
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
    open(options: DialogOptions) {
      const id = idCounter++;
      dialogs.value.push({
        id,
        closing: false,
        ...options,
      });
      return id;
    },
    list() {
      return dialogs.value;
    },
    close(id: number) {
      const dialog = dialogs.value.find((dialog) => dialog.id === id);
      if (dialog) {
        dialog.closing = true;
      }
    },
    remove(id: number) {
      dialogs.value = dialogs.value.filter((dialog) => dialog.id !== id);
    },

    loading(message: string) {
      const id = this.open({
        title: undefined,
        message,
        type: "info",
        actions: [],
      });
      return {
        [Symbol.dispose]: () => {
          this.close(id);
        },
      };
    },
  });
}
