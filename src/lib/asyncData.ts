import { ref } from "vue";

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

export function useAsync<T>(asyncFunction: () => Promise<T>) {
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
