import { i18n } from ".";

export const appErrorTranslationKeys = {
  DESKTOP_REQUIRED: "errors.desktopRequired",
  DATA_DIRECTORY_UNAVAILABLE: "errors.dataDirectoryUnavailable",
  LOG_DIRECTORY_UNAVAILABLE: "errors.logDirectoryUnavailable",
  DIRECTORY_CREATE_FAILED: "errors.directoryCreateFailed",
  DIRECTORY_OPEN_FAILED: "errors.directoryOpenFailed",
  DATA_READ_FAILED: "errors.dataReadFailed",
  DATA_INVALID: "errors.dataInvalid",
  DATA_SAVE_FAILED: "errors.dataSaveFailed",
  KEYRING_UNAVAILABLE: "errors.keyringUnavailable",
  KEYRING_READ_FAILED: "errors.keyringReadFailed",
  KEYRING_WRITE_FAILED: "errors.keyringWriteFailed",
  KEYRING_DELETE_FAILED: "errors.keyringDeleteFailed",
  NETWORK_CLIENT_FAILED: "errors.networkClientFailed",
  VERSION_INVALID: "errors.versionInvalid",
  UPDATE_CHECK_FAILED: "errors.updateCheckFailed",
  UPDATE_RESPONSE_TOO_LARGE: "errors.updateResponseTooLarge",
  UPDATE_DATA_INVALID: "errors.updateDataInvalid",
  UPDATE_TAG_INVALID: "errors.updateTagInvalid",
  UPDATE_MANIFEST_FAILED: "errors.updateManifestFailed",
  UPDATE_DOWNLOAD_TIMEOUT: "errors.updateDownloadTimeout",
  UPDATE_DOWNLOAD_FAILED: "errors.updateDownloadFailed",
  UPDATE_INSTALL_FAILED: "errors.updateInstallFailed",
  UPDATE_NOT_AVAILABLE: "errors.updateNotAvailable",
  PROVIDER_NAME_REQUIRED: "errors.providerNameRequired",
  PROVIDER_EXISTS: "errors.providerExists",
  PROVIDER_NAME_EXISTS: "errors.providerNameExists",
  PROVIDER_NOT_FOUND: "errors.providerNotFound",
  PROVIDER_ORDER_INVALID: "errors.providerOrderInvalid",
  API_KEY_REQUIRED: "errors.apiKeyRequired",
  API_KEY_NOT_FOUND: "errors.apiKeyNotFound",
  CLIPBOARD_WRITE_FAILED: "errors.clipboardWriteFailed",
  SETTINGS_READ_FAILED: "errors.settingsReadFailed",
  SETTINGS_INVALID: "errors.settingsInvalid",
  SETTINGS_SAVE_FAILED: "errors.settingsSaveFailed",
  LOG_UNAVAILABLE: "errors.logUnavailable",
  LOG_CLEAR_FAILED: "errors.logClearFailed",
  UNKNOWN: "errors.unknown",
} as const;

export type AppErrorCode = keyof typeof appErrorTranslationKeys;

export interface AppErrorPayload {
  code: AppErrorCode;
}

export function getAppErrorCode(error: unknown): AppErrorCode {
  if (typeof error === "object" && error !== null && "code" in error) {
    const code = (error as { code?: unknown }).code;
    if (typeof code === "string" && code in appErrorTranslationKeys) return code as AppErrorCode;
  }
  if (typeof error === "string") {
    try {
      return getAppErrorCode(JSON.parse(error));
    } catch {
      return "UNKNOWN";
    }
  }
  return "UNKNOWN";
}

export function translateAppError(error: unknown, fallbackKey?: string): string {
  const code = getAppErrorCode(error);
  const key = code === "UNKNOWN" && fallbackKey ? fallbackKey : appErrorTranslationKeys[code];
  return i18n.global.t(key);
}
