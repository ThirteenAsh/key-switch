<script setup lang="ts">
import { computed } from "vue";
import type { KeyStatus } from "../types/domain";

const props = defineProps<{ status: KeyStatus }>();

const statusText = computed(() => ({
  valid: "可用",
  untested: "未检测",
  invalid: "无效",
  error: "检测异常",
  checking: "检测中",
}[props.status]));
</script>

<template>
  <span class="status-badge" :class="`status-badge--${status}`">
    <i aria-hidden="true" />
    {{ statusText }}
  </span>
</template>
