import { invoke } from "@tauri-apps/api/core";

export interface AppInfo {
  version: string;
  dataDirectory: string;
}

export async function getAppInfo(): Promise<AppInfo | null> {
  if (!("__TAURI_INTERNALS__" in window)) return null;
  return invoke<AppInfo>("get_app_info");
}
