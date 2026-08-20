<script setup lang="ts">
import { ref, watch } from "vue";
import { KeyRound, X } from "@lucide/vue";
import AppButton from "./ui/AppButton.vue";
const props = withDefaults(defineProps<{ open: boolean; providerName: string; mode?: "create" | "edit"; initialRemark?: string }>(), {
  mode: "create",
  initialRemark: "",
});
const emit = defineEmits<{ close: []; save: [payload: { remark: string; value: string }] }>();
const remark = ref(""); const value = ref(""); const error = ref("");
watch(() => props.open, (open) => { if (open) { remark.value = props.initialRemark; value.value = ""; error.value = ""; } });
function submit() { if (!value.value.trim()) { error.value = "请输入 API Key"; return; } emit("save", { remark: remark.value.trim(), value: value.value.trim() }); }
</script>
<template>
  <Teleport to="body">
    <Transition name="key-dialog-fade">
      <div v-if="open" class="dialog-backdrop" @click.self="emit('close')">
        <section class="key-dialog" role="dialog" aria-modal="true">
          <header><div><h2>{{ mode === 'edit' ? '编辑 API Key' : '添加 API Key' }}</h2><p>{{ providerName }}</p></div><AppButton variant="ghost" size="icon-sm" aria-label="关闭" @click="emit('close')"><X :size="16" :stroke-width="2" /></AppButton></header>
          <form @submit.prevent="submit">
            <label>备注<input v-model="remark" maxlength="64" placeholder="例如：开发环境" /></label>
            <label>{{ mode === 'edit' ? '新的 API Key' : 'API Key' }}<input v-model="value" type="password" autocomplete="new-password" spellcheck="false" :placeholder="mode === 'edit' ? '输入新的 API Key' : '粘贴 API Key'" /></label>
            <p v-if="error" class="form-error">{{ error }}</p>
            <footer><AppButton variant="secondary" type="button" @click="emit('close')">取消</AppButton><AppButton variant="primary" type="submit"><KeyRound :size="15" :stroke-width="2" />{{ mode === 'edit' ? '确认替换' : '保存 Key' }}</AppButton></footer>
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
.key-dialog { width: 460px; max-width: calc(100vw - 32px); padding: 24px; border: 1px solid rgba(226,232,240,.9); border-radius: 18px; background: #fff; box-shadow: 0 28px 70px -18px rgba(15,23,42,.3); }
header, footer { display: flex; justify-content: space-between; align-items: center; gap: 12px; } h2,p { margin: 0; } h2 { color:#0f172a; font-size:18px; letter-spacing:-.01em; } header p { margin-top: 4px; color: #64748b; font-size: 13px; } form { display:grid; gap: 16px; margin-top: 22px; } label { display:grid; gap: 7px; color:#334155; font-size:13px; font-weight:500; } input { box-sizing:border-box; width:100%; height:40px; border:1px solid #cbd5e1; border-radius:10px; padding:0 12px; color:#0f172a; background:#fff; font:inherit; transition:border-color .15s ease, box-shadow .15s ease; } input:focus-visible { outline:none; border-color:#6366f1; box-shadow:0 0 0 3px rgba(99,102,241,.14); } footer { justify-content:flex-end; margin-top:4px; }.form-error{color:#dc2626;font-size:13px}

@media (prefers-reduced-motion: reduce) {
  .key-dialog-fade-enter-active,
  .key-dialog-fade-leave-active,
  .key-dialog-fade-enter-active .key-dialog,
  .key-dialog-fade-leave-active .key-dialog { transition-duration: .01ms; }
}
</style>
