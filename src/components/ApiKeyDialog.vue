<script setup lang="ts">
import { ref, watch } from "vue";
import { KeyRound, X } from "@lucide/vue";
import { useI18n } from "vue-i18n";
import AppButton from "./ui/AppButton.vue";
const props = withDefaults(defineProps<{ open: boolean; providerName: string; mode?: "create" | "edit"; initialRemark?: string }>(), {
  mode: "create",
  initialRemark: "",
});
const emit = defineEmits<{ close: []; save: [payload: { remark: string; value: string }] }>();
const { t } = useI18n();
const remark = ref(""); const value = ref(""); const error = ref("");
watch(() => props.open, (open) => { if (open) { remark.value = props.initialRemark; value.value = ""; error.value = ""; } });
function submit() {
  const nextRemark = remark.value.trim();
  const nextValue = value.value.trim();
  if (props.mode === "create" && !nextValue) { error.value = t("keyDialog.errors.keyRequired"); return; }
  if (props.mode === "edit" && !nextValue && nextRemark === props.initialRemark.trim()) {
    error.value = t("keyDialog.errors.noChanges");
    return;
  }
  emit("save", { remark: nextRemark, value: nextValue });
}
</script>
<template>
  <Teleport to="body">
    <Transition name="key-dialog-fade">
      <div v-if="open" class="dialog-backdrop" @click.self="emit('close')">
        <section class="key-dialog" role="dialog" aria-modal="true">
          <header><div><h2>{{ t(mode === 'edit' ? 'keyDialog.editTitle' : 'keyDialog.addTitle') }}</h2><p>{{ providerName }}</p></div><AppButton variant="ghost" size="icon-sm" :aria-label="t('common.close')" @click="emit('close')"><X :size="16" :stroke-width="2" /></AppButton></header>
          <form @submit.prevent="submit">
            <label>{{ t("keyDialog.remark") }}<input v-model="remark" maxlength="64" :placeholder="t('keyDialog.remarkPlaceholder')" /></label>
            <label>{{ t(mode === 'edit' ? 'keyDialog.newKey' : 'keyDialog.key') }}<input v-model="value" type="password" autocomplete="new-password" spellcheck="false" :placeholder="t(mode === 'edit' ? 'common.optional' : 'keyDialog.keyPlaceholder')" /></label>
            <p v-if="error" class="form-error">{{ error }}</p>
            <footer><AppButton variant="secondary" type="button" @click="emit('close')">{{ t("common.cancel") }}</AppButton><AppButton variant="primary" type="submit"><KeyRound :size="15" :stroke-width="2" />{{ t(mode === 'edit' ? 'keyDialog.saveChanges' : 'keyDialog.saveKey') }}</AppButton></footer>
          </form>
        </section>
      </div>
    </Transition>
  </Teleport>
</template>
<style scoped>
.key-dialog-fade-enter-active,
.key-dialog-fade-leave-active { transition: opacity .18s ease; }
.key-dialog-fade-enter-active .key-dialog,
.key-dialog-fade-leave-active .key-dialog { transition: transform .22s cubic-bezier(.22,1,.36,1), opacity .18s ease; }
.key-dialog-fade-enter-from,
.key-dialog-fade-leave-to { opacity: 0; }
.key-dialog-fade-enter-from .key-dialog { opacity: 0; transform: translateY(10px) scale(.97); }
.key-dialog-fade-leave-to .key-dialog { opacity: 0; transform: translateY(6px) scale(.98); }
.key-dialog { width: 460px; max-width: calc(100vw - 32px); padding: 24px; border: 1px solid var(--border); border-radius: 18px; background: var(--surface-raised); box-shadow: 0 28px 70px -18px rgba(15,23,42,.3); }
header, footer { display: flex; justify-content: space-between; align-items: center; gap: 12px; } h2,p { margin: 0; } h2 { color:var(--text-primary); font-size:18px; letter-spacing:-.01em; } header p { margin-top: 4px; color: var(--text-muted); font-size: 13px; } form { display:grid; gap: 16px; margin-top: 22px; } label { display:grid; gap: 7px; color:var(--text-secondary); font-size:13px; font-weight:500; } input { box-sizing:border-box; width:100%; height:40px; border:1px solid var(--border-strong); border-radius:10px; padding:0 12px; color:var(--text-primary); background:var(--surface-raised); font:inherit; transition:border-color .15s ease, box-shadow .15s ease; } input:focus-visible { outline:none; border-color:#6366f1; box-shadow:0 0 0 3px rgba(99,102,241,.14); } footer { justify-content:flex-end; margin-top:4px; }.form-error{color:var(--danger-text);font-size:13px}

@media (prefers-reduced-motion: reduce) {
  .key-dialog-fade-enter-active,
  .key-dialog-fade-leave-active,
  .key-dialog-fade-enter-active .key-dialog,
  .key-dialog-fade-leave-active .key-dialog { transition-duration: .01ms; }
}
</style>
