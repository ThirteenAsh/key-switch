<script setup lang="ts">
import { Plus, SlidersHorizontal } from "@lucide/vue";
import AppButton from "../components/ui/AppButton.vue";
import ProviderAvatar from "../components/ProviderAvatar.vue";
import { useDashboardStore } from "../stores/dashboard";

const store = useDashboardStore();
</script>

<template>
  <section class="providers-view">
    <div class="view-toolbar">
      <div>
        <h1>供应商</h1>
        <p class="view-description">管理内置服务商和自定义 API 端点。</p>
      </div>
      <AppButton variant="primary">
        <Plus :size="15" :stroke-width="2.2" />
        <span>添加供应商</span>
      </AppButton>
    </div>
    <div class="provider-management-grid">
      <article v-for="provider in store.providers" :key="provider.id" class="provider-management-card">
        <div class="provider-name">
          <ProviderAvatar :provider="provider" />
          <div>
            <strong>{{ provider.name }}</strong>
            <p>{{ provider.kind === 'builtin' ? '内置供应商' : '自定义供应商' }}</p>
          </div>
        </div>
        <div class="provider-card-footer">
          <span>{{ provider.keys.length }} 个 Key</span>
          <AppButton variant="secondary" size="sm">
            <SlidersHorizontal :size="13" :stroke-width="2" />
            <span>配置</span>
          </AppButton>
        </div>
      </article>
    </div>
  </section>
</template>
