import { computed, onScopeDispose, ref } from "vue";
import { defineStore } from "pinia";
import {
  isDesktopApp,
  loadAppSettings,
  saveAppSettings,
  setAppWindowTheme,
  SETTINGS_SCHEMA_VERSION,
  type AppSettings,
} from "../api/app";
import { setLocalePreference as applyLocalePreference } from "../i18n";
import {
  clearLegacyLocalePreference,
  isLocalePreference,
  readLegacyLocalePreference,
  type LocalePreference,
} from "../i18n/locale";

export type ThemePreference = "system" | "light" | "dark";

export function isThemePreference(value: unknown): value is ThemePreference {
  return value === "system" || value === "light" || value === "dark";
}

async function applyThemePreference(preference: ThemePreference): Promise<void> {
  const resolvedTheme = preference === "system"
    ? (window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light")
    : preference;
  document.documentElement.dataset.theme = resolvedTheme;
  document.documentElement.style.colorScheme = resolvedTheme;
  try {
    await setAppWindowTheme(preference === "system" ? null : preference);
  } catch {
    // WebView 主题仍可正常使用；原生窗口主题失败已写入运行日志。
  }
}

function normalizeSettings(settings: AppSettings): AppSettings {
  return {
    schemaVersion: SETTINGS_SCHEMA_VERSION,
    localePreference: isLocalePreference(settings.localePreference)
      ? settings.localePreference
      : "system",
    themePreference: isThemePreference(settings.themePreference)
      ? settings.themePreference
      : "system",
  };
}

export const useSettingsStore = defineStore("settings", () => {
  const legacyLocalePreference = readLegacyLocalePreference();
  const initialSettings: AppSettings = {
    schemaVersion: SETTINGS_SCHEMA_VERSION,
    localePreference: legacyLocalePreference ?? "system",
    themePreference: "system",
  };
  const settings = ref<AppSettings>(initialSettings);
  const loaded = ref(false);
  const localePreference = computed<LocalePreference>(() => (
    isLocalePreference(settings.value.localePreference)
      ? settings.value.localePreference
      : "system"
  ));
  const themePreference = computed<ThemePreference>(() => (
    isThemePreference(settings.value.themePreference)
      ? settings.value.themePreference
      : "system"
  ));

  const systemThemeMedia = window.matchMedia("(prefers-color-scheme: dark)");
  const handleSystemThemeChange = () => {
    if (themePreference.value === "system") void applyThemePreference("system");
  };
  systemThemeMedia.addEventListener("change", handleSystemThemeChange);
  onScopeDispose(() => systemThemeMedia.removeEventListener("change", handleSystemThemeChange));

  let persistedSettings = { ...initialSettings };
  let changeRevision = 0;
  let writeQueue: Promise<void> = Promise.resolve();

  async function load(): Promise<void> {
    applyLocalePreference(localePreference.value);
    await applyThemePreference(themePreference.value);
    try {
      if (!isDesktopApp()) return;
      const loadedSettings = normalizeSettings(await loadAppSettings(legacyLocalePreference ?? undefined));
      settings.value = loadedSettings;
      persistedSettings = { ...loadedSettings };
      applyLocalePreference(localePreference.value);
      await applyThemePreference(themePreference.value);
      clearLegacyLocalePreference();
    } finally {
      loaded.value = true;
    }
  }

  async function updateSettings(
    update: Partial<Omit<AppSettings, "schemaVersion">>,
  ): Promise<void> {
    const nextSettings: AppSettings = {
      ...settings.value,
      ...update,
      schemaVersion: SETTINGS_SCHEMA_VERSION,
    };
    settings.value = nextSettings;
    applyLocalePreference(localePreference.value);
    await applyThemePreference(themePreference.value);

    if (!isDesktopApp()) {
      persistedSettings = { ...nextSettings };
      return;
    }

    const revision = ++changeRevision;
    let savedSettings: AppSettings | undefined;
    let saveError: unknown;
    writeQueue = writeQueue.then(async () => {
      try {
        savedSettings = normalizeSettings(await saveAppSettings(nextSettings));
        persistedSettings = { ...savedSettings };
      } catch (error) {
        saveError = error;
      }
    });
    await writeQueue;

    if (saveError) {
      if (revision === changeRevision) {
        settings.value = { ...persistedSettings };
        applyLocalePreference(localePreference.value);
        await applyThemePreference(themePreference.value);
      }
      throw saveError;
    }

    if (savedSettings && revision === changeRevision) {
      settings.value = savedSettings;
      applyLocalePreference(localePreference.value);
      await applyThemePreference(themePreference.value);
    }
  }

  async function updateLocalePreference(preference: LocalePreference): Promise<void> {
    await updateSettings({ localePreference: preference });
  }

  async function updateThemePreference(preference: ThemePreference): Promise<void> {
    await updateSettings({ themePreference: preference });
  }

  return {
    settings,
    loaded,
    localePreference,
    themePreference,
    load,
    updateSettings,
    updateLocalePreference,
    updateThemePreference,
  };
});
