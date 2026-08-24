export const supportedLocales = ["zh-CN", "zh-TW", "en-US", "ja-JP"] as const;
export const localePreferences = ["system", ...supportedLocales] as const;

export type AppLocale = (typeof supportedLocales)[number];
export type LocalePreference = (typeof localePreferences)[number];

export const LOCALE_STORAGE_KEY = "key-switch.locale";

export function isLocalePreference(value: unknown): value is LocalePreference {
  return typeof value === "string" && (localePreferences as readonly string[]).includes(value);
}

export function readLegacyLocalePreference(): LocalePreference | null {
  try {
    const stored = window.localStorage.getItem(LOCALE_STORAGE_KEY);
    return isLocalePreference(stored) ? stored : null;
  } catch {
    return null;
  }
}

export function clearLegacyLocalePreference(): void {
  try {
    window.localStorage.removeItem(LOCALE_STORAGE_KEY);
  } catch {
    // 旧偏好清理失败不会影响 settings.json 中的新设置。
  }
}

export function normalizeSystemLocale(value: string): AppLocale | null {
  const locale = value.trim().replace(/_/g, "-").toLowerCase();
  if (!locale) return null;
  if (locale.startsWith("zh-hant") || /^(zh-)?(tw|hk|mo)(-|$)/.test(locale) || /^zh-(tw|hk|mo)(-|$)/.test(locale)) return "zh-TW";
  if (locale.startsWith("zh")) return "zh-CN";
  if (locale.startsWith("ja")) return "ja-JP";
  if (locale.startsWith("en")) return "en-US";
  return null;
}

export function resolveSystemLocale(): AppLocale {
  let systemLocales: readonly string[];
  try {
    systemLocales = navigator.languages?.length
      ? navigator.languages
      : navigator.language
        ? [navigator.language]
        : [];
  } catch {
    return "zh-CN";
  }

  if (systemLocales.length === 0) return "zh-CN";
  for (const locale of systemLocales) {
    const normalized = normalizeSystemLocale(locale);
    if (normalized) return normalized;
  }

  // 能读到系统语言但当前尚不支持时，为非中文用户提供通用英文界面。
  return "en-US";
}

export function isChineseLocale(locale: AppLocale): boolean {
  return locale === "zh-CN" || locale === "zh-TW";
}
