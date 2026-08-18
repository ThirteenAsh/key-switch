<script setup lang="ts">
import { ref, watch } from "vue";
import { ImagePlus, Plus, Trash2, X } from "@lucide/vue";
import AppButton from "./ui/AppButton.vue";

const props = defineProps<{ open: boolean }>();

const emit = defineEmits<{
  close: [];
  add: [payload: { name: string; baseUrl: string; logo?: string }];
}>();

const name = ref("");
const baseUrl = ref("");
const logo = ref("");
const error = ref("");
const fileInput = ref<HTMLInputElement | null>(null);

watch(() => props.open, (isOpen) => {
  if (!isOpen) return;
  name.value = "";
  baseUrl.value = "";
  logo.value = "";
  error.value = "";
  if (fileInput.value) fileInput.value.value = "";
});

function selectAvatar() {
  fileInput.value?.click();
}

function handleAvatarChange(event: Event) {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0];
  if (!file) return;

  const allowedTypes = ["image/png", "image/jpeg", "image/webp"];
  if (!allowedTypes.includes(file.type)) {
    error.value = "头像仅支持 PNG、JPG 或 WebP 图片";
    input.value = "";
    return;
  }
  if (file.size > 2 * 1024 * 1024) {
    error.value = "头像图片不能超过 2MB";
    input.value = "";
    return;
  }

  const reader = new FileReader();
  reader.onload = () => {
    logo.value = typeof reader.result === "string" ? reader.result : "";
    error.value = "";
  };
  reader.onerror = () => {
    error.value = "读取头像失败，请重新选择图片";
  };
  reader.readAsDataURL(file);
}

function removeAvatar() {
  logo.value = "";
  if (fileInput.value) fileInput.value.value = "";
}

function submit() {
  const normalizedName = name.value.trim();
  const normalizedBaseUrl = baseUrl.value.trim();
  if (!normalizedName) {
    error.value = "请输入供应商名称";
    return;
  }
  if (!normalizedBaseUrl) {
    error.value = "请输入 API 接口基础地址";
    return;
  }

  try {
    const parsedUrl = new URL(normalizedBaseUrl);
    if (parsedUrl.protocol !== "https:" && parsedUrl.protocol !== "http:") throw new Error();
  } catch {
    error.value = "请输入有效的 http 或 https 地址";
    return;
  }

  emit("add", { name: normalizedName, baseUrl: normalizedBaseUrl, logo: logo.value || undefined });
}
</script>

<template>
  <Teleport to="body">
    <Transition name="custom-dialog-fade">
      <div v-if="open" class="dialog-backdrop" role="presentation" @click.self="emit('close')">
        <section class="custom-provider-dialog" role="dialog" aria-modal="true" aria-labelledby="custom-provider-title">
          <header>
            <div>
              <h2 id="custom-provider-title">新增自定义供应商</h2>
              <p>配置供应商名称、接口地址和可选头像。</p>
            </div>
            <AppButton variant="ghost" size="icon-sm" aria-label="关闭" @click="emit('close')">
              <X :size="15" :stroke-width="2" />
            </AppButton>
          </header>

          <form @submit.prevent="submit">
            <div class="avatar-field">
              <span class="field-label">供应商头像</span>
              <div class="avatar-picker">
                <img v-if="logo" :src="logo" alt="已选择的供应商头像" />
                <ImagePlus v-else :size="22" :stroke-width="1.8" aria-hidden="true" />
                <input ref="fileInput" type="file" accept="image/png,image/jpeg,image/webp" @change="handleAvatarChange" />
                <div class="avatar-actions">
                  <AppButton variant="secondary" size="sm" type="button" @click="selectAvatar">
                    <ImagePlus :size="14" :stroke-width="2" />
                    <span>{{ logo ? "更换图片" : "上传图片" }}</span>
                  </AppButton>
                  <AppButton v-if="logo" variant="ghost" size="icon-sm" type="button" aria-label="移除头像" @click="removeAvatar">
                    <Trash2 :size="14" :stroke-width="2" />
                  </AppButton>
                </div>
              </div>
              <small>可选，支持 PNG、JPG、WebP，最大 2MB。</small>
            </div>

            <label>
              <span class="field-label">供应商名称</span>
              <input v-model="name" maxlength="64" placeholder="例如：公司内部模型网关 / OneAPI" autocomplete="off" autofocus />
            </label>
            <label>
              <span class="field-label">API 接口基础地址 (Base URL)</span>
              <input v-model="baseUrl" type="url" placeholder="https://api.example.com/v1" autocomplete="url" />
            </label>

            <p v-if="error" class="form-error" role="alert">{{ error }}</p>
            <footer>
              <AppButton variant="secondary" type="button" @click="emit('close')">取消</AppButton>
              <AppButton variant="primary" type="submit">
                <Plus :size="15" :stroke-width="2.2" />
                <span>新增供应商</span>
              </AppButton>
            </footer>
          </form>
        </section>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.custom-dialog-fade-enter-active,
.custom-dialog-fade-leave-active { transition: opacity 0.15s ease-out; }
.custom-dialog-fade-enter-from,
.custom-dialog-fade-leave-to { opacity: 0; }

.custom-provider-dialog {
  width: 500px;
  max-width: calc(100vw - 32px);
  padding: 24px;
  border-radius: 16px;
  background: #fff;
  box-shadow: 0 25px 60px -15px rgba(15, 23, 42, 0.25);
}

.custom-provider-dialog > header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
}

.custom-provider-dialog h2 { margin: 0; color: #0f172a; font-size: 18px; }
.custom-provider-dialog header p { margin: 4px 0 0; color: #64748b; font-size: 12.5px; }

.custom-provider-dialog form { display: grid; gap: 17px; margin-top: 22px; }
.custom-provider-dialog label { display: grid; gap: 7px; }
.field-label { color: #1e293b; font-size: 13px; font-weight: 600; }

.custom-provider-dialog input:not([type="file"]) {
  height: 42px;
  padding: 0 13px;
  color: #0f172a;
  font: inherit;
  border: 1px solid #e2e8f0;
  border-radius: 9px;
  outline: 0;
  background: #f8fafc;
  transition: border-color 0.15s ease, box-shadow 0.15s ease, background 0.15s ease;
}

.custom-provider-dialog input:not([type="file"]):focus {
  border-color: #38bdf8;
  background: #fff;
  box-shadow: 0 0 0 3px rgba(56, 189, 248, 0.15);
}

.avatar-field { display: grid; gap: 8px; }
.avatar-picker { display: flex; align-items: center; gap: 12px; }
.avatar-picker > img,
.avatar-picker > svg {
  display: grid;
  width: 52px;
  height: 52px;
  flex: 0 0 52px;
  place-items: center;
  color: #94a3b8;
  border: 1px solid #e2e8f0;
  border-radius: 12px;
  background: #f8fafc;
  object-fit: cover;
}
.avatar-picker input { display: none; }
.avatar-actions { display: flex; align-items: center; gap: 4px; }
.avatar-field small { color: #94a3b8; font-size: 11.5px; }
.form-error { margin: -6px 0 0; color: #dc2626; font-size: 12px; }
.custom-provider-dialog footer { display: flex; justify-content: flex-end; gap: 10px; margin-top: 2px; }
</style>
