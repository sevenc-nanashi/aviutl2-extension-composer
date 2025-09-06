export type Brand<T, B> = T & { __brand: B };
export type LocalizedString = { [locale: string]: string };
export type MaybeLocalizedString = string | { [locale: string]: string };
export interface ExtensionShallowMetadata {
  id: string;
  name: LocalizedString;
  description?: LocalizedString;
  version: string;
  author: {
    name: LocalizedString;
    url?: string;
  };
  logo?: string;
  categories?: (LocalizedString | string)[];
}
