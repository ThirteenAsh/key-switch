<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import type { KeyCheckErrorCode, KeyStatus } from "../types/domain";

const props = defineProps<{ status: KeyStatus; errorCode?: KeyCheckErrorCode }>();
const { t } = useI18n();

const statusText = computed(() => t(`status.${props.status}`));
const detailText = computed(() => props.errorCode ? t(`statusReasons.${props.errorCode}`) : "");
</script>

<template>
  <span class="status-badge" :class="`status-badge--${status}`" :title="detailText || undefined" :aria-label="detailText ? `${statusText}：${detailText}` : statusText">
    <i aria-hidden="true" />
    {{ statusText }}
  </span>
</template>
