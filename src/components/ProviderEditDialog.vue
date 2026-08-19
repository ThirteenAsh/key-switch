<script setup lang="ts">
import { ref, watch } from "vue";
import { Save, X } from "@lucide/vue";
import AppButton from "./ui/AppButton.vue";
import type { ProviderSummary } from "../types/domain";

const props = defineProps<{
  open: boolean;
  provider: ProviderSummary | null;
}>();

const emit = defineEmits<{
  close: [];
  save: [payload: { id: string; name: string; platformUrl: string }];
}>();

const name = ref("");
const platformUrl = ref("");
const error = ref("");

watch(() => [props.open, props.provider] as const, ([isOpen, provider]) => {
  if (!isOpen || !provider) return;
  name.value = provider.name;
  platformUrl.value = provider.platformUrl ?? "";
  error.value = "";
}, { immediate: true });

function ensureHttpsPrefix() {
  const value = platformUrl.value.trim();
  if (value && !/^https?:\/\//i.test(value)) {
    platformUrl.value = `https://${value}`;
  }
  return platformUrl.value.trim();
}

function submit() {
  const normalizedName = name.value.trim();
  const normalizedPlatformUrl = ensureHttpsPrefix();

  if (!props.provider || !normalizedName) {
    error.value = "请输入供应商名称";
    return;
  }

  if (!normalizedPlatformUrl) {
    error.value = "请输入平台管理地址";
    return;
  }

  try {
    const parsedUrl = new URL(normalizedPlatformUrl);
    if (parsedUrl.protocol !== "https:" && parsedUrl.protocol !== "http:") throw new Error();
  } catch {
    error.value = "请输入有效的 http 或 https 地址";
    return;
  }

  emit("save", { id: props.provider.id, name: normalizedName, platformUrl: normalizedPlatformUrl });
}
</script>

<template>
  <Teleport to="body">
    <Transition name="edit-dialog-fade">
      <div v-if="open && provider" class="dialog-backdrop" role="presentation" @click.self="emit('close')">
        <section class="provider-edit-dialog" role="dialog" aria-modal="true" aria-labelledby="provider-edit-title">
          <header class="provider-edit-header">
            <div>
              <h2 id="provider-edit-title">供应商配置</h2>
              <p>修改供应商名称和平台管理地址。</p>
            </div>
            <AppButton variant="ghost" size="icon-sm" aria-label="关闭" @click="emit('close')">
              <X :size="15" :stroke-width="2" />
            </AppButton>
          </header>

          <form class="provider-edit-form" @submit.prevent="submit">
            <label>
              <span>供应商名称</span>
              <input v-model="name" maxlength="64" autocomplete="off" />
            </label>
            <label>
              <span>平台管理地址</span>
              <input v-model="platformUrl" type="text" inputmode="url" placeholder="https://platform.example.com" autocomplete="url" @blur="ensureHttpsPrefix" />
            </label>
            <p v-if="error" class="provider-edit-error" role="alert">{{ error }}</p>
            <footer>
              <AppButton variant="secondary" type="button" @click="emit('close')">取消</AppButton>
              <AppButton variant="primary" type="submit">
                <Save :size="15" :stroke-width="2" />
                <span>保存</span>
              </AppButton>
            </footer>
          </form>
        </section>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.edit-dialog-fade-enter-active,
.edit-dialog-fade-leave-active {
  transition: opacity 0.15s ease-out;
}

.edit-dialog-fade-enter-from,
.edit-dialog-fade-leave-to {
  opacity: 0;
}

.provider-edit-dialog {
  width: 480px;
  max-width: calc(100vw - 32px);
  padding: 24px;
  border-radius: 16px;
  background: #fff;
  box-shadow: 0 25px 60px -15px rgba(15, 23, 42, 0.25);
}

.provider-edit-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
}

.provider-edit-header h2 {
  margin: 0;
  color: #0f172a;
  font-size: 18px;
}

.provider-edit-header p {
  margin: 4px 0 0;
  color: #64748b;
  font-size: 12.5px;
}

.provider-edit-form {
  display: grid;
  gap: 18px;
  margin-top: 22px;
}

.provider-edit-form label {
  display: grid;
  gap: 7px;
  color: #1e293b;
  font-size: 13px;
  font-weight: 600;
}

.provider-edit-form input {
  height: 42px;
  padding: 0 13px;
  color: #0f172a;
  font: inherit;
  font-weight: 400;
  border: 1px solid #e2e8f0;
  border-radius: 9px;
  outline: 0;
  background: #f8fafc;
  transition: border-color 0.15s ease, box-shadow 0.15s ease, background 0.15s ease;
}

.provider-edit-form input:focus {
  border-color: #38bdf8;
  background: #fff;
  box-shadow: 0 0 0 3px rgba(56, 189, 248, 0.15);
}

.provider-edit-error {
  margin: -6px 0 0;
  color: #dc2626;
  font-size: 12px;
}

.provider-edit-form footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  margin-top: 2px;
}
</style>
