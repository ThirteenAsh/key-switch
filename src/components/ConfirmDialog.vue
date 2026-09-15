<script setup lang="ts">
import { AlertTriangle, X } from "@lucide/vue";
import { useI18n } from "vue-i18n";
import AppButton from "./ui/AppButton.vue";

defineProps<{ open: boolean; title: string; message: string; confirmLabel?: string }>();
const emit = defineEmits<{ close: []; confirm: [] }>();
const { t } = useI18n();
</script>

<template>
  <Teleport to="body">
    <Transition name="confirm-fade">
      <div v-if="open" class="dialog-backdrop" @click.self="emit('close')">
        <section class="confirm-dialog" role="alertdialog" aria-modal="true" aria-labelledby="confirm-dialog-title">
          <header>
            <span class="confirm-dialog__icon"><AlertTriangle :size="20" :stroke-width="2" /></span>
            <div>
              <h2 id="confirm-dialog-title">{{ title }}</h2>
              <p>{{ message }}</p>
            </div>
            <AppButton variant="ghost" size="icon-sm" :aria-label="t('common.close')" @click="emit('close')"><X :size="15" /></AppButton>
          </header>
          <footer>
            <AppButton variant="secondary" @click="emit('close')">{{ t('common.cancel') }}</AppButton>
            <AppButton variant="danger" @click="emit('confirm')">{{ confirmLabel ?? t('common.delete') }}</AppButton>
          </footer>
        </section>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.confirm-fade-enter-active,.confirm-fade-leave-active { transition: opacity .15s ease; }
.confirm-fade-enter-from,.confirm-fade-leave-to { opacity: 0; }
.confirm-dialog { width: 400px; max-width: calc(100vw - 32px); padding: 22px; border-radius: 16px; background: var(--surface-raised); box-shadow: 0 25px 60px -15px rgba(15,23,42,.25); }
header { display:flex; align-items:flex-start; gap:12px; } header > .app-btn { margin-left:auto; } h2,p { margin:0; } h2 { font-size:17px; color:var(--text-primary); } p { margin-top:5px; color:var(--text-muted); font-size:13px; line-height:1.6; }.confirm-dialog__icon { display:grid; place-items:center; flex:0 0 38px; width:38px; height:38px; color:var(--danger-text); border-radius:10px; background:var(--danger-surface); } footer { display:flex; justify-content:flex-end; gap:10px; margin-top:24px; }
</style>
