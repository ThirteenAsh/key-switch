<template>
  <footer class="app-footer">
    <p>{{ version }}</p>
    <Transition name="footer-status" mode="out-in">
      <p v-if="updateStore.installStatus === 'downloading'" key="downloading" class="update-status update-status--downloading" role="status">
        <LoaderCircle :size="13" aria-hidden="true" />
        {{ t("update.downloading") }}
      </p>
      <p v-else-if="updateStore.installStatus === 'timeout'" key="timeout" class="update-status update-status--warning" role="alert">
        <TriangleAlert :size="13" aria-hidden="true" />
        {{ t("update.timeout") }}
      </p>
      <p v-else-if="updateStore.installStatus === 'failed'" key="failed" class="update-status update-status--warning" role="alert">
        <TriangleAlert :size="13" aria-hidden="true" />
        {{ t("update.failed") }}
      </p>
    </Transition>
  </footer>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { LoaderCircle, TriangleAlert } from "@lucide/vue";
import { useI18n } from "vue-i18n";
import { getAppInfo } from "../api/app";
import { useUpdateStore } from "../stores/update";

const version = ref("v1.0.0");
const updateStore = useUpdateStore();
const { t } = useI18n();

onMounted(async () => {
  try {
    const appInfo = await getAppInfo();
    if (appInfo) version.value = `v${appInfo.version}`;
  } catch {
    // 无法读取桌面应用信息时保留构建版本作为回退。
  }
});
</script>

<style scoped>
.app-footer p {
  color: #94a3b8;
}

.update-status {
  justify-self: end;
  color: #64748b !important;
  font-weight: 500;
}

.update-status--warning {
  color: #d97706 !important;
}

.update-status--downloading svg {
  animation: footer-status-spin 0.8s linear infinite;
}

.footer-status-enter-active,
.footer-status-leave-active {
  transition: opacity 0.18s ease, transform 0.18s ease;
}

.footer-status-enter-from,
.footer-status-leave-to {
  opacity: 0;
  transform: translateY(3px);
}

@keyframes footer-status-spin {
  to { transform: rotate(360deg); }
}
</style>
