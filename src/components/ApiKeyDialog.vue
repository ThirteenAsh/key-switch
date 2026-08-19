<script setup lang="ts">
import { ref, watch } from "vue";
import { KeyRound, X } from "@lucide/vue";
import AppButton from "./ui/AppButton.vue";
const props = defineProps<{ open: boolean; providerName: string }>();
const emit = defineEmits<{ close: []; save: [payload: { remark: string; value: string }] }>();
const remark = ref(""); const value = ref(""); const error = ref("");
watch(() => props.open, (open) => { if (open) { remark.value = ""; value.value = ""; error.value = ""; } });
function submit() { if (!value.value.trim()) { error.value = "请输入 API Key"; return; } emit("save", { remark: remark.value.trim(), value: value.value.trim() }); }
</script>
<template>
  <Teleport to="body"><div v-if="open" class="dialog-backdrop" @click.self="emit('close')"><section class="key-dialog" role="dialog" aria-modal="true">
    <header><div><h2>添加 API Key</h2><p>{{ providerName }}</p></div><AppButton variant="ghost" size="icon-sm" @click="emit('close')"><X :size="15" /></AppButton></header>
    <form @submit.prevent="submit"><label>备注<input v-model="remark" maxlength="64" placeholder="例如：开发环境" /></label><label>API Key<textarea v-model="value" rows="3" autocomplete="off" placeholder="粘贴 API Key" /></label><p v-if="error" class="form-error">{{ error }}</p><footer><AppButton variant="secondary" type="button" @click="emit('close')">取消</AppButton><AppButton variant="primary" type="submit"><KeyRound :size="14" />保存 Key</AppButton></footer></form>
  </section></div></Teleport>
</template>
<style scoped>
.key-dialog { width: 460px; max-width: calc(100vw - 32px); padding: 24px; border-radius: 16px; background: #fff; box-shadow: 0 25px 60px -15px rgba(15,23,42,.25); }
header, footer { display: flex; justify-content: space-between; align-items: center; gap: 12px; } h2,p { margin: 0; } header p { margin-top: 4px; color: #64748b; font-size: 13px; } form { display:grid; gap: 16px; margin-top: 20px; } label { display:grid; gap: 7px; color:#334155; font-size:13px; } input,textarea { box-sizing:border-box; width:100%; border:1px solid #cbd5e1; border-radius:8px; padding:9px 10px; font:inherit; resize:vertical; transition:border-color .15s ease, box-shadow .15s ease; } input:focus-visible,textarea:focus-visible { outline:none; border-color:#3b82f6; box-shadow:0 0 0 3px rgba(59,130,246,.18); } footer { justify-content:flex-end; margin-top:4px; }.form-error{color:#dc2626;font-size:13px}
</style>
