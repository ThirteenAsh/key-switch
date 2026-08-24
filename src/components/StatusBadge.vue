<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import type { KeyStatus } from "../types/domain";

const props = defineProps<{ status: KeyStatus }>();
const { t } = useI18n();

const statusText = computed(() => t(`status.${props.status}`));
</script>

<template>
  <span class="status-badge" :class="`status-badge--${status}`">
    <i aria-hidden="true" />
    {{ statusText }}
  </span>
</template>
