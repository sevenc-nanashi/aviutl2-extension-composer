import { Composer } from "vue-i18n";

export class UnreachableError extends Error {
  constructor(message?: string) {
    super(message ?? "This code should be unreachable");
    this.name = "UnreachableError";
  }
}

export class ExhaustiveError extends Error {
  constructor(value: never, message?: string) {
    super(message ?? `Unhandled case: ${JSON.stringify(value, null, 2)}`);
    this.name = "ExhaustiveError";
  }
}

export function errorToLocalizedString(i18n: Composer, error: unknown): string {
  if (error instanceof Error) {
    if (error.message.startsWith("#")) {
      return i18n.t(`errors.${error.message.slice(1)}`);
    }
    return i18n.t("unknownError", { message: error.message });
  } else if (typeof error === "string") {
    if (error.startsWith("#")) {
      return i18n.t(`errors.${error.slice(1)}`);
    }
    return i18n.t("unknownError", { message: error });
  } else {
    try {
      const str = JSON.stringify(error);
      return i18n.t("unknownError", { message: str });
    } catch {
      return i18n.t("unknownError", { message: String(error) });
    }
  }
}

export function errorToString(error: unknown): string {
  if (error instanceof Error) {
    return error.message;
  } else if (typeof error === "string") {
    return error;
  } else {
    try {
      return JSON.stringify(error);
    } catch {
      return String(error);
    }
  }
}
