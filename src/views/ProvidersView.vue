<script setup lang="ts">
import { Plus, Settings2 } from "@lucide/vue";
import ProviderAvatar from "../components/ProviderAvatar.vue";
import { useDashboardStore } from "../stores/dashboard";

const store = useDashboardStore();
</script>

<template>
  <section class="providers-view">
    <div class="view-toolbar">
      <div><h1>供应商</h1><p class="view-description">管理内置服务商和自定义 API 端点。</p></div>
      <button class="button button--primary" type="button"><Plus :size="21" /> 添加供应商</button>
    </div>
    <div class="provider-management-grid">
      <article v-for="provider in store.providers" :key="provider.id" class="provider-management-card">
        <div class="provider-name"><ProviderAvatar :provider="provider" /><div><strong>{{ provider.name }}</strong><p>{{ provider.kind === 'builtin' ? '内置供应商' : '自定义供应商' }}</p></div></div>
        <div class="provider-card-footer"><span>{{ provider.keys.length }} 个 Key</span><button class="button button--secondary button--compact" type="button"><Settings2 :size="17" /> 配置</button></div>
      </article>
    </div>
  </section>
</template>
