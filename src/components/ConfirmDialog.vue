<script setup lang="ts">
import { AlertTriangle, X } from "@lucide/vue";
import AppButton from "./ui/AppButton.vue";

defineProps<{ open: boolean; title: string; message: string; confirmLabel?: string }>();
const emit = defineEmits<{ close: []; confirm: [] }>();
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
            <AppButton variant="ghost" size="icon-sm" aria-label="关闭" @click="emit('close')"><X :size="15" /></AppButton>
          </header>
          <footer>
            <AppButton variant="secondary" @click="emit('close')">取消</AppButton>
            <AppButton variant="danger" @click="emit('confirm')">{{ confirmLabel ?? '确认删除' }}</AppButton>
          </footer>
        </section>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.confirm-fade-enter-active,.confirm-fade-leave-active { transition: opacity .15s ease; }
.confirm-fade-enter-from,.confirm-fade-leave-to { opacity: 0; }
.confirm-dialog { width: 400px; max-width: calc(100vw - 32px); padding: 22px; border-radius: 16px; background: #fff; box-shadow: 0 25px 60px -15px rgba(15,23,42,.25); }
header { display:flex; align-items:flex-start; gap:12px; } header > .app-btn { margin-left:auto; } h2,p { margin:0; } h2 { font-size:17px; color:#0f172a; } p { margin-top:5px; color:#64748b; font-size:13px; line-height:1.6; }.confirm-dialog__icon { display:grid; place-items:center; flex:0 0 38px; width:38px; height:38px; color:#dc2626; border-radius:10px; background:#fef2f2; } footer { display:flex; justify-content:flex-end; gap:10px; margin-top:24px; }
</style>
