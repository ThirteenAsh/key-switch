<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { storeToRefs } from "pinia";
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
import { checkForAppUpdates, clearLogs, getAppInfo, openDataDirectory as openAppDataDirectory, openLogDirectory as openAppLogDirectory } from "../api/app";
import type { UpdateInfo } from "../api/app";
import { useUpdateStore } from "../stores/update";
import { isThemePreference, useSettingsStore } from "../stores/settings";
import { isLocalePreference } from "../i18n/locale";
import { translateAppError } from "../i18n/errors";

const { t } = useI18n();
const dataDirectory = ref("");
const version = ref("v1.0.1");
const notice = ref("");
const clearLogDialogOpen = ref(false);
const checkingForUpdates = ref(false);
const availableUpdate = ref<UpdateInfo | null>(null);
const updateStore = useUpdateStore();
const settingsStore = useSettingsStore();
const { installing: installingUpdate } = storeToRefs(updateStore);
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
  if (checkingForUpdates.value) return;
  checkingForUpdates.value = true;
  try {
    const update = await checkForAppUpdates();
    if (!update) {
      notify(t("settings.version.latest"));
      return;
    }
    availableUpdate.value = update;
  } catch (error) { notify(translateAppError(error, "settings.version.checkFailed")); }
  finally { checkingForUpdates.value = false; }
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
  const releaseTag = availableUpdate.value.releaseTag;
  availableUpdate.value = null;
  void updateStore.install(releaseTag).catch(() => {});
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
        <h1>{{ t("settings.title") }}</h1>
        <p class="view-description">{{ t("settings.description") }}</p>
      </div>
    </div>
    <div class="settings-stack">
      <article class="settings-card settings-card--select">
        <div class="settings-heading">
          <Languages :size="18" :stroke-width="1.8" />
          <div>
            <h2>{{ t("settings.language.title") }}</h2>
            <p>{{ t("settings.language.description") }}</p>
          </div>
        </div>
        <div class="settings-value">
          <AppSelect
            :model-value="localePreference"
            :options="languageOptions"
            :label="t('settings.language.label')"
            @update:model-value="changeLocalePreference"
          />
        </div>
      </article>

      <article class="settings-card settings-card--select">
        <div class="settings-heading">
          <Palette :size="18" :stroke-width="1.8" />
          <div>
            <h2>{{ t("settings.appearance.title") }}</h2>
            <p>{{ t("settings.appearance.description") }}</p>
          </div>
        </div>
        <div class="settings-value">
          <AppSelect
            :model-value="themePreference"
            :options="themeOptions"
            :label="t('settings.appearance.label')"
            @update:model-value="changeThemePreference"
          />
        </div>
      </article>

      <article class="settings-card">
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

      <article class="settings-card">
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

      <article class="settings-card settings-card--version">
        <div class="settings-heading">
          <img :src="appIcon" class="settings-app-icon" alt="Key Switch" />
          <div>
            <h2>Key Switch</h2>
            <p>{{ t("settings.version.current", { version }) }}</p>
          </div>
        </div>
        <div class="settings-version-actions">
          <AppButton variant="secondary" size="sm" @click="openGithub">
            <img :src="githubIcon" class="button-github-icon" alt="" />
            GitHub
          </AppButton>
          <AppButton variant="primary" size="sm" :loading="checkingForUpdates" @click="checkForUpdates">
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
