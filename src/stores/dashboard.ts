import { computed, ref } from "vue";
import { defineStore } from "pinia";
import { builtinProviderCatalog } from "../data/providerCatalog";
import type { ProviderSummary } from "../types/domain";

export const useDashboardStore = defineStore("dashboard", () => {
  const providers = ref<ProviderSummary[]>([]);
  const query = ref("");
  const expandedProviderId = ref("");

  const filteredProviders = computed(() => {
    const keyword = query.value.trim().toLocaleLowerCase();
    if (!keyword) return providers.value;

    return providers.value.filter((provider) =>
      provider.name.toLocaleLowerCase().includes(keyword) ||
      provider.keys.some((key) => key.remark.toLocaleLowerCase().includes(keyword)),
    );
  });

  const summary = computed(() => {
    const allKeys = providers.value.flatMap((provider) => provider.keys);
    return {
      providerCount: providers.value.length,
      keyCount: allKeys.length,
      availableKeyCount: allKeys.filter((key) => key.status === "valid").length,
    };
  });

  function toggleProvider(providerId: string) {
    expandedProviderId.value = expandedProviderId.value === providerId ? "" : providerId;
  }

  function reorderProviders(fromIndex: number, toIndex: number) {
    if (fromIndex < 0 || fromIndex >= providers.value.length) return;
    if (toIndex < 0 || toIndex >= providers.value.length) return;
    if (fromIndex === toIndex) return;

    const [moved] = providers.value.splice(fromIndex, 1);
    providers.value.splice(toIndex, 0, moved);
  }

  function addBuiltinProvider(providerId: string) {
    const provider = builtinProviderCatalog.find((item) => item.id === providerId);
    if (!provider || providers.value.some((item) => item.id === provider.id)) return false;

    providers.value.push({ ...provider, kind: "builtin", keys: [] });
    expandedProviderId.value = provider.id;
    return true;
  }

  function addCustomProvider(name: string, platformUrl: string, logo?: string) {
    const normalizedName = name.trim();
    if (!normalizedName || providers.value.some((provider) => provider.name === normalizedName)) return false;

    const id = `custom-${crypto.randomUUID()}`;
    providers.value.push({
      id,
      name: normalizedName,
      abbreviation: normalizedName.slice(0, 2).toUpperCase(),
      tone: "gray",
      kind: "custom",
      platformUrl: platformUrl.trim() || undefined,
      logo,
      keys: [],
    });
    expandedProviderId.value = id;
    return true;
  }

  function updateProviderConfiguration(providerId: string, name: string, platformUrl: string) {
    const provider = providers.value.find((item) => item.id === providerId);
    const normalizedName = name.trim();
    const normalizedPlatformUrl = platformUrl.trim();
    if (!provider || !normalizedName || !normalizedPlatformUrl) return false;

    if (providers.value.some((item) => item.id !== providerId && item.name === normalizedName)) return false;

    provider.name = normalizedName;
    provider.platformUrl = normalizedPlatformUrl;
    return true;
  }

  return {
    providers,
    query,
    filteredProviders,
    summary,
    expandedProviderId,
    toggleProvider,
    reorderProviders,
    addBuiltinProvider,
    addCustomProvider,
    updateProviderConfiguration,
  };
});
