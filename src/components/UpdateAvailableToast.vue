<script setup lang="ts">
import { ArrowRight, Sparkles, X } from "@lucide/vue";
import { useI18n } from "vue-i18n";
import type { UpdateInfo } from "../api/app";
import AppButton from "./ui/AppButton.vue";

defineProps<{ update: UpdateInfo | null }>();
const emit = defineEmits<{ close: []; view: [] }>();
const { t } = useI18n();
</script>

<template>
  <Teleport to="body">
    <Transition name="update-toast">
      <aside v-if="update" class="update-toast" role="status" aria-live="polite">
        <span class="update-toast__icon" aria-hidden="true">
          <Sparkles :size="18" :stroke-width="2" />
        </span>
        <div class="update-toast__content">
          <strong>{{ t("update.available", { version: update.latestVersion }) }}</strong>
          <span>{{ t("update.current", { version: update.currentVersion }) }}</span>
          <AppButton variant="ghost" size="sm" @click="emit('view')">
            {{ t("update.view") }}
            <ArrowRight :size="14" :stroke-width="2" />
          </AppButton>
        </div>
        <AppButton class="update-toast__close" variant="ghost" size="icon-sm" :aria-label="t('update.remindLater')" @click="emit('close')">
          <X :size="15" />
        </AppButton>
      </aside>
    </Transition>
  </Teleport>
</template>

<style scoped>
.update-toast {
  position: fixed;
  z-index: 40;
  right: 20px;
  bottom: 62px;
  display: flex;
  width: min(340px, calc(100vw - 40px));
  gap: 12px;
  padding: 14px;
  color: var(--text-primary);
  border: 1px solid var(--border);
  border-radius: 14px;
  background: var(--surface-popover);
  box-shadow: 0 18px 48px -16px rgba(15, 23, 42, 0.36);
  backdrop-filter: blur(12px);
}

.update-toast__icon {
  display: grid;
  flex: 0 0 36px;
  width: 36px;
  height: 36px;
  color: var(--accent-text);
  place-items: center;
  border-radius: 10px;
  background: var(--indigo-surface);
}

.update-toast__content {
  display: flex;
  min-width: 0;
  flex: 1;
  flex-direction: column;
  align-items: flex-start;
}

.update-toast__content strong {
  font-size: 13.5px;
  line-height: 1.45;
}

.update-toast__content > span {
  margin: 2px 0 5px;
  color: var(--text-muted);
  font-size: 11.5px;
}

.update-toast__content :deep(.app-btn) {
  height: 27px;
  margin-left: -8px;
  padding: 0 8px;
  color: var(--accent-text);
}

.update-toast__close {
  flex: 0 0 auto;
  margin: -4px -4px 0 0;
}

.update-toast-enter-active,
.update-toast-leave-active {
  transition: opacity 0.2s ease, transform 0.24s cubic-bezier(0.16, 1, 0.3, 1);
}

.update-toast-enter-from,
.update-toast-leave-to {
  opacity: 0;
  transform: translateY(10px) scale(0.98);
}
</style>
