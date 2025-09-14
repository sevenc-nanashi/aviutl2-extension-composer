import { App, inject, ref, InjectionKey } from "vue";

export type ToastType =
  | "info"
  | "warning"
  | "danger"
  | "error"
  | "success"
  | undefined;
export type ToastOptions = {
  message: string;
  color?: ToastType;
  duration?: number;
};

export type ToastState = {
  closing: boolean;
  id: number;
  message: string;
  type: ToastType;
  duration: number;
};

export type ToastContext = {
  open: (options: ToastOptions) => number;
  list: () => ToastState[];
  close: (id: number) => void;
  remove: (id: number) => void;
};

const toastContextKey = Symbol("toast") as InjectionKey<ToastContext>;

export function useToast() {
  const context = inject(toastContextKey, null);
  if (!context) {
    throw new Error("useToast must be used within a ToastProvider");
  }
  return context;
}

class Counter {
  private count = 0;

  next() {
    return this.count++;
  }
}

export function toastPlugin(app: App) {
  const toasts = ref<ToastState[]>([]);
  const idCounter = new Counter();
  app.provide(toastContextKey, {
    open(options: ToastOptions) {
      const id = idCounter.next();
      const duration = options.duration ?? 3000;
      toasts.value.push({
        id,
        closing: false,
        message: options.message,
        type: options.color,
        duration,
      });
      setTimeout(() => this.close(id), duration);
      return id;
    },
    list() {
      return toasts.value;
    },
    close(id: number) {
      const toast = toasts.value.find((toast) => toast.id === id);
      if (toast) {
        toast.closing = true;
      }
    },
    remove(id: number) {
      toasts.value = toasts.value.filter((toast) => toast.id !== id);
    },
  });
}
