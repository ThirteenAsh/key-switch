import { computed, ref } from "vue";
import { defineStore } from "pinia";
import { builtinProviderCatalog } from "../data/providerCatalog";
import { checkApiKey, checkProviderKeys, createApiKey, createProvider, deleteApiKey, deleteProvider, listProviders, reorderProviders, updateApiKey, updateProvider } from "../api/app";
import type { ProviderSummary } from "../types/domain";

export const useDashboardStore = defineStore("dashboard", () => {
  const providers = ref<ProviderSummary[]>([]);
  const query = ref("");
  const expandedProviderId = ref("");
  const filteredProviders = computed(() => {
    const keyword = query.value.trim().toLocaleLowerCase();
    return !keyword ? providers.value : providers.value.filter((p) => p.name.toLocaleLowerCase().includes(keyword) || p.keys.some((key) => key.remark.toLocaleLowerCase().includes(keyword)));
  });
  const summary = computed(() => { const keys = providers.value.flatMap((p) => p.keys); return { providerCount: providers.value.length, keyCount: keys.length, availableKeyCount: keys.filter((key) => key.status === "valid").length }; });
  function toggleProvider(id: string) { expandedProviderId.value = expandedProviderId.value === id ? "" : id; }
  async function load() { providers.value = await listProviders(); }
  async function addBuiltinProvider(id: string) {
    const provider = builtinProviderCatalog.find((item) => item.id === id);
    if (!provider || providers.value.some((item) => item.id === id)) return false;
    providers.value.push(await createProvider({ ...provider, kind: "builtin" })); expandedProviderId.value = id; return true;
  }
  async function addCustomProvider(name: string, platformUrl: string, logo?: string) {
    const normalized = name.trim(); if (!normalized || providers.value.some((p) => p.name === normalized)) return false;
    const id = `custom-${crypto.randomUUID()}`;
    providers.value.push(await createProvider({ id, name: normalized, abbreviation: normalized.slice(0, 2).toUpperCase(), tone: "gray", kind: "custom", platformUrl: platformUrl.trim() || undefined, logo })); expandedProviderId.value = id; return true;
  }
  async function updateProviderConfiguration(id: string, name: string, platformUrl: string) {
    const updated = await updateProvider({ id, name, platformUrl: platformUrl.trim() || undefined }); const index = providers.value.findIndex((p) => p.id === id); if (index < 0) return false; providers.value[index] = updated; return true;
  }
  async function removeProvider(providerId: string) { await deleteProvider(providerId); providers.value = providers.value.filter((provider) => provider.id !== providerId); if (expandedProviderId.value === providerId) expandedProviderId.value = ""; }
  function reorderProvidersLocally(from: number, to: number) { const [moved] = providers.value.splice(from, 1); providers.value.splice(to, 0, moved); void reorderProviders(providers.value.map((p) => p.id)); }
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
  return { providers, query, filteredProviders, summary, expandedProviderId, toggleProvider, load, addBuiltinProvider, addCustomProvider, updateProviderConfiguration, removeProvider, reorderProviders: reorderProvidersLocally, addKey, replaceKey, deleteKey, checkKey, checkKeys };
});
