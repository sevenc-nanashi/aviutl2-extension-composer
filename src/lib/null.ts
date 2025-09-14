export function ensureNotNullish<T>(
  value: T | null | undefined,
  message = "Value is nullish",
): T {
  if (value === null || value === undefined) {
    throw new Error(message);
  }
  return value;
}
