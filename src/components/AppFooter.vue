<template>
  <footer class="app-footer">
    <p>{{ version }}</p>
  </footer>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { getAppInfo } from "../api/app";

const version = ref("v1.0.0-rc.1");

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
</style>
