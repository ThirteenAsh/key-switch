<script setup lang="ts">
import { ref, watch } from "vue";
import { ImagePlus, Plus, Trash2, X } from "@lucide/vue";
import { useI18n } from "vue-i18n";
import AppButton from "./ui/AppButton.vue";
import ProviderValidationFields from "./ProviderValidationFields.vue";
import type { ProviderValidation } from "../types/domain";
import { createEmptyValidation, normalizeProviderValidation, validateProviderValidation } from "../utils/providerValidation";

const props = defineProps<{ open: boolean }>();
const { t } = useI18n();

const emit = defineEmits<{
  close: [];
  add: [payload: { name: string; platformUrl: string; logo?: string; validation: ProviderValidation }];
}>();

const name = ref("");
const platformUrl = ref("");
const logo = ref("");
const validation = ref<ProviderValidation>(createEmptyValidation());
const error = ref("");
const fileInput = ref<HTMLInputElement | null>(null);

watch(() => props.open, (isOpen) => {
  if (!isOpen) return;
  name.value = "";
  platformUrl.value = "";
  logo.value = "";
  validation.value = createEmptyValidation();
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
    error.value = t("providerDialog.errors.invalidAvatarType");
    input.value = "";
    return;
  }
  if (file.size > 2 * 1024 * 1024) {
    error.value = t("providerDialog.errors.avatarTooLarge");
    input.value = "";
    return;
  }

  const reader = new FileReader();
  reader.onload = () => {
    logo.value = typeof reader.result === "string" ? reader.result : "";
    error.value = "";
  };
  reader.onerror = () => {
    error.value = t("providerDialog.errors.avatarReadFailed");
  };
  reader.readAsDataURL(file);
}

function removeAvatar() {
  logo.value = "";
  if (fileInput.value) fileInput.value.value = "";
}

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
  if (!normalizedName) {
    error.value = t("providerDialog.errors.nameRequired");
    return;
  }
  if (!normalizedPlatformUrl) {
    error.value = t("providerDialog.errors.platformUrlRequired");
    return;
  }

  try {
    const parsedUrl = new URL(normalizedPlatformUrl);
    if (parsedUrl.protocol !== "https:" && parsedUrl.protocol !== "http:") throw new Error();
  } catch {
    error.value = t("providerDialog.errors.invalidUrl");
    return;
  }

  const validationError = validateProviderValidation(validation.value);
  if (validationError) {
    error.value = t(`providerDialog.errors.${validationError}`);
    return;
  }

  emit("add", { name: normalizedName, platformUrl: normalizedPlatformUrl, logo: logo.value || undefined, validation: normalizeProviderValidation(validation.value) });
}
</script>

<template>
  <Teleport to="body">
    <Transition name="custom-dialog-fade">
      <div v-if="open" class="dialog-backdrop" role="presentation" @click.self="emit('close')">
        <section class="custom-provider-dialog" role="dialog" aria-modal="true" aria-labelledby="custom-provider-title">
          <header>
            <div>
              <h2 id="custom-provider-title">{{ t("providerDialog.customTitle") }}</h2>
              <p>{{ t("providerDialog.customDescription") }}</p>
            </div>
            <AppButton variant="ghost" size="icon-sm" :aria-label="t('common.close')" @click="emit('close')">
              <X :size="15" :stroke-width="2" />
            </AppButton>
          </header>

          <form @submit.prevent="submit">
            <div class="avatar-field">
              <span class="field-label">{{ t("providerDialog.avatar") }}</span>
              <div class="avatar-picker">
                <img v-if="logo" :src="logo" :alt="t('common.selectedProviderAvatarAlt')" />
                <ImagePlus v-else :size="22" :stroke-width="1.8" aria-hidden="true" />
                <input ref="fileInput" type="file" accept="image/png,image/jpeg,image/webp" @change="handleAvatarChange" />
                <div class="avatar-actions">
                  <AppButton variant="secondary" size="sm" type="button" @click="selectAvatar">
                    <ImagePlus :size="14" :stroke-width="2" />
                    <span>{{ t(logo ? "common.replaceImage" : "common.uploadImage") }}</span>
                  </AppButton>
                  <AppButton v-if="logo" variant="ghost" size="icon-sm" type="button" :aria-label="t('common.removeAvatar')" @click="removeAvatar">
                    <Trash2 :size="14" :stroke-width="2" />
                  </AppButton>
                </div>
              </div>
              <small>{{ t("providerDialog.avatarHelpOptional") }}</small>
            </div>

            <label>
              <span class="field-label">{{ t("providerDialog.name") }}</span>
              <input v-model="name" maxlength="64" :placeholder="t('providerDialog.namePlaceholder')" autocomplete="off" autofocus />
            </label>
            <label>
              <span class="field-label">{{ t("providerDialog.platformUrl") }}</span>
              <input v-model="platformUrl" type="text" inputmode="url" placeholder="https://platform.example.com" autocomplete="url" @blur="ensureHttpsPrefix" />
            </label>

            <ProviderValidationFields v-model="validation" />

            <p v-if="error" class="form-error" role="alert">{{ error }}</p>
            <footer>
              <AppButton variant="secondary" type="button" @click="emit('close')">{{ t("common.cancel") }}</AppButton>
              <AppButton variant="primary" type="submit">
                <Plus :size="15" :stroke-width="2.2" />
                <span>{{ t("providerDialog.addProvider") }}</span>
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
  max-height: calc(100vh - 32px);
  overflow-y: auto;
  padding: 24px;
  box-sizing: border-box;
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
