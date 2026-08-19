<script setup lang="ts">
import { computed, ref } from "vue";
import { Plus, SlidersHorizontal, Trash2 } from "@lucide/vue";
import AppButton from "../components/ui/AppButton.vue";
import ProviderAvatar from "../components/ProviderAvatar.vue";
import CustomProviderDialog from "../components/CustomProviderDialog.vue";
import ProviderEditDialog from "../components/ProviderEditDialog.vue";
import ConfirmDialog from "../components/ConfirmDialog.vue";
import { useDashboardStore } from "../stores/dashboard";
import type { ProviderSummary } from "../types/domain";

const store = useDashboardStore();
const selectedProvider = ref<ProviderSummary | null>(null);
const editDialogOpen = ref(false);
const customDialogOpen = ref(false);
const notice = ref("");
const deleteTarget = ref<ProviderSummary | null>(null);
const deleteMessage = computed(() => {
  if (!deleteTarget.value) return "";
  const count = deleteTarget.value.keys.length;
  return count > 0
    ? `确定删除“${deleteTarget.value.name}”吗？该供应商的 ${count} 个 API Key 也会一并删除，此操作无法撤销。`
    : `确定删除“${deleteTarget.value.name}”吗？此操作无法撤销。`;
});

function openProviderConfiguration(provider: ProviderSummary) {
  selectedProvider.value = provider;
  editDialogOpen.value = true;
}

async function addCustomProvider(payload: { name: string; platformUrl: string; logo?: string }) {
  if (!await store.addCustomProvider(payload.name, payload.platformUrl, payload.logo)) {
    notice.value = "新增失败：供应商名称已存在";
    return;
  }

  customDialogOpen.value = false;
  notice.value = "已新增自定义供应商，首页已同步显示";
  window.setTimeout(() => {
    notice.value = "";
  }, 2800);
}

async function saveProviderConfiguration(payload: { id: string; name: string; platformUrl: string }) {
  if (!await store.updateProviderConfiguration(payload.id, payload.name, payload.platformUrl)) {
    notice.value = "保存失败：供应商名称已存在或配置无效";
    return;
  }

  editDialogOpen.value = false;
  notice.value = "供应商配置已保存，首页地址已同步更新";
  window.setTimeout(() => {
    notice.value = "";
  }, 2800);
}

async function deleteProvider() {
  if (!deleteTarget.value) return;
  const provider = deleteTarget.value;
  try {
    await store.removeProvider(provider.id);
    notice.value = `已删除供应商“${provider.name}”`;
  } catch {
    notice.value = "删除供应商失败";
  } finally {
    deleteTarget.value = null;
    window.setTimeout(() => { notice.value = ""; }, 2800);
  }
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
          <div class="provider-card-actions">
            <AppButton variant="secondary" size="sm" @click="openProviderConfiguration(provider)">
              <SlidersHorizontal :size="13" :stroke-width="2" />
              <span>配置</span>
            </AppButton>
            <AppButton variant="danger" size="icon-sm" title="删除供应商" :aria-label="`删除 ${provider.name}`" @click="deleteTarget = provider">
              <Trash2 :size="14" :stroke-width="2" />
            </AppButton>
          </div>
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
    <ConfirmDialog :open="Boolean(deleteTarget)" title="删除供应商" :message="deleteMessage" confirm-label="删除供应商" @close="deleteTarget = null" @confirm="deleteProvider" />
  </section>
</template>
