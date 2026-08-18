<script setup lang="ts">
import { computed } from "vue";

export interface ButtonProps {
  variant?: "primary" | "secondary" | "ghost" | "danger" | "success" | "accent";
  size?: "sm" | "md" | "lg" | "icon" | "icon-sm";
  loading?: boolean;
  disabled?: boolean;
  type?: "button" | "submit" | "reset";
}

const props = withDefaults(defineProps<ButtonProps>(), {
  variant: "primary",
  size: "md",
  loading: false,
  disabled: false,
  type: "button",
});

const emit = defineEmits<{
  (e: "click", event: MouseEvent): void;
}>();

const handleClick = (e: MouseEvent) => {
  if (props.disabled || props.loading) return;
  emit("click", e);
};

const classes = computed(() => [
  "app-btn",
  `app-btn--${props.variant}`,
  `app-btn--${props.size}`,
  {
    "is-loading": props.loading,
    "is-disabled": props.disabled || props.loading,
  },
]);
</script>

<template>
  <button
    :type="type"
    :class="classes"
    :disabled="disabled || loading"
    @click="handleClick"
  >
    <!-- 加载中 Spinner -->
    <svg
      v-if="loading"
      class="app-btn__spinner"
      viewBox="0 0 24 24"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <circle
        cx="12"
        cy="12"
        r="9"
        stroke="currentColor"
        stroke-width="2.5"
        stroke-linecap="round"
        stroke-dasharray="42 18"
      />
    </svg>

    <!-- 按钮内容 -->
    <span class="app-btn__content" :style="{ opacity: loading ? 0 : 1 }">
      <slot />
    </span>
  </button>
</template>

<style scoped>
.app-btn {
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  font-family: inherit;
  font-weight: 500;
  border: none;
  outline: none;
  cursor: pointer;
  user-select: none;
  white-space: nowrap;
  text-decoration: none;
  transition: all 0.18s cubic-bezier(0.16, 1, 0.3, 1);
  -webkit-font-smoothing: antialiased;
}

.app-btn:focus-visible {
  box-shadow: 0 0 0 2px rgba(15, 23, 42, 0.2), 0 0 0 4px rgba(15, 23, 42, 0.08);
}

.app-btn__content {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  transition: opacity 0.15s ease;
}

/* 尺寸规范 */
.app-btn--sm {
  height: 32px;
  padding: 0 11px;
  font-size: 12.5px;
  border-radius: 8px;
}

.app-btn--md {
  height: 36px;
  padding: 0 14px;
  font-size: 13px;
  border-radius: 9px;
}

.app-btn--lg {
  height: 42px;
  padding: 0 18px;
  font-size: 14px;
  border-radius: 10px;
}

.app-btn--icon {
  width: 34px;
  height: 34px;
  padding: 0;
  border-radius: 8px;
}

.app-btn--icon-sm {
  width: 28px;
  height: 28px;
  padding: 0;
  border-radius: 7px;
}

/* 变体质感（现代极简高级质感，彻底消除廉价纯色块感） */

/* 1. Primary: 现代深邃曜黑（Linear / Raycast / Apple 风格）+ 顶部微高光 + 柔和弥散阴影 */
.app-btn--primary {
  background: linear-gradient(180deg, #1e293b 0%, #0f172a 100%);
  color: #ffffff;
  box-shadow:
    0 1px 2px rgba(0, 0, 0, 0.12),
    0 3px 8px rgba(15, 23, 42, 0.12),
    inset 0 1px 0 rgba(255, 255, 255, 0.16);
}
.app-btn--primary:hover:not(.is-disabled) {
  background: linear-gradient(180deg, #334155 0%, #1e293b 100%);
  box-shadow:
    0 2px 4px rgba(0, 0, 0, 0.12),
    0 6px 14px rgba(15, 23, 42, 0.18),
    inset 0 1px 0 rgba(255, 255, 255, 0.24);
  transform: translateY(-1px);
}
.app-btn--primary:active:not(.is-disabled) {
  transform: scale(0.98) translateY(0);
  box-shadow:
    0 1px 2px rgba(0, 0, 0, 0.15),
    0 2px 4px rgba(15, 23, 42, 0.1);
}

/* 2. Accent: 极光蓝紫微渐变（若需要彩色主调） */
.app-btn--accent {
  background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%);
  color: #ffffff;
  box-shadow:
    0 1px 2px rgba(0, 0, 0, 0.1),
    0 4px 12px rgba(99, 102, 241, 0.28),
    inset 0 1px 0 rgba(255, 255, 255, 0.22);
}
.app-btn--accent:hover:not(.is-disabled) {
  background: linear-gradient(135deg, #4f46e5 0%, #4338ca 100%);
  box-shadow:
    0 2px 4px rgba(0, 0, 0, 0.1),
    0 6px 16px rgba(99, 102, 241, 0.36),
    inset 0 1px 0 rgba(255, 255, 255, 0.3);
  transform: translateY(-1px);
}
.app-btn--accent:active:not(.is-disabled) {
  transform: scale(0.98);
}

/* 3. Secondary: 柔和半透明浅灰底衬 */
.app-btn--secondary {
  background: rgba(15, 23, 42, 0.045);
  color: #334155;
}
.app-btn--secondary:hover:not(.is-disabled) {
  background: rgba(15, 23, 42, 0.08);
  color: #0f172a;
  transform: translateY(-1px);
}
.app-btn--secondary:active:not(.is-disabled) {
  background: rgba(15, 23, 42, 0.1);
  transform: scale(0.98);
}

/* 4. Ghost: 纯平幽灵按钮（工具栏/行内操作标配） */
.app-btn--ghost {
  background: transparent;
  color: #64748b;
}
.app-btn--ghost:hover:not(.is-disabled) {
  background: rgba(15, 23, 42, 0.05);
  color: #0f172a;
}
.app-btn--ghost:active:not(.is-disabled) {
  background: rgba(15, 23, 42, 0.08);
  transform: scale(0.96);
}

/* 5. Danger: 柔和暗红底衬与微光 */
.app-btn--danger {
  background: rgba(239, 68, 68, 0.08);
  color: #ef4444;
}
.app-btn--danger:hover:not(.is-disabled) {
  background: rgba(239, 68, 68, 0.16);
  color: #dc2626;
  box-shadow: 0 3px 10px rgba(239, 68, 68, 0.15);
  transform: translateY(-1px);
}
.app-btn--danger:active:not(.is-disabled) {
  background: rgba(239, 68, 68, 0.2);
  transform: scale(0.98);
}

/* 6. Success: 柔和翠绿底衬与微光 */
.app-btn--success {
  background: rgba(16, 185, 129, 0.1);
  color: #059669;
}
.app-btn--success:hover:not(.is-disabled) {
  background: rgba(16, 185, 129, 0.18);
  color: #047857;
  box-shadow: 0 3px 10px rgba(16, 185, 129, 0.15);
  transform: translateY(-1px);
}
.app-btn--success:active:not(.is-disabled) {
  background: rgba(16, 185, 129, 0.22);
  transform: scale(0.98);
}

/* 禁用与加载中 */
.is-disabled {
  opacity: 0.45;
  cursor: not-allowed;
  transform: none !important;
  box-shadow: none !important;
}

.app-btn__spinner {
  position: absolute;
  width: 16px;
  height: 16px;
  animation: app-btn-spin 0.75s linear infinite;
}

@keyframes app-btn-spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}
</style>
