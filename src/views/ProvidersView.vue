<script setup lang="ts">
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import { Plus, SlidersHorizontal, Trash2 } from "@lucide/vue";
import AppButton from "../components/ui/AppButton.vue";
import ProviderAvatar from "../components/ProviderAvatar.vue";
import CustomProviderDialog from "../components/CustomProviderDialog.vue";
import ProviderEditDialog from "../components/ProviderEditDialog.vue";
import ConfirmDialog from "../components/ConfirmDialog.vue";
import { useDashboardStore } from "../stores/dashboard";
import type { ProviderSummary, ProviderValidation } from "../types/domain";
import { translateAppError } from "../i18n/errors";

const store = useDashboardStore();
const { t } = useI18n();
const selectedProvider = ref<ProviderSummary | null>(null);
const editDialogOpen = ref(false);
const customDialogOpen = ref(false);
const notice = ref("");
const deleteTarget = ref<ProviderSummary | null>(null);
const deleteMessage = computed(() => {
  if (!deleteTarget.value) return "";
  const count = deleteTarget.value.keys.length;
  return count > 0
    ? t("providers.deleteDialog.withKeys", { name: deleteTarget.value.name, count }, count)
    : t("providers.deleteDialog.withoutKeys", { name: deleteTarget.value.name });
});

function openProviderConfiguration(provider: ProviderSummary) {
  selectedProvider.value = provider;
  editDialogOpen.value = true;
}

async function addCustomProvider(payload: { name: string; platformUrl: string; logo?: string; validation: ProviderValidation }) {
  try {
    if (!await store.addCustomProvider(payload)) {
      notice.value = t("providers.notices.addFailed");
      return;
    }

    customDialogOpen.value = false;
    notice.value = t("providers.notices.added");
    window.setTimeout(() => {
      notice.value = "";
    }, 2800);
  } catch (error) {
    notice.value = translateAppError(error, "providers.notices.addFailed");
  }
}

async function saveProviderConfiguration(payload: { id: string; name: string; platformUrl: string; validation: ProviderValidation }) {
  try {
    if (!await store.updateProviderConfiguration(payload.id, payload.name, payload.platformUrl, payload.validation)) {
      notice.value = t("providers.notices.saveFailed");
      return;
    }

    editDialogOpen.value = false;
    notice.value = t("providers.notices.saved");
    window.setTimeout(() => {
      notice.value = "";
    }, 2800);
  } catch (error) {
    notice.value = translateAppError(error, "providers.notices.saveFailed");
  }
}

async function deleteProvider() {
  if (!deleteTarget.value) return;
  const provider = deleteTarget.value;
  try {
    await store.removeProvider(provider.id);
    notice.value = t("providers.notices.deleted", { name: provider.name });
  } catch (error) {
    notice.value = translateAppError(error, "providers.notices.deleteFailed");
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
        <h1>{{ t("providers.title") }}</h1>
        <p class="view-description">{{ t("providers.description") }}</p>
      </div>
      <AppButton variant="primary" @click="customDialogOpen = true">
        <Plus :size="15" :stroke-width="2.2" />
        <span>{{ t("providers.addProvider") }}</span>
      </AppButton>
    </div>
    <div class="provider-management-grid">
      <article v-for="provider in store.providers" :key="provider.id" class="provider-management-card">
        <div class="provider-name">
          <ProviderAvatar :provider="provider" />
          <div>
            <strong>{{ provider.name }}</strong>
            <p>{{ t(provider.kind === 'builtin' ? 'providers.builtin' : 'providers.custom') }}</p>
          </div>
        </div>
        <div class="provider-card-footer">
          <span>{{ t("providers.keyCount", { count: provider.keys.length }, provider.keys.length) }}</span>
          <div class="provider-card-actions">
            <AppButton variant="secondary" size="sm" @click="openProviderConfiguration(provider)">
              <SlidersHorizontal :size="13" :stroke-width="2" />
              <span>{{ t("common.configure") }}</span>
            </AppButton>
            <AppButton variant="danger" size="icon-sm" :title="t('providers.deleteAction')" :aria-label="t('providers.deleteAria', { name: provider.name })" @click="deleteTarget = provider">
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
    <ConfirmDialog :open="Boolean(deleteTarget)" :title="t('providers.deleteDialog.title')" :message="deleteMessage" :confirm-label="t('providers.deleteAction')" @close="deleteTarget = null" @confirm="deleteProvider" />
  </section>
</template>
