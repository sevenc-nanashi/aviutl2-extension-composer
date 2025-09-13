import { Ref, ref, UnwrapRef } from "vue";

export type AsyncState<T> =
  | {
      state: "loading";
    }
  | {
      state: "error";
      error: Error;
    }
  | {
      state: "success";
      data: T;
    };

export type RefreshableAsyncState<T> = AsyncState<T> & {
  refresh: () => Promise<void>;
};

export function useAsync<T>(
  asyncFunction: () => Promise<T>,
): Ref<AsyncState<UnwrapRef<T>>> {
  const result = ref<AsyncState<T>>({ state: "loading" });

  asyncFunction()
    .then((data) => {
      result.value = { state: "success", data };
    })
    .catch((error) => {
      result.value = { state: "error", error };
    });

  return result;
}

export function useRefreshableAsync<T>(asyncFunction: () => Promise<T>): {
  readonly value: AsyncState<UnwrapRef<T>>;
  refresh: (this: void) => Promise<void>;
} {
  const result = ref<AsyncState<T>>({ state: "loading" });

  const refresh = async () => {
    result.value = { state: "loading" };
    try {
      const data = await asyncFunction();
      result.value = { state: "success", data };
    } catch (error) {
      result.value = { state: "error", error: error as Error };
    }
  };

  // Initial load
  refresh();

  return {
    get value() {
      return result.value;
    },
    refresh,
  };
}
