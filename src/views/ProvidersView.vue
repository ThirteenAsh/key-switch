<script setup lang="ts">
import { ref } from "vue";
import { Plus, SlidersHorizontal } from "@lucide/vue";
import AppButton from "../components/ui/AppButton.vue";
import ProviderAvatar from "../components/ProviderAvatar.vue";
import CustomProviderDialog from "../components/CustomProviderDialog.vue";
import ProviderEditDialog from "../components/ProviderEditDialog.vue";
import { useDashboardStore } from "../stores/dashboard";
import type { ProviderSummary } from "../types/domain";

const store = useDashboardStore();
const selectedProvider = ref<ProviderSummary | null>(null);
const editDialogOpen = ref(false);
const customDialogOpen = ref(false);
const notice = ref("");

function openProviderConfiguration(provider: ProviderSummary) {
  selectedProvider.value = provider;
  editDialogOpen.value = true;
}

function addCustomProvider(payload: { name: string; baseUrl: string; logo?: string }) {
  if (!store.addCustomProvider(payload.name, payload.baseUrl, payload.logo)) {
    notice.value = "新增失败：供应商名称已存在";
    return;
  }

  customDialogOpen.value = false;
  notice.value = "已新增自定义供应商，首页已同步显示";
  window.setTimeout(() => {
    notice.value = "";
  }, 2800);
}

function saveProviderConfiguration(payload: { id: string; name: string; baseUrl: string }) {
  if (!store.updateProviderConfiguration(payload.id, payload.name, payload.baseUrl)) {
    notice.value = "保存失败：供应商名称已存在或配置无效";
    return;
  }

  editDialogOpen.value = false;
  notice.value = "供应商配置已保存，首页地址已同步更新";
  window.setTimeout(() => {
    notice.value = "";
  }, 2800);
}
</script>

<template>
  <section class="providers-view">
    <div class="view-toolbar">
      <div>
        <h1>供应商</h1>
        <p class="view-description">管理内置服务商和自定义 API 端点。</p>
      </div>
      <AppButton variant="primary" @click="customDialogOpen = true">
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
          <AppButton variant="secondary" size="sm" @click="openProviderConfiguration(provider)">
            <SlidersHorizontal :size="13" :stroke-width="2" />
            <span>配置</span>
          </AppButton>
        </div>
      </article>
    </div>
    <p v-if="notice" class="toast" role="status">{{ notice }}</p>
    <ProviderEditDialog
      :open="editDialogOpen"
      :provider="selectedProvider"
      @close="editDialogOpen = false"
      @save="saveProviderConfiguration"
    />
    <CustomProviderDialog :open="customDialogOpen" @close="customDialogOpen = false" @add="addCustomProvider" />
  </section>
</template>
