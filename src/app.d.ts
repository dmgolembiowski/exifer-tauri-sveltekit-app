/// <reference types="@sveltejs/kit" />

// See https://kit.svelte.dev/docs/types#the-app-namespace
// for information about these interfaces
declare namespace App {
  // interface Locals {}
  // interface Platform {}
  // interface Session {}
  // interface Stuff {}
}

type Dict<T> = Record<string, T>;

declare const __TAURI__: {
  invoke: typeof import('@tauri-apps/api/tauri').invoke;
}
