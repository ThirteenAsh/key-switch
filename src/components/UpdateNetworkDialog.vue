<script setup lang="ts">
import { nextTick, onBeforeUnmount, ref, watch } from "vue";
import { WifiOff, X } from "@lucide/vue";
import { useI18n } from "vue-i18n";
import AppButton from "./ui/AppButton.vue";

const props = defineProps<{ open: boolean }>();
const emit = defineEmits<{ close: []; confirm: [] }>();
const { t } = useI18n();
const dialog = ref<HTMLElement | null>(null);
let previousFocus: HTMLElement | null = null;
let background: HTMLElement | null = null;
let wasInert = false;

function restoreBackground() {
  if (background && !wasInert) background.removeAttribute("inert");
  background = null;
}

function restoreFocus() {
  if (props.open) return;
  if (previousFocus?.isConnected) previousFocus.focus();
  else document.querySelector<HTMLButtonElement>(".app-shell button:not(:disabled)")?.focus();
  previousFocus = null;
}

watch(() => props.open, async (open) => {
  if (!open) {
    restoreBackground();
    return;
  }
  previousFocus = document.activeElement instanceof HTMLElement ? document.activeElement : null;
  await nextTick();
  if (!props.open) return;
  background = document.querySelector<HTMLElement>(".app-shell");
  wasInert = background?.hasAttribute("inert") ?? false;
  background?.setAttribute("inert", "");
  dialog.value?.querySelector<HTMLButtonElement>("[data-initial-focus]")?.focus();
}, { immediate: true });

function onKeydown(event: KeyboardEvent) {
  if (event.key === "Escape") {
    event.preventDefault();
    event.stopPropagation();
    emit("close");
  } else if (event.key === "Tab") {
    const buttons = dialog.value?.querySelectorAll<HTMLButtonElement>("button:not(:disabled)");
    if (!buttons?.length) return;
    const first = buttons[0];
    const last = buttons[buttons.length - 1];
    if (event.shiftKey && document.activeElement === first) {
      event.preventDefault();
      last.focus();
    } else if (!event.shiftKey && document.activeElement === last) {
      event.preventDefault();
      first.focus();
    }
  }
}

onBeforeUnmount(() => {
  restoreBackground();
  restoreFocus();
  if (props.open) emit("close");
});
</script>

<template>
  <Teleport to="body">
    <Transition name="network-dialog" @after-leave="restoreFocus">
      <div v-if="open" class="dialog-backdrop network-backdrop" @click.self="emit('close')">
        <section ref="dialog" class="network-dialog" role="alertdialog" aria-modal="true" aria-labelledby="update-network-title" aria-describedby="update-network-description" @keydown="onKeydown">
          <header>
            <span class="network-dialog__icon"><WifiOff :size="20" :stroke-width="2" /></span>
            <div class="network-dialog__heading">
              <h2 id="update-network-title">{{ t("update.proxyFailedTitle") }}</h2>
            </div>
            <AppButton variant="ghost" size="icon-sm" :aria-label="t('common.close')" @click="emit('close')"><X :size="16" /></AppButton>
          </header>
          <div id="update-network-description" class="network-dialog__description">
            <p>{{ t("update.proxyFailedMessage") }}</p>
            <p>{{ t("update.directRetryHint") }}</p>
          </div>
          <footer>
            <AppButton variant="secondary" data-initial-focus @click="emit('close')">{{ t("common.cancel") }}</AppButton>
            <AppButton variant="primary" @click="emit('confirm')">{{ t("update.retryDirect") }}</AppButton>
          </footer>
        </section>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.network-backdrop { z-index: 90; }
.network-dialog { width: 440px; max-width: calc(100vw - 32px); padding: 22px; border: 1px solid var(--border); border-radius: 18px; background: var(--surface-raised); box-shadow: 0 28px 70px -18px rgba(15,23,42,.3); }
header { display: flex; align-items: center; gap: 12px; }
.network-dialog__icon { display: grid; place-items: center; flex: 0 0 38px; height: 38px; border-radius: 10px; color: var(--warning-text); background: var(--warning-surface); }
.network-dialog__heading { min-width: 0; flex: 1; }
h2, p { margin: 0; }
h2 { color: var(--text-primary); font-size: 17px; line-height: 1.5; }
.network-dialog__description { margin-top: 8px; padding-left: 50px; color: var(--text-muted); font-size: 13px; line-height: 1.7; }
.network-dialog__description p + p { margin-top: 4px; }
footer { display: flex; justify-content: flex-end; flex-wrap: wrap; gap: 10px; margin-top: 22px; }
.network-dialog-enter-active, .network-dialog-leave-active { transition: opacity .18s ease; }
.network-dialog-enter-active .network-dialog, .network-dialog-leave-active .network-dialog { transition: transform .22s cubic-bezier(.22,1,.36,1), opacity .18s ease; }
.network-dialog-enter-from, .network-dialog-leave-to { opacity: 0; }
.network-dialog-enter-from .network-dialog { opacity: 0; transform: translateY(10px) scale(.97); }
.network-dialog-leave-to .network-dialog { opacity: 0; transform: translateY(6px) scale(.98); }
@media (prefers-reduced-motion: reduce) {
  .network-dialog-enter-active, .network-dialog-leave-active, .network-dialog-enter-active .network-dialog, .network-dialog-leave-active .network-dialog { transition: none; }
  .network-dialog-enter-from .network-dialog, .network-dialog-leave-to .network-dialog { transform: none; }
}
</style>
