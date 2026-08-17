<script setup lang="ts">
import { onMounted, ref } from "vue";
import { ClipboardCheck, Database, FolderOpen, Shield } from "@lucide/vue";
import { getAppInfo } from "../api/app";

const dataDirectory = ref("等待本地数据层接入");
const version = ref("v0.1.0");

onMounted(async () => {
  const appInfo = await getAppInfo();
  if (!appInfo) return;

  dataDirectory.value = appInfo.dataDirectory;
  version.value = `v${appInfo.version}`;
});
</script>

<template>
  <section class="settings-view">
    <div class="view-toolbar"><div><h1>设置</h1><p class="view-description">本地数据、安全行为和应用信息。</p></div></div>
    <div class="settings-stack">
      <article class="settings-card"><div class="settings-heading"><Database :size="22" /><div><h2>本地存储</h2><p>数据库将在首次添加数据时初始化。</p></div></div><div class="settings-value"><code>{{ dataDirectory }}</code><button class="button button--secondary button--compact" type="button"><FolderOpen :size="17" /> 打开位置</button></div></article>
      <article class="settings-card"><div class="settings-heading"><Shield :size="22" /><div><h2>密钥保护</h2><p>完整 API Key 不会显示在列表、日志或前端持久化存储中。</p></div></div><span class="settings-pending">待接入系统密钥库</span></article>
      <article class="settings-card"><div class="settings-heading"><ClipboardCheck :size="22" /><div><h2>复制行为</h2><p>后续可配置复制后自动清除剪贴板的时间。</p></div></div><span class="settings-pending">待实现</span></article>
    </div>
    <p class="settings-version">当前应用版本：{{ version }}</p>
  </section>
</template>
