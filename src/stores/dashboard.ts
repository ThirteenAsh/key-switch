import { computed, ref } from "vue";
import { defineStore } from "pinia";
import { mockProviders } from "../data/mock";
import type { ProviderSummary } from "../types/domain";

export const useDashboardStore = defineStore("dashboard", () => {
  const providers = ref<ProviderSummary[]>(mockProviders);
  const query = ref("");
  const expandedProviderId = ref("openai");

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

  return { providers, query, filteredProviders, summary, expandedProviderId, toggleProvider };
});
