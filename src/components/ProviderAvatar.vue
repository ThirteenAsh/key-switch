<script setup lang="ts">
import { useI18n } from "vue-i18n";
import type { ProviderSummary } from "../types/domain";

defineProps<{ provider: Pick<ProviderSummary, "name" | "abbreviation" | "tone" | "logo" | "kind"> }>();
const { t } = useI18n();
</script>

<template>
  <span
    class="provider-avatar"
    :class="[`provider-avatar--${provider.tone}`, { 'provider-avatar--image': provider.logo }]"
  >
    <img v-if="provider.logo" :class="{ 'provider-avatar__custom-image': provider.kind === 'custom' }" :src="provider.logo" :alt="t('common.providerIconAlt', { name: provider.name })" />
    <span v-else class="provider-abbr">{{ provider.abbreviation }}</span>
  </span>
</template>

<style scoped>
.provider-avatar {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 38px;
  height: 38px;
  flex: 0 0 38px;
  border-radius: 10px;
  background: transparent;
  border: 1px solid var(--border);
  box-sizing: border-box;
  overflow: hidden;
}

.provider-avatar img {
  width: 22px;
  height: 22px;
  max-width: 22px;
  max-height: 22px;
  object-fit: contain;
  display: block;
  margin: auto;
}

.provider-avatar img.provider-avatar__custom-image {
  width: 100%;
  height: 100%;
  max-width: none;
  max-height: none;
  object-fit: cover;
}

:global(:root[data-theme="dark"] .provider-avatar--image) {
  background: rgba(255, 255, 255, 0.92);
}

.provider-abbr {
  font-size: 13px;
  font-weight: 700;
  color: var(--text-secondary);
  line-height: 1;
}

.provider-avatar--blue {
  border-color: var(--border-strong);
}
.provider-avatar--violet {
  border-color: var(--border-strong);
}
.provider-avatar--orange {
  border-color: var(--border-strong);
}
.provider-avatar--indigo {
  border-color: var(--border-strong);
}
.provider-avatar--gray {
  border-color: var(--border-strong);
}
</style>
