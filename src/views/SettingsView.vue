<script setup lang="ts">
import { onMounted, ref } from "vue";
import { FileClock, FolderOpen, RefreshCw, Trash2 } from "@lucide/vue";
import { openUrl } from "@tauri-apps/plugin-opener";
import githubIcon from "../assets/icons8-github.svg";
import appIcon from "../assets/key-switch.svg";
import AppButton from "../components/ui/AppButton.vue";
import ConfirmDialog from "../components/ConfirmDialog.vue";
import UpdateAvailableDialog from "../components/UpdateAvailableDialog.vue";
import { checkForAppUpdates, clearLogs, getAppInfo, installAppUpdate, openDataDirectory as openAppDataDirectory, openLogDirectory as openAppLogDirectory } from "../api/app";
import type { UpdateInfo } from "../api/app";

const dataDirectory = ref("正在读取本地数据目录");
const logDirectory = ref("正在读取日志目录");
const version = ref("v1.0.0-rc");
const notice = ref("");
const clearLogDialogOpen = ref(false);
const checkingForUpdates = ref(false);
const availableUpdate = ref<UpdateInfo | null>(null);
const installingUpdate = ref(false);

function notify(message: string) {
  notice.value = message;
  window.setTimeout(() => { notice.value = ""; }, 2800);
}

async function openDataDirectory() {
  try { await openAppDataDirectory(); }
  catch { notify("无法打开本地存储位置"); }
}

async function openLogDirectory() {
  try { await openAppLogDirectory(); }
  catch { notify("无法打开日志目录"); }
}

async function confirmClearLogs() {
  try {
    await clearLogs();
    clearLogDialogOpen.value = false;
    notify("日志已清空");
  } catch { notify("清空日志失败"); }
}

async function openGithub() {
  try { await openUrl("https://github.com/ThirteenAsh/key-switch"); }
  catch { notify("无法打开 GitHub 仓库"); }
}

async function checkForUpdates() {
  if (checkingForUpdates.value) return;
  checkingForUpdates.value = true;
  try {
    const update = await checkForAppUpdates();
    if (!update) {
      notify("当前已是最新版本");
      return;
    }
    availableUpdate.value = update;
  } catch { notify("检查更新失败，请稍后重试"); }
  finally { checkingForUpdates.value = false; }
}

async function openUpdateRelease() {
  if (!availableUpdate.value) return;
  try {
    const url = new URL(availableUpdate.value.releaseUrl);
    if (url.protocol !== "https:" || url.hostname !== "github.com" || !url.pathname.startsWith("/ThirteenAsh/key-switch/releases/")) {
      throw new Error("无效的 Release 地址");
    }
    await openUrl(url.href);
    availableUpdate.value = null;
  } catch { notify("无法打开版本下载页面"); }
}

function closeUpdateDialog() {
  if (!installingUpdate.value) availableUpdate.value = null;
}

async function installAvailableUpdate() {
  if (!availableUpdate.value || installingUpdate.value) return;
  installingUpdate.value = true;
  try {
    await installAppUpdate(availableUpdate.value.releaseTag);
  } catch {
    notify("自动更新失败，可前往 GitHub Release 手动下载");
    installingUpdate.value = false;
  }
}

onMounted(async () => {
  const appInfo = await getAppInfo();
  if (!appInfo) return;
  dataDirectory.value = appInfo.dataDirectory;
  logDirectory.value = appInfo.logDirectory;
  version.value = `v${appInfo.version}`;
});
</script>

<template>
  <section class="settings-view">
    <div class="view-toolbar">
      <div>
        <h1>设置</h1>
        <p class="view-description">本地数据与应用信息。</p>
      </div>
    </div>
    <div class="settings-stack">
      <article class="settings-card">
        <div class="settings-heading">
          <FolderOpen :size="18" :stroke-width="1.8" />
          <div>
            <h2>本地存储</h2>
            <p>供应商与 Key 元数据保存在此目录。</p>
          </div>
        </div>
        <div class="settings-value">
          <code>{{ dataDirectory }}</code>
          <AppButton variant="secondary" size="sm" @click="openDataDirectory">打开</AppButton>
        </div>
      </article>

      <article class="settings-card">
        <div class="settings-heading">
          <FileClock :size="18" :stroke-width="1.8" />
          <div>
            <h2>运行日志</h2>
            <p>本地操作与检测结果</p>
          </div>
        </div>
        <div class="settings-value settings-log-value">
          <div class="settings-log-actions">
            <AppButton variant="danger" size="sm" @click="clearLogDialogOpen = true">
              <Trash2 :size="14" :stroke-width="2" />
              清空
            </AppButton>
            <AppButton variant="secondary" size="sm" @click="openLogDirectory">
              <FolderOpen :size="14" :stroke-width="2" />
              打开目录
            </AppButton>
          </div>
        </div>
      </article>

      <article class="settings-card settings-card--version">
        <div class="settings-heading">
          <img :src="appIcon" class="settings-app-icon" alt="Key Switch" />
          <div>
            <h2>Key Switch</h2>
            <p>当前版本 {{ version }}</p>
          </div>
        </div>
        <div class="settings-version-actions">
          <AppButton variant="secondary" size="sm" @click="openGithub">
            <img :src="githubIcon" class="button-github-icon" alt="" />
            GitHub
          </AppButton>
          <AppButton variant="primary" size="sm" :loading="checkingForUpdates" @click="checkForUpdates">
            <RefreshCw :size="14" :stroke-width="2" />
            检查更新
          </AppButton>
        </div>
      </article>
    </div>
    <Transition name="toast"><p v-if="notice" class="toast" role="status">{{ notice }}</p></Transition>
    <ConfirmDialog
      :open="clearLogDialogOpen"
      title="清空运行日志"
      message="确定清空全部本地运行日志吗？此操作无法撤销。"
      confirm-label="清空日志"
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
