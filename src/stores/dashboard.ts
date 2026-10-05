import { computed, ref } from "vue";
import { defineStore } from "pinia";
import { builtinProviderCatalog, getBuiltinProviderName } from "../data/providerCatalog";
import { checkApiKey, checkProviderKeys, createApiKey, createProvider, deleteApiKey, deleteProvider, listProviders, reorderProviders, updateApiKey, updateProvider } from "../api/app";
import type { ProviderSummary, ProviderValidation } from "../types/domain";
import type { AppLocale } from "../i18n/locale";

export const useDashboardStore = defineStore("dashboard", () => {
  const providers = ref<ProviderSummary[]>([]);
  const selectedProviderId = ref("");
  const loading = ref(true);
  const loadError = ref<unknown>(null);
  const sorting = ref(false);
  const summary = computed(() => { const keys = providers.value.flatMap((p) => p.keys); return { providerCount: providers.value.length, keyCount: keys.length, availableKeyCount: keys.filter((key) => key.status === "valid").length }; });
  const selectedProvider = computed(() => providers.value.find((provider) => provider.id === selectedProviderId.value) ?? providers.value[0]);
  async function load() {
    if (sorting.value) return;
    loading.value = true;
    loadError.value = null;
    try {
      providers.value = await listProviders();
      if (!providers.value.some((provider) => provider.id === selectedProviderId.value)) {
        selectedProviderId.value = providers.value[0]?.id ?? "";
      }
    }
    catch (error) { loadError.value = error; throw error; }
    finally { loading.value = false; }
  }
  async function addBuiltinProvider(id: string, locale: AppLocale) {
    const provider = builtinProviderCatalog.find((item) => item.id === id);
    if (!provider || providers.value.some((item) => item.id === id)) return false;
    providers.value.push(await createProvider({
      id: provider.id,
      name: getBuiltinProviderName(provider, locale),
      abbreviation: provider.abbreviation,
      tone: provider.tone,
      logo: provider.logo,
      kind: "builtin",
      platformUrl: provider.platformUrl,
      validation: { mode: "none" },
    }));
    selectedProviderId.value = id;
    return true;
  }
  async function addCustomProvider(input: { name: string; platformUrl: string; logo?: string; validation: ProviderValidation }) {
    const { name, platformUrl, logo, validation } = input;
    const normalized = name.trim(); if (!normalized || providers.value.some((p) => p.name === normalized)) return false;
    const id = `custom-${crypto.randomUUID()}`;
    providers.value.push(await createProvider({ id, name: normalized, abbreviation: normalized.slice(0, 2).toUpperCase(), tone: "gray", kind: "custom", platformUrl: platformUrl.trim() || undefined, logo, validation })); selectedProviderId.value = id; return true;
  }
  async function updateProviderConfiguration(id: string, name: string, platformUrl: string, validation: ProviderValidation) {
    const updated = await updateProvider({ id, name, platformUrl: platformUrl.trim() || undefined, validation }); const index = providers.value.findIndex((p) => p.id === id); if (index < 0) return false; providers.value[index] = updated; return true;
  }
  async function removeProvider(providerId: string) { await deleteProvider(providerId); providers.value = providers.value.filter((provider) => provider.id !== providerId); if (selectedProviderId.value === providerId) selectedProviderId.value = ""; }
  async function reorderProvidersLocally(from: number, to: number) {
    if (sorting.value || from === to || from < 0 || to < 0 || from >= providers.value.length || to >= providers.value.length) return;
    selectedProviderId.value = selectedProvider.value?.id ?? selectedProviderId.value;
    const previousIds = providers.value.map((provider) => provider.id);
    const [moved] = providers.value.splice(from, 1);
    providers.value.splice(to, 0, moved);
    sorting.value = true;
    try { await reorderProviders(providers.value.map((provider) => provider.id)); }
    catch (error) {
      // 只恢复顺序，保留排序期间发生的 Key 更新和新供应商。
      const positions = new Map(previousIds.map((id, index) => [id, index]));
      providers.value.sort((a, b) => (positions.get(a.id) ?? previousIds.length) - (positions.get(b.id) ?? previousIds.length));
      throw error;
    } finally { sorting.value = false; }
  }
  async function addKey(input: { providerId: string; remark: string; value: string }) { const key = await createApiKey(input); const provider = providers.value.find((p) => p.id === input.providerId); if (provider) provider.keys.push(key); }
  async function replaceKey(providerId: string, input: { id: string; remark: string; value: string }) { const updated = await updateApiKey(input); const provider = providers.value.find((p) => p.id === providerId); const index = provider?.keys.findIndex((key) => key.id === input.id) ?? -1; if (provider && index >= 0) provider.keys[index] = updated; }
  async function deleteKey(providerId: string, keyId: string) { await deleteApiKey(keyId); const provider = providers.value.find((p) => p.id === providerId); if (provider) provider.keys = provider.keys.filter((key) => key.id !== keyId); }
  async function checkKeys(providerId: string) {
    const provider = providers.value.find((item) => item.id === providerId);
    const previousStatuses = provider?.keys.map((key) => key.status) ?? [];
    if (provider) provider.keys.forEach((key) => { key.status = "checking"; });
    try {
      const keys = await checkProviderKeys(providerId);
      if (provider) provider.keys = keys;
      return keys;
    } catch (error) {
      if (provider) provider.keys.forEach((key, index) => { key.status = previousStatuses[index] ?? "untested"; });
      throw error;
    }
  }
  async function checkKey(providerId: string, keyId: string) {
    const provider = providers.value.find((item) => item.id === providerId);
    const key = provider?.keys.find((item) => item.id === keyId);
    const previousStatus = key?.status ?? "untested";
    if (key) key.status = "checking";
    try {
      const updated = await checkApiKey(providerId, keyId);
      const index = provider?.keys.findIndex((item) => item.id === keyId) ?? -1;
      if (provider && index >= 0) provider.keys[index] = updated;
      return updated;
    } catch (error) {
      if (key) key.status = previousStatus;
      throw error;
    }
  }
  return { providers, summary, selectedProviderId, selectedProvider, loading, loadError, sorting, load, addBuiltinProvider, addCustomProvider, updateProviderConfiguration, removeProvider, reorderProviders: reorderProvidersLocally, addKey, replaceKey, deleteKey, checkKey, checkKeys };
});
