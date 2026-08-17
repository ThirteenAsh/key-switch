<script setup lang="ts">
import type { ProviderSummary } from "../types/domain";

defineProps<{ provider: Pick<ProviderSummary, "abbreviation" | "tone"> }>();
</script>

<template>
  <span class="provider-avatar" :class="`provider-avatar--${provider.tone}`">{{ provider.abbreviation }}</span>
</template>
