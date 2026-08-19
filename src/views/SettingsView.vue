<script setup lang="ts">
import { onMounted, ref } from "vue";
import { FolderOpen, RefreshCw } from "@lucide/vue";
import { openUrl } from "@tauri-apps/plugin-opener";
import githubIcon from "../assets/icons8-github.svg";
import appIcon from "../assets/key-switch.svg";
import AppButton from "../components/ui/AppButton.vue";
import { getAppInfo, openDataDirectory as openAppDataDirectory } from "../api/app";

const dataDirectory = ref("正在读取本地数据目录");
const version = ref("v0.0.2");
const notice = ref("");

function notify(message: string) {
  notice.value = message;
  window.setTimeout(() => { notice.value = ""; }, 2800);
}

async function openDataDirectory() {
  try { await openAppDataDirectory(); }
  catch { notify("无法打开本地存储位置"); }
}

async function openGithub() {
  try { await openUrl("https://github.com/ThirteenAsh/key-switch"); }
  catch { notify("无法打开 GitHub 仓库"); }
}

function checkForUpdates() { notify("测试版暂不支持自动更新"); }

onMounted(async () => {
  const appInfo = await getAppInfo();
  if (!appInfo) return;
  dataDirectory.value = appInfo.dataDirectory;
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
          <AppButton variant="primary" size="sm" @click="checkForUpdates">
            <RefreshCw :size="14" :stroke-width="2" />
            检查更新
          </AppButton>
        </div>
      </article>
    </div>
    <Transition name="toast"><p v-if="notice" class="toast" role="status">{{ notice }}</p></Transition>
  </section>
</template>
