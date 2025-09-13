import { App, inject, ref, InjectionKey } from "vue";

export type GeneralDialogType =
  | "info"
  | "warning"
  | "danger"
  | "error"
  | "success"
  | undefined;
export type GeneralDialogOptions = {
  title: string | undefined;
  message: string;
  type?: GeneralDialogType;
  allowDismiss?: boolean;
  onDismiss?: () => void;
  actions: {
    label: string;
    color?: "primary" | "success" | "warning" | "danger" | "error" | undefined;
    onClick?: () => Promise<void> | void;
  }[];
};
export type AskingDialogOptions<T> = {
  title: string | undefined;
  message: string;
  type?: GeneralDialogType;
  actions: {
    label: string;
    color?: "primary" | "success" | "warning" | "danger" | "error" | undefined;
    value: T;
  }[];
};
export type LodaingDialogOptions = {
  message: string;
};

export type DialogState = {
  closing: boolean;
  id: number;
} & (
  | {
      type: "general";
      options: GeneralDialogOptions;
    }
  | {
      type: "loading";
      options: LodaingDialogOptions;
    }
);

export type DialogContext = {
  open: (options: GeneralDialogOptions) => number;
  list: () => DialogState[];
  close: (id: number) => void;
  remove: (id: number) => void;

  loading: (message: string) => { [Symbol.dispose](): void };
  ask: <T>(options: AskingDialogOptions<T>) => Promise<T | null>;
};

const dialogContextKey = Symbol("dialog") as InjectionKey<DialogContext>;

export function useDialog() {
  const context = inject(dialogContextKey, null);
  if (!context) {
    throw new Error("useDialog must be used within a DialogProvider");
  }
  return context;
}

class Counter {
  private count = 0;

  next() {
    return this.count++;
  }
}

export function dialogPlugin(app: App) {
  const dialogs = ref<DialogState[]>([]);
  const idCounter = new Counter();
  app.provide(dialogContextKey, {
    open(options: GeneralDialogOptions) {
      const id = idCounter.next();
      dialogs.value.push({
        id,
        closing: false,
        type: "general",
        options,
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
      const id = idCounter.next();
      dialogs.value.push({
        id,
        closing: false,
        type: "loading",
        options: { message },
      });

      return {
        [Symbol.dispose]: () => {
          this.close(id);
        },
      };
    },
    ask<T>(options: AskingDialogOptions<T>) {
      return new Promise<T | null>((resolve) => {
        const id = idCounter.next();
        const onAction = (value: T | null) => {
          resolve(value);
        };
        dialogs.value.push({
          id,
          closing: false,
          type: "general",
          options: {
            title: options.title,
            message: options.message,
            type: options.type,
            allowDismiss: true,
            onDismiss: () => onAction(null),
            actions: options.actions.map((action) => ({
              label: action.label,
              color: action.color,
              onClick: () => onAction(action.value),
            })),
          },
        });
      });
    },
  });
}
