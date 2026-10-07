<script setup lang="ts">
import { version as appVersion } from "../../package.json";
import { computed, onMounted, ref } from "vue";
import { storeToRefs } from "pinia";
import { useRoute } from "vue-router";
import { settingsNavigation } from "../data/settingsNavigation";
import { FileClock, FolderOpen, Languages, Palette, RefreshCw, Trash2 } from "@lucide/vue";
import { useI18n } from "vue-i18n";
import { openUrl } from "@tauri-apps/plugin-opener";
import githubIcon from "../assets/icons8-github.svg";
import appIcon from "../assets/key-switch.svg";
import AppButton from "../components/ui/AppButton.vue";
import AppSelect from "../components/ui/AppSelect.vue";
import type { AppSelectOption } from "../components/ui/AppSelect.vue";
import ConfirmDialog from "../components/ConfirmDialog.vue";
import UpdateAvailableDialog from "../components/UpdateAvailableDialog.vue";
import { clearLogs, getAppInfo, openDataDirectory as openAppDataDirectory, openLogDirectory as openAppLogDirectory } from "../api/app";
import type { UpdateInfo } from "../api/app";
import { useUpdateStore } from "../stores/update";
import { isThemePreference, useSettingsStore } from "../stores/settings";
import { isLocalePreference } from "../i18n/locale";
import { translateAppError } from "../i18n/errors";

const { t } = useI18n();
const route = useRoute();
const category = computed(() => settingsNavigation.find((item) => item.id === route.meta.settingsCategory) ?? settingsNavigation[0]);
const dataDirectory = ref("");
const version = ref(`v${appVersion}`);
const notice = ref("");
const clearLogDialogOpen = ref(false);
const availableUpdate = ref<UpdateInfo | null>(null);
const updateStore = useUpdateStore();
const settingsStore = useSettingsStore();
const { installing: installingUpdate, checking: checkingForUpdates, busy: updateBusy } = storeToRefs(updateStore);
const { localePreference, themePreference } = storeToRefs(settingsStore);
const languageOptions = computed<AppSelectOption[]>(() => [
  { value: "system", label: t("settings.language.system") },
  { value: "zh-CN", label: t("settings.language.simplifiedChinese") },
  { value: "zh-TW", label: t("settings.language.traditionalChinese") },
  { value: "en-US", label: t("settings.language.english") },
  { value: "ja-JP", label: t("settings.language.japanese") },
]);
const themeOptions = computed<AppSelectOption[]>(() => [
  { value: "system", label: t("settings.appearance.system") },
  { value: "light", label: t("settings.appearance.light") },
  { value: "dark", label: t("settings.appearance.dark") },
]);

async function changeLocalePreference(value: string): Promise<void> {
  if (!isLocalePreference(value)) return;
  try {
    await settingsStore.updateLocalePreference(value);
  } catch (error) {
    notify(translateAppError(error));
  }
}

async function changeThemePreference(value: string): Promise<void> {
  if (!isThemePreference(value)) return;
  try {
    await settingsStore.updateThemePreference(value);
  } catch (error) {
    notify(translateAppError(error));
  }
}

function notify(message: string) {
  notice.value = message;
  window.setTimeout(() => { notice.value = ""; }, 2800);
}

async function openDataDirectory() {
  try { await openAppDataDirectory(); }
  catch (error) { notify(translateAppError(error, "settings.storage.openFailed")); }
}

async function openLogDirectory() {
  try { await openAppLogDirectory(); }
  catch (error) { notify(translateAppError(error, "settings.logs.openFailed")); }
}

async function confirmClearLogs() {
  try {
    await clearLogs();
    clearLogDialogOpen.value = false;
    notify(t("settings.logs.cleared"));
  } catch (error) { notify(translateAppError(error, "settings.logs.clearFailed")); }
}

async function openGithub() {
  try { await openUrl("https://github.com/ThirteenAsh/key-switch"); }
  catch { notify(t("settings.version.githubFailed")); }
}

async function checkForUpdates() {
  if (updateBusy.value) return;
  try {
    const update = await updateStore.check();
    if (update === undefined) return;
    if (!update) {
      notify(t("settings.version.latest"));
      return;
    }
    availableUpdate.value = update;
  } catch (error) { notify(translateAppError(error, "settings.version.checkFailed")); }
}

async function openUpdateRelease() {
  if (!availableUpdate.value) return;
  try {
    const url = new URL(availableUpdate.value.releaseUrl);
    if (url.protocol !== "https:" || url.hostname !== "github.com" || !url.pathname.startsWith("/ThirteenAsh/key-switch/releases/")) {
      throw new Error("Invalid release URL");
    }
    await openUrl(url.href);
    availableUpdate.value = null;
  } catch { notify(t("settings.version.releaseFailed")); }
}

function closeUpdateDialog() {
  if (!installingUpdate.value) availableUpdate.value = null;
}

async function installAvailableUpdate() {
  if (!availableUpdate.value || installingUpdate.value) return;
  const update = availableUpdate.value;
  availableUpdate.value = null;
  void updateStore.install(update).catch(() => {});
}

onMounted(async () => {
  try {
    const appInfo = await getAppInfo();
    if (!appInfo) return;
    dataDirectory.value = appInfo.dataDirectory;
    version.value = `v${appInfo.version}`;
  } catch (error) {
    notify(translateAppError(error));
  }
});
</script>

<template>
  <section class="settings-view">
    <div class="view-toolbar">
      <div>
        <h1 class="settings-page-title"><component :is="category.icon" :size="24" aria-hidden="true" />{{ t(category.labelKey) }}</h1>
      </div>
    </div>
    <div class="settings-stack">
      <article v-if="category.id === 'general'" class="settings-card settings-card--select">
        <div class="settings-heading">
          <Languages :size="18" :stroke-width="1.8" />
          <div>
            <h2>{{ t("settings.language.title") }}</h2>
            <p>{{ t("settings.language.description") }}</p>
          </div>
        </div>
        <div class="settings-value">
          <AppSelect
            full-width
            :model-value="localePreference"
            :options="languageOptions"
            :label="t('settings.language.label')"
            @update:model-value="changeLocalePreference"
          />
        </div>
      </article>

      <article v-if="category.id === 'general'" class="settings-card settings-card--select">
        <div class="settings-heading">
          <Palette :size="18" :stroke-width="1.8" />
          <div>
            <h2>{{ t("settings.appearance.title") }}</h2>
            <p>{{ t("settings.appearance.description") }}</p>
          </div>
        </div>
        <div class="settings-value">
          <AppSelect
            full-width
            :model-value="themePreference"
            :options="themeOptions"
            :label="t('settings.appearance.label')"
            @update:model-value="changeThemePreference"
          />
        </div>
      </article>

      <article v-if="category.id === 'data'" class="settings-card">
        <div class="settings-heading">
          <FolderOpen :size="18" :stroke-width="1.8" />
          <div>
            <h2>{{ t("settings.storage.title") }}</h2>
            <p>{{ t("settings.storage.description") }}</p>
          </div>
        </div>
        <div class="settings-value">
          <code>{{ dataDirectory || t("settings.storage.loading") }}</code>
          <AppButton variant="secondary" size="sm" @click="openDataDirectory">{{ t("common.open") }}</AppButton>
        </div>
      </article>

      <article v-if="category.id === 'data'" class="settings-card">
        <div class="settings-heading">
          <FileClock :size="18" :stroke-width="1.8" />
          <div>
            <h2>{{ t("settings.logs.title") }}</h2>
            <p>{{ t("settings.logs.description") }}</p>
          </div>
        </div>
        <div class="settings-value settings-log-value">
          <div class="settings-log-actions">
            <AppButton variant="danger" size="sm" @click="clearLogDialogOpen = true">
              <Trash2 :size="14" :stroke-width="2" />
              {{ t("common.clear") }}
            </AppButton>
            <AppButton variant="secondary" size="sm" @click="openLogDirectory">
              <FolderOpen :size="14" :stroke-width="2" />
              {{ t("settings.logs.openDirectory") }}
            </AppButton>
          </div>
        </div>
      </article>

      <article v-if="category.id === 'about'" class="settings-card settings-card--version">
        <div class="settings-heading">
          <img :src="appIcon" class="settings-app-icon" :alt="t('common.appName')" />
          <div>
            <h2>{{ t("common.appName") }}</h2>
            <p>{{ t("settings.version.current", { version }) }}</p>
          </div>
        </div>
        <div class="settings-version-actions">
          <AppButton variant="secondary" size="sm" @click="openGithub">
            <img :src="githubIcon" class="button-github-icon" alt="" />
            {{ t("settings.version.github") }}
          </AppButton>
          <AppButton variant="primary" size="sm" :loading="checkingForUpdates" :disabled="updateBusy" @click="checkForUpdates">
            <RefreshCw :size="14" :stroke-width="2" />
            {{ t("settings.version.checkUpdates") }}
          </AppButton>
        </div>
      </article>
    </div>
    <Transition name="toast"><p v-if="notice" class="toast" role="status">{{ notice }}</p></Transition>
    <ConfirmDialog
      :open="clearLogDialogOpen"
      :title="t('settings.logs.clearDialogTitle')"
      :message="t('settings.logs.clearDialogMessage')"
      :confirm-label="t('settings.logs.clearLogs')"
      @close="clearLogDialogOpen = false"
      @confirm="confirmClearLogs"
    />
    <UpdateAvailableDialog
      :open="Boolean(availableUpdate)"
      :update="availableUpdate"
      :installing="installingUpdate"
      @close="closeUpdateDialog"
      @install="installAvailableUpdate"
      @release="openUpdateRelease"
    />
  </section>
</template>

<style scoped>
.settings-view .view-toolbar { min-height: var(--app-heading-height); padding: 0; margin-bottom: 16px; }
.settings-page-title { display: flex; align-items: center; gap: 9px; font-size: 14px; line-height: 1.4; font-weight: 600; }
.settings-page-title svg { width: 24px; height: 24px; padding: 3px; flex-shrink: 0; }
.settings-stack { max-width: none; padding-top: 0; }
.settings-card { min-height: 72px; padding: 16px; gap: 16px; flex-wrap: wrap; }
.settings-heading { flex: 1 1 200px; min-width: 0; gap: 9px; }
.settings-heading > div { min-width: 0; }
.settings-heading svg { flex: 0 0 20px; width: 20px; height: 20px; padding: 2px; }
.settings-heading h2 { font-size: 12.5px; }
.settings-heading p { font-size: 11.5px; line-height: 1.6; overflow-wrap: anywhere; }
.settings-app-icon { width: 24px; height: 24px; flex-basis: 24px; }
.settings-value { min-width: 0; max-width: 100%; flex-wrap: wrap; }
.settings-value code { max-width: 100%; white-space: normal; overflow-wrap: anywhere; }
.settings-card--select .settings-value { width: 190px; max-width: 100%; }
.settings-value :deep(.app-select__trigger) { height: 36px; font-size: 12.5px; }
.settings-value :deep(.app-select__option) { min-height: 34px; font-size: 12.5px; }
.settings-version-actions, .settings-log-actions { min-width: 0; max-width: 100%; flex-wrap: wrap; }
</style>
