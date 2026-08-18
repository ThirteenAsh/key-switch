<script setup lang="ts">
import type { ProviderSummary } from "../types/domain";

defineProps<{ provider: Pick<ProviderSummary, "name" | "abbreviation" | "tone" | "logo"> }>();
</script>

<template>
  <span class="provider-avatar" :class="`provider-avatar--${provider.tone}`">
    <img v-if="provider.logo" :src="provider.logo" :alt="`${provider.name} 图标`" />
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
  border: 1px solid #e2e8f0;
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

.provider-abbr {
  font-size: 13px;
  font-weight: 700;
  color: #334155;
  line-height: 1;
}

.provider-avatar--blue {
  border-color: #cbd5e1;
}
.provider-avatar--violet {
  border-color: #cbd5e1;
}
.provider-avatar--orange {
  border-color: #cbd5e1;
}
.provider-avatar--indigo {
  border-color: #cbd5e1;
}
.provider-avatar--gray {
  border-color: #cbd5e1;
}
</style>
