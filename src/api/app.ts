import { invoke } from "@tauri-apps/api/core";
import type { ApiKeySummary, ProviderSummary } from "../types/domain";

export interface AppInfo {
  version: string;
  dataDirectory: string;
  logDirectory: string;
}

export interface UpdateInfo {
  currentVersion: string;
  latestVersion: string;
  title: string;
  notes: string;
  releaseUrl: string;
  prerelease: boolean;
  publishedAt?: string;
  releaseTag: string;
}

export async function getAppInfo(): Promise<AppInfo | null> {
  if (!("__TAURI_INTERNALS__" in window)) return null;
  return invoke<AppInfo>("get_app_info");
}

function desktopInvoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  if (!("__TAURI_INTERNALS__" in window)) return Promise.reject(new Error("请在桌面应用中使用本地数据功能"));
  return invoke<T>(command, args);
}
export const listProviders = () => desktopInvoke<ProviderSummary[]>("list_providers");
export const checkForAppUpdates = () => desktopInvoke<UpdateInfo | null>("check_for_updates");
export const installAppUpdate = (releaseTag: string) => desktopInvoke<void>("install_update", { releaseTag });
export const openDataDirectory = () => desktopInvoke<void>("open_data_directory");
export const openLogDirectory = () => desktopInvoke<void>("open_log_directory");
export const clearLogs = () => desktopInvoke<void>("clear_logs");
export const createProvider = (input: Omit<ProviderSummary, "keys">) => desktopInvoke<ProviderSummary>("create_provider", { input });
export const updateProvider = (input: { id: string; name: string; platformUrl?: string }) => desktopInvoke<ProviderSummary>("update_provider", { input });
export const deleteProvider = (providerId: string) => desktopInvoke<void>("delete_provider", { providerId });
export const reorderProviders = (providerIds: string[]) => desktopInvoke<void>("reorder_providers", { providerIds });
export const createApiKey = (input: { providerId: string; remark: string; value: string }) => desktopInvoke<ApiKeySummary>("create_api_key", { input });
export const updateApiKey = (input: { id: string; remark: string; value: string }) => desktopInvoke<ApiKeySummary>("update_api_key", { input });
export const copyApiKey = (keyId: string) => desktopInvoke<void>("copy_api_key", { keyId });
export const deleteApiKey = (keyId: string) => desktopInvoke<void>("delete_api_key", { keyId });
export const checkApiKey = (providerId: string, keyId: string) => desktopInvoke<ApiKeySummary>("check_api_key", { providerId, keyId });
export const checkProviderKeys = (providerId: string) => desktopInvoke<ApiKeySummary[]>("check_provider_keys", { providerId });
