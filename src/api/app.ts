import { invoke } from "@tauri-apps/api/core";
import type { ApiKeySummary, ProviderSummary, ProviderValidation } from "../types/domain";
import type { AppErrorPayload } from "../i18n/errors";

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

export type ClientLogLevel = "INFO" | "WARN" | "ERROR";

export interface ClientLogInput {
  level: ClientLogLevel;
  event: string;
  detail?: string;
}

export const SETTINGS_SCHEMA_VERSION = 1;

export interface AppSettings {
  schemaVersion: number;
  localePreference: string;
  themePreference: "system" | "light" | "dark";
}

export function isDesktopApp(): boolean {
  return "__TAURI_INTERNALS__" in window;
}

export async function getAppInfo(): Promise<AppInfo | null> {
  if (!isDesktopApp()) return null;
  return desktopInvoke<AppInfo>("get_app_info");
}

function errorCodeForLog(error: unknown): string {
  if (typeof error === "object" && error !== null && "code" in error) {
    const code = (error as { code?: unknown }).code;
    if (typeof code === "string" && /^[A-Z0-9_]{1,64}$/.test(code)) return code;
  }
  if (typeof error === "string") {
    try {
      return errorCodeForLog(JSON.parse(error));
    } catch {
      return "UNKNOWN";
    }
  }
  if (error instanceof Error && error.name) return error.name.replace(/[^A-Za-z0-9_]/g, "_").slice(0, 64);
  return "UNKNOWN";
}

export function writeClientLog(input: ClientLogInput): Promise<void> {
  if (!isDesktopApp()) return Promise.resolve();
  return invoke<void>("write_client_log", { input });
}

function desktopInvoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  if (!isDesktopApp()) {
    return Promise.reject({ code: "DESKTOP_REQUIRED" } satisfies AppErrorPayload);
  }
  return invoke<T>(command, args).catch((error: unknown) => {
    if (command !== "write_client_log") {
      void writeClientLog({
        level: "ERROR",
        event: "tauri_command_failed",
        detail: `command=${command} code=${errorCodeForLog(error)}`,
      }).catch(() => undefined);
    }
    throw error;
  });
}
export const loadAppSettings = (legacyLocalePreference?: string) => desktopInvoke<AppSettings>(
  "load_app_settings",
  { legacyLocalePreference: legacyLocalePreference ?? null },
);
export const saveAppSettings = (settings: AppSettings) => desktopInvoke<AppSettings>("save_app_settings", { settings });
export const listProviders = () => desktopInvoke<ProviderSummary[]>("list_providers");
export const checkForAppUpdates = () => desktopInvoke<UpdateInfo | null>("check_for_updates");
export const installAppUpdate = (releaseTag: string) => desktopInvoke<void>("install_update", { releaseTag });
export const openDataDirectory = () => desktopInvoke<void>("open_data_directory");
export const openLogDirectory = () => desktopInvoke<void>("open_log_directory");
export const clearLogs = () => desktopInvoke<void>("clear_logs");
export const createProvider = (input: Omit<ProviderSummary, "keys" | "validationSupported">) => desktopInvoke<ProviderSummary>("create_provider", { input });
export const updateProvider = (input: { id: string; name: string; platformUrl?: string; validation: ProviderValidation }) => desktopInvoke<ProviderSummary>("update_provider", { input });
export const deleteProvider = (providerId: string) => desktopInvoke<void>("delete_provider", { providerId });
export const reorderProviders = (providerIds: string[]) => desktopInvoke<void>("reorder_providers", { providerIds });
export const createApiKey = (input: { providerId: string; remark: string; value: string }) => desktopInvoke<ApiKeySummary>("create_api_key", { input });
export const updateApiKey = (input: { id: string; remark: string; value: string }) => desktopInvoke<ApiKeySummary>("update_api_key", { input });
export const copyApiKey = (keyId: string) => desktopInvoke<void>("copy_api_key", { keyId });
export const deleteApiKey = (keyId: string) => desktopInvoke<void>("delete_api_key", { keyId });
export const checkApiKey = (providerId: string, keyId: string) => desktopInvoke<ApiKeySummary>("check_api_key", { providerId, keyId });
export const checkProviderKeys = (providerId: string) => desktopInvoke<ApiKeySummary[]>("check_provider_keys", { providerId });
