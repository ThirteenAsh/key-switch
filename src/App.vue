<template>
  <div class="app-shell">
    <div class="app-body">
      <AppSidebar />
      <main class="page-content">
        <RouterView v-slot="{ Component, route }">
          <Transition name="page-fade" mode="out-in">
            <component :is="Component" :key="route.path" />
          </Transition>
        </RouterView>
      </main>
    </div>
    <AppFooter />
    <UpdateAvailableToast
      v-if="!updateDialogOpen"
      :update="availableUpdate"
      @close="dismissUpdate"
      @view="viewUpdate"
    />
    <UpdateAvailableDialog
      :open="updateDialogOpen"
      :update="availableUpdate"
      :installing="installingUpdate"
      @close="closeUpdateDialog"
      @install="installAvailableUpdate"
      @release="openUpdateRelease"
    />
    <Transition name="toast"><p v-if="notice" class="toast" role="status">{{ notice }}</p></Transition>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { openUrl } from "@tauri-apps/plugin-opener";
import type { UpdateInfo } from "./api/app";
import { checkForAppUpdates, installAppUpdate } from "./api/app";
import AppFooter from "./components/AppFooter.vue";
import AppSidebar from "./components/AppSidebar.vue";
import UpdateAvailableDialog from "./components/UpdateAvailableDialog.vue";
import UpdateAvailableToast from "./components/UpdateAvailableToast.vue";
import { useDashboardStore } from "./stores/dashboard";

const store = useDashboardStore();
const availableUpdate = ref<UpdateInfo | null>(null);
const updateDialogOpen = ref(false);
const installingUpdate = ref(false);
const notice = ref("");
let noticeTimer: number | undefined;
let updateToastTimer: number | undefined;

function notify(message: string) {
  notice.value = message;
  window.clearTimeout(noticeTimer);
  noticeTimer = window.setTimeout(() => { notice.value = ""; }, 3200);
}

async function checkForUpdatesAtStartup() {
  try {
    availableUpdate.value = await checkForAppUpdates();
    if (availableUpdate.value) {
      window.clearTimeout(updateToastTimer);
      updateToastTimer = window.setTimeout(dismissUpdate, 5000);
    }
  } catch {
    // 启动检查不应打断用户；失败详情已由 Rust 侧返回给手动检查入口。
  }
}

function dismissUpdate() {
  window.clearTimeout(updateToastTimer);
  availableUpdate.value = null;
  updateDialogOpen.value = false;
}

function viewUpdate() {
  window.clearTimeout(updateToastTimer);
  updateDialogOpen.value = true;
}

function closeUpdateDialog() {
  if (!installingUpdate.value) updateDialogOpen.value = false;
}

async function openUpdateRelease() {
  if (!availableUpdate.value) return;
  try {
    const url = new URL(availableUpdate.value.releaseUrl);
    if (url.protocol !== "https:" || url.hostname !== "github.com" || !url.pathname.startsWith("/ThirteenAsh/key-switch/releases/")) {
      throw new Error("无效的 Release 地址");
    }
    await openUrl(url.href);
    dismissUpdate();
  } catch {
    notify("无法打开版本下载页面");
  }
}

async function installAvailableUpdate() {
  if (!availableUpdate.value || installingUpdate.value) return;
  installingUpdate.value = true;
  try {
    await installAppUpdate(availableUpdate.value.releaseTag);
  } catch {
    installingUpdate.value = false;
    notify("自动更新失败，可前往 GitHub Release 手动下载");
  }
}

onMounted(() => {
  void store.load();
  if ("__TAURI_INTERNALS__" in window) void checkForUpdatesAtStartup();
});
</script>

<style scoped>
/* 页面级轻快丝滑过渡动画（0.15s） */
.page-fade-enter-active,
.page-fade-leave-active {
  transition: opacity 0.15s ease-out,
              transform 0.15s cubic-bezier(0.16, 1, 0.3, 1);
}

.page-fade-enter-from {
  opacity: 0;
  transform: translateY(4px);
}

.page-fade-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}
</style>
