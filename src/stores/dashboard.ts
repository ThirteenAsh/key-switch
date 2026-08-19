import { computed, ref } from "vue";
import { defineStore } from "pinia";
import { builtinProviderCatalog } from "../data/providerCatalog";
import { checkProviderKeys, createApiKey, createProvider, deleteApiKey, listProviders, reorderProviders, updateProvider } from "../api/app";
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
  function reorderProvidersLocally(from: number, to: number) { const [moved] = providers.value.splice(from, 1); providers.value.splice(to, 0, moved); void reorderProviders(providers.value.map((p) => p.id)); }
  async function addKey(input: { providerId: string; remark: string; value: string }) { const key = await createApiKey(input); const provider = providers.value.find((p) => p.id === input.providerId); if (provider) provider.keys.push(key); }
  async function deleteKey(providerId: string, keyId: string) { await deleteApiKey(keyId); const provider = providers.value.find((p) => p.id === providerId); if (provider) provider.keys = provider.keys.filter((key) => key.id !== keyId); }
  async function checkKeys(providerId: string) { const keys = await checkProviderKeys(providerId); const provider = providers.value.find((p) => p.id === providerId); if (provider) provider.keys = keys; }
  return { providers, query, filteredProviders, summary, expandedProviderId, toggleProvider, load, addBuiltinProvider, addCustomProvider, updateProviderConfiguration, reorderProviders: reorderProvidersLocally, addKey, deleteKey, checkKeys };
});
