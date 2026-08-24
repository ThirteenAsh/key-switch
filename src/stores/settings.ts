import { computed, ref } from "vue";
import { defineStore } from "pinia";
import {
  isDesktopApp,
  loadAppSettings,
  saveAppSettings,
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

function normalizeSettings(settings: AppSettings): AppSettings {
  return {
    schemaVersion: SETTINGS_SCHEMA_VERSION,
    localePreference: isLocalePreference(settings.localePreference)
      ? settings.localePreference
      : "system",
  };
}

export const useSettingsStore = defineStore("settings", () => {
  const legacyLocalePreference = readLegacyLocalePreference();
  const initialSettings: AppSettings = {
    schemaVersion: SETTINGS_SCHEMA_VERSION,
    localePreference: legacyLocalePreference ?? "system",
  };
  const settings = ref<AppSettings>(initialSettings);
  const loaded = ref(false);
  const localePreference = computed<LocalePreference>(() => (
    isLocalePreference(settings.value.localePreference)
      ? settings.value.localePreference
      : "system"
  ));

  let persistedSettings = { ...initialSettings };
  let changeRevision = 0;
  let writeQueue: Promise<void> = Promise.resolve();

  async function load(): Promise<void> {
    applyLocalePreference(localePreference.value);
    try {
      if (!isDesktopApp()) return;
      const loadedSettings = normalizeSettings(await loadAppSettings(legacyLocalePreference ?? undefined));
      settings.value = loadedSettings;
      persistedSettings = { ...loadedSettings };
      applyLocalePreference(localePreference.value);
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
      }
      throw saveError;
    }

    if (savedSettings && revision === changeRevision) {
      settings.value = savedSettings;
      applyLocalePreference(localePreference.value);
    }
  }

  async function updateLocalePreference(preference: LocalePreference): Promise<void> {
    await updateSettings({ localePreference: preference });
  }

  return {
    settings,
    loaded,
    localePreference,
    load,
    updateSettings,
    updateLocalePreference,
  };
});
