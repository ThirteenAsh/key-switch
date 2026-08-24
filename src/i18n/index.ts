import { computed, ref } from "vue";
import { createI18n } from "vue-i18n";
import enUS from "./messages/en-US";
import jaJP from "./messages/ja-JP";
import zhCN from "./messages/zh-CN";
import zhTW from "./messages/zh-TW";
import {
  readLegacyLocalePreference,
  resolveSystemLocale,
  type AppLocale,
  type LocalePreference,
} from "./locale";

const systemLocale = ref<AppLocale>(resolveSystemLocale());
export const localePreference = ref<LocalePreference>(readLegacyLocalePreference() ?? "system");
export const effectiveLocale = computed<AppLocale>(() => (
  localePreference.value === "system" ? systemLocale.value : localePreference.value
));

export const i18n = createI18n({
  legacy: false,
  locale: effectiveLocale.value,
  fallbackLocale: "zh-CN",
  messages: {
    "zh-CN": zhCN,
    "zh-TW": zhTW,
    "en-US": enUS,
    "ja-JP": jaJP,
  },
});

function applyDocumentLocale(locale: AppLocale): void {
  document.documentElement.lang = locale;
  document.documentElement.dir = "ltr";
}

export function setLocalePreference(preference: LocalePreference): void {
  if (preference === "system") systemLocale.value = resolveSystemLocale();
  localePreference.value = preference;
  i18n.global.locale.value = effectiveLocale.value;
  applyDocumentLocale(effectiveLocale.value);
}

export function initializeLocale(): void {
  i18n.global.locale.value = effectiveLocale.value;
  applyDocumentLocale(effectiveLocale.value);
}
