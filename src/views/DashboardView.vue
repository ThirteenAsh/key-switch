<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import {
  Copy,
  Trash2,
  SquarePen,
  Check,
  Info,
  KeyRound,
  Plus,
  Search,
  ShieldCheck,
  RefreshCw
} from "@lucide/vue";
import { openUrl } from "@tauri-apps/plugin-opener";
import AppButton from "../components/ui/AppButton.vue";
import ProviderAvatar from "../components/ProviderAvatar.vue";
import ProviderConfigDialog from "../components/ProviderConfigDialog.vue";
import ApiKeyDialog from "../components/ApiKeyDialog.vue";
import ConfirmDialog from "../components/ConfirmDialog.vue";
import StatusBadge from "../components/StatusBadge.vue";
import { useDashboardStore } from "../stores/dashboard";
import { copyApiKey } from "../api/app";
import { effectiveLocale } from "../i18n";
import { translateAppError } from "../i18n/errors";
import type { ApiKeySummary, ProviderSummary, ProviderValidation } from "../types/domain";

const store = useDashboardStore();
const { t } = useI18n();
const notice = ref("");
const configDialogOpen = ref(false);
const copiedKeyId = ref<string | null>(null);
const checkingProviderId = ref<string | null>(null);
const keyDialogProvider = ref<ProviderSummary | null>(null);
const editingKey = ref<ApiKeySummary | null>(null);
const deleteTarget = ref<{ providerId: string; keyId: string } | null>(null);

const provider = computed(() => store.selectedProvider);
const keyQuery = ref("");
const visibleKeys = computed(() => {
  const keyword = keyQuery.value.trim().toLocaleLowerCase(effectiveLocale.value);
  return provider.value?.keys.filter((key) => !keyword || key.remark.toLocaleLowerCase(effectiveLocale.value).includes(keyword) || key.maskedValue.toLocaleLowerCase(effectiveLocale.value).includes(keyword)) ?? [];
});
watch(() => provider.value?.id, () => { keyQuery.value = ""; });
async function retryLoad() {
  try { await store.load(); } catch { /* 错误状态在页面内展示。 */ }
}

function notify(message: string) {
  notice.value = message;
  window.setTimeout(() => {
    notice.value = "";
  }, 2800);
}

async function handleCopy(keyId: string) {
  try {
    await copyApiKey(keyId);
    copiedKeyId.value = keyId;
    notify(t("dashboard.notices.copied"));
    setTimeout(() => { if (copiedKeyId.value === keyId) copiedKeyId.value = null; }, 2000);
  } catch (error) { notify(translateAppError(error, "dashboard.notices.copyFailed")); }
}

async function handleRefreshProvider(providerId: string, event: MouseEvent) {
  event.stopPropagation();
  const provider = store.providers.find((item) => item.id === providerId);
  if (provider && !provider.validationSupported) {
    notify(t("dashboard.notices.validationUnsupported"));
    return;
  }
  if (checkingProviderId.value || provider?.keys.some((key) => key.status === "checking")) return;
  checkingProviderId.value = providerId;
  notify(t("dashboard.notices.checkingProvider"));
  try {
    const keys = await store.checkKeys(providerId);
    const validCount = keys.filter((key) => key.status === "valid").length;
    const invalidCount = keys.filter((key) => key.status === "invalid").length;
    const errorCount = keys.filter((key) => key.status === "error").length;
    const unsupportedCount = keys.filter((key) => key.status === "unsupported").length;
    notify(t("dashboard.notices.checkComplete", { valid: validCount, invalid: invalidCount, error: errorCount, unsupported: unsupportedCount }));
  }
  catch (error) { notify(translateAppError(error, "dashboard.notices.checkFailed")); }
  finally { checkingProviderId.value = null; }
}

function openCreateKeyDialog(provider: ProviderSummary) {
  editingKey.value = null;
  keyDialogProvider.value = provider;
}

async function handleCheckKey(providerId: string, keyId: string) {
  const provider = store.providers.find((item) => item.id === providerId);
  if (provider && !provider.validationSupported) {
    notify(t("dashboard.notices.validationUnsupported"));
    return;
  }
  try {
    const key = await store.checkKey(providerId, keyId);
    const message = key.status === "valid"
      ? t("dashboard.notices.keyValid")
      : key.status === "invalid"
        ? t("dashboard.notices.keyInvalid")
        : key.status === "unsupported"
          ? t("dashboard.notices.validationUnsupported")
          : key.checkErrorCode
            ? t(`statusReasons.${key.checkErrorCode}`)
            : t("dashboard.notices.keyCheckError");
    notify(message);
  } catch (error) { notify(translateAppError(error, "dashboard.notices.checkFailed")); }
}

function openEditKeyDialog(provider: ProviderSummary, key: ApiKeySummary) {
  editingKey.value = key;
  keyDialogProvider.value = provider;
}

function closeKeyDialog() {
  keyDialogProvider.value = null;
  editingKey.value = null;
}

async function saveKey(payload: { remark: string; value: string }) {
  if (!keyDialogProvider.value) return;
  try {
    if (editingKey.value) {
      const keyWasReplaced = Boolean(payload.value);
      await store.replaceKey(keyDialogProvider.value.id, { id: editingKey.value.id, ...payload });
      closeKeyDialog();
      notify(t(keyWasReplaced ? "dashboard.notices.keyReplaced" : "dashboard.notices.remarkSaved"));
    } else {
      await store.addKey({ providerId: keyDialogProvider.value.id, ...payload });
      closeKeyDialog();
      notify(t("dashboard.notices.keySaved"));
    }
  } catch (error) {
    notify(translateAppError(error, editingKey.value ? "dashboard.notices.editSaveFailed" : "dashboard.notices.keySaveFailed"));
  }
}

function requestDeleteKey(providerId: string, keyId: string) { deleteTarget.value = { providerId, keyId }; }
async function deleteKey() {
  if (!deleteTarget.value) return;
  const target = deleteTarget.value;
  try { await store.deleteKey(target.providerId, target.keyId); notify(t("dashboard.notices.keyDeleted")); }
  catch (error) { notify(translateAppError(error, "dashboard.notices.keyDeleteFailed")); }
  finally { deleteTarget.value = null; }
}

function getProviderEndpoint(provider: ProviderSummary): string {
  return provider.platformUrl || t("dashboard.noPlatformUrl");
}

async function openProviderPlatform(url: string) {
  try {
    const parsedUrl = new URL(url);
    if (parsedUrl.protocol !== "https:" && parsedUrl.protocol !== "http:") {
      throw new Error("Unsupported URL protocol");
    }

    if ("__TAURI_INTERNALS__" in window) {
      await openUrl(parsedUrl);
      return;
    }

    window.open(parsedUrl.href, "_blank", "noopener,noreferrer");
  } catch {
    notify(t("dashboard.notices.openPlatformFailed"));
  }
}

async function addBuiltinProvider(providerId: string) {
  try {
    if (!await store.addBuiltinProvider(providerId, effectiveLocale.value)) {
      notify(t("dashboard.notices.providerConfigured"));
      return;
    }
    configDialogOpen.value = false;
    notify(t("dashboard.notices.providerAdded"));
  } catch (error) {
    notify(translateAppError(error));
  }
}

async function addCustomProvider(name: string, platformUrl: string, logo: string | undefined, validation: ProviderValidation) {
  try {
    if (!await store.addCustomProvider({ name, platformUrl, logo, validation })) {
      notify(t("dashboard.notices.providerExists"));
      return;
    }
    configDialogOpen.value = false;
    notify(t("dashboard.notices.customProviderAdded"));
  } catch (error) {
    notify(translateAppError(error));
  }
}
</script>

<template>
  <section class="dashboard-view">
    <div class="view-toolbar">
      <div class="dashboard-heading">
        <Transition name="provider-heading" mode="out-in">
          <div :key="provider?.id ?? 'empty'" class="dashboard-title">
            <ProviderAvatar v-if="provider" :provider="provider" />
            <h1 :title="provider?.name">{{ provider?.name ?? t("dashboard.title") }}</h1>
            <template v-if="provider">
              <span class="provider-tag" :class="provider.kind === 'builtin' ? 'provider-tag--builtin' : 'provider-tag--custom'">{{ t(provider.kind === 'builtin' ? 'dashboard.officialProvider' : 'dashboard.customProvider') }}</span>
              <a v-if="provider.platformUrl" :href="provider.platformUrl" :title="provider.platformUrl" class="provider-endpoint-link" @click.prevent="openProviderPlatform(provider.platformUrl)">{{ getProviderEndpoint(provider) }}</a>
              <span v-else class="provider-endpoint-link">{{ getProviderEndpoint(provider) }}</span>
            </template>
          </div>
        </Transition>
      </div>
      <div class="toolbar-actions">
        <AppButton variant="primary" @click="configDialogOpen = true">
          <Plus :size="15" :stroke-width="2.2" />
          <span>{{ t("providers.addProvider") }}</span>
        </AppButton>
      </div>
    </div>

    <div v-if="store.loading" class="empty-state" role="status">
      <RefreshCw :size="28" class="is-spinning" aria-hidden="true" />
      <p>{{ t('dashboard.loading') }}</p>
    </div>
    <div v-else-if="store.loadError" class="empty-state" role="alert">
      <Info :size="32" aria-hidden="true" />
      <h2>{{ t('dashboard.loadFailed') }}</h2>
      <p>{{ translateAppError(store.loadError) }}</p>
      <AppButton variant="secondary" @click="retryLoad">{{ t('common.retry') }}</AppButton>
    </div>
    <Transition v-else-if="provider" name="provider-switch" mode="out-in">
      <div :key="provider.id" class="provider-detail">
        <div class="key-list-toolbar">
          <label class="search-field">
            <Search :size="15" aria-hidden="true" />
            <input v-model="keyQuery" type="search" :placeholder="t('dashboard.keySearchPlaceholder')" :aria-label="t('dashboard.keySearchPlaceholder')" />
          </label>
          <div class="key-list-actions">
            <span class="key-count-summary">{{ t('providers.keyCount', { count: provider.keys.length }) }}</span>
            <AppButton variant="ghost" size="icon-sm" :loading="checkingProviderId === provider.id"
              :disabled="!provider.validationSupported || !provider.keys.length || Boolean(checkingProviderId) || provider.keys.some((key) => key.status === 'checking')"
              :title="t(provider.validationSupported ? 'dashboard.refreshStatus' : 'dashboard.validationUnsupported')" :aria-label="t(provider.validationSupported ? 'dashboard.refreshStatus' : 'dashboard.validationUnsupported')"
              @click="handleRefreshProvider(provider.id, $event)"><RefreshCw :size="16" /></AppButton>
            <AppButton variant="primary" @click="openCreateKeyDialog(provider)"><Plus :size="15" /><span>{{ t('dashboard.addKey') }}</span></AppButton>
          </div>
        </div>
        <div v-if="visibleKeys.length" class="key-table-wrap provider-key-panel">
          <table class="key-table" :aria-label="t('dashboard.keysForProvider', { name: provider.name })">
            <thead>
              <tr>
                <th>{{ t("dashboard.columns.remark") }}</th>
                <th>{{ t("dashboard.columns.maskedKey") }}</th>
                <th>{{ t("dashboard.columns.status") }}</th>
                <th>{{ t("dashboard.columns.actions") }}</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="key in visibleKeys" :key="key.id">
                <td>{{ key.remark || t("keyDialog.unnamed") }}</td>
                <td class="masked-key">
                  <code>{{ key.maskedValue }}</code>
                </td>
                <td><StatusBadge :status="key.status" :error-code="key.checkErrorCode" /></td>
                <td><span class="key-actions">
                  <AppButton
                    variant="ghost"
                    size="icon-sm"
                    :loading="key.status === 'checking'"
                    :disabled="!provider.validationSupported"
                    :title="t(provider.validationSupported ? 'dashboard.actions.checkKey' : 'dashboard.validationUnsupported')"
                    :aria-label="t(provider.validationSupported ? 'dashboard.actions.checkKey' : 'dashboard.validationUnsupported')"
                    @click="handleCheckKey(provider.id, key.id)"
                  >
                    <ShieldCheck :size="15" :stroke-width="2" />
                  </AppButton>
                  <AppButton
                    variant="ghost"
                    size="icon-sm"
                    :title="t('dashboard.actions.editKey')"
                    :aria-label="t('dashboard.actions.editKey')"
                    @click="openEditKeyDialog(provider, key)"
                  >
                    <SquarePen :size="15" :stroke-width="2" />
                  </AppButton>
                  <AppButton
                    :variant="copiedKeyId === key.id ? 'success' : 'ghost'"
                    size="icon-sm"
                    :title="t(copiedKeyId === key.id ? 'dashboard.actions.copied' : 'dashboard.actions.copyKey')"
                    :aria-label="t(copiedKeyId === key.id ? 'dashboard.actions.copied' : 'dashboard.actions.copyKey')"
                    @click="handleCopy(key.id)"
                  >
                    <Check v-if="copiedKeyId === key.id" :size="14" :stroke-width="2.2" />
                    <Copy v-else :size="15" :stroke-width="2" />
                  </AppButton>
                  <AppButton
                    variant="danger"
                    size="icon-sm"
                    :title="t('dashboard.actions.deleteKey')"
                    :aria-label="t('dashboard.actions.deleteKey')"
                    @click="requestDeleteKey(provider.id, key.id)"
                  >
                    <Trash2 :size="15" :stroke-width="2" />
                  </AppButton>
                </span></td>
              </tr>
            </tbody>
          </table>
          <p class="key-disclosure">
            <Info :size="14" :stroke-width="1.8" />
            <span>{{ t("dashboard.keyDisclosure") }}</span>
          </p>
        </div>
        <div v-else class="empty-state">
          <KeyRound :size="32" aria-hidden="true" />
          <h2>{{ t(provider.keys.length ? 'dashboard.empty.noKeysMatch' : 'dashboard.empty.noKeysTitle') }}</h2>
          <p>{{ t(provider.keys.length ? 'dashboard.empty.noMatchesDescription' : 'dashboard.empty.noKeysDescription') }}</p>
          <AppButton variant="secondary" @click="provider.keys.length ? keyQuery = '' : openCreateKeyDialog(provider)">{{ t(provider.keys.length ? 'dashboard.empty.clearSearch' : 'dashboard.addKey') }}</AppButton>
        </div>
      </div>
    </Transition>
    <div v-else class="empty-state">
      <Search :size="32" aria-hidden="true" />
      <h2>{{ t('dashboard.empty.noProvidersTitle') }}</h2>
      <p>{{ t('dashboard.empty.noProvidersDescription') }}</p>
      <AppButton variant="secondary" @click="configDialogOpen = true">{{ t('providers.addProvider') }}</AppButton>
    </div>

    <Transition name="toast"><p v-if="notice" class="toast" role="status">{{ notice }}</p></Transition>
    <ProviderConfigDialog :open="configDialogOpen" :configured-providers="store.providers" @close="configDialogOpen = false" @add-builtin="addBuiltinProvider" @add-custom="addCustomProvider" />
    <ApiKeyDialog
      :open="Boolean(keyDialogProvider)"
      :provider-name="keyDialogProvider?.name ?? ''"
      :mode="editingKey ? 'edit' : 'create'"
      :initial-remark="editingKey?.remark ?? ''"
      @close="closeKeyDialog"
      @save="saveKey"
    />
    <ConfirmDialog :open="Boolean(deleteTarget)" :title="t('dashboard.deleteKeyDialog.title')" :message="t('dashboard.deleteKeyDialog.message')" @close="deleteTarget = null" @confirm="deleteKey" />
  </section>
</template>

<style scoped>
.dashboard-view .view-toolbar { min-height: var(--app-heading-height); padding: 0; margin-bottom: 16px; }
.dashboard-heading { flex: 1; min-width: 0; }
.dashboard-title { display: flex; flex: 1; align-items: center; gap: 9px; min-width: 0; }
.dashboard-title h1 { min-width: 0; max-width: 40%; flex-shrink: 0; font-size: 14px; line-height: 1.4; font-weight: 600; overflow: hidden; white-space: nowrap; text-overflow: ellipsis; }
.dashboard-title .provider-tag { flex-shrink: 0; }
.dashboard-title .provider-endpoint-link { flex: 1; min-width: 0; max-width: none; }
.dashboard-title :deep(.provider-avatar) { width: 24px; height: 24px; flex-basis: 24px; border-radius: 7px; }
.dashboard-title :deep(.provider-avatar img:not(.provider-avatar__custom-image)) { width: 18px; height: 18px; }
.dashboard-title :deep(.provider-abbr) { font-size: 11px; }
.toolbar-actions { flex-shrink: 0; }
.key-count-summary { color: var(--text-muted); font-size: 12px; white-space: nowrap; }
.key-list-toolbar { display: flex; align-items: center; justify-content: space-between; flex-wrap: wrap; gap: 12px; margin-bottom: 16px; }
.key-list-toolbar .search-field { min-width: 0; flex: 0 1 250px; }
.key-list-actions { display: flex; align-items: center; flex-shrink: 0; gap: 8px; margin-left: auto; }
.provider-key-panel { overflow-x: auto; padding: 8px 16px 12px; border: 1px solid var(--border); border-radius: 14px; background: var(--surface-card); }
.key-table { min-width: 480px; }
.key-table th:nth-child(1) { width: 24%; }
.key-table th:nth-child(2) { width: auto; }
.key-table th:nth-child(3) { width: 112px; }
.key-table th:nth-child(4) { width: 138px; }
.key-table td { overflow-wrap: anywhere; }
.key-table .masked-key { display: table-cell; vertical-align: middle; }
.masked-key code { display: inline-block; max-width: 100%; vertical-align: middle; }
.key-actions { display: flex; width: 100%; }
.key-actions :deep(.app-btn) { flex: 0 0 28px; }
.key-table :deep(.status-badge) { max-width: 100%; }
.key-table :deep(.status-badge i) { flex-shrink: 0; }
.provider-heading-enter-active, .provider-heading-leave-active { transition: opacity .12s ease; }
.provider-switch-enter-active, .provider-switch-leave-active { transition: opacity .15s ease, transform .15s cubic-bezier(.16, 1, .3, 1); }
.provider-heading-enter-from, .provider-heading-leave-to, .provider-switch-enter-from, .provider-switch-leave-to { opacity: 0; }
.provider-switch-enter-from { transform: translateY(4px); }
.provider-switch-leave-to { transform: translateY(-4px); }
.provider-switch-leave-active, .provider-heading-leave-active { pointer-events: none; }
.is-spinning { animation: spin .8s linear infinite; }
@keyframes spin { to { transform: rotate(360deg); } }
@media (prefers-reduced-motion: reduce) {
  .provider-heading-enter-active, .provider-heading-leave-active, .provider-switch-enter-active, .provider-switch-leave-active { transition: none; }
  .provider-switch-enter-from, .provider-switch-leave-to { transform: none; }
}
</style>
