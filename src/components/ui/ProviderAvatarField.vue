<script setup lang="ts">
import { onBeforeUnmount, ref } from "vue";
import { ImagePlus, Trash2 } from "@lucide/vue";
import { useI18n } from "vue-i18n";
import AppButton from "./AppButton.vue";

const logo = defineModel<string>({ required: true });
const reading = defineModel<boolean>("reading", { required: true });
const { t } = useI18n();
const fileInput = ref<HTMLInputElement | null>(null);
const error = ref<"invalidAvatarType" | "avatarTooLarge" | "avatarReadFailed" | "">("");
const imageTypes = ["image/png", "image/jpeg", "image/webp"];
let reader: FileReader | undefined;

onBeforeUnmount(() => {
  reader?.abort();
  reading.value = false;
});

function handleImageChange(event: Event) {
  const input = event.currentTarget as HTMLInputElement;
  const file = input.files?.[0];
  input.value = "";
  if (!file || reading.value) return;

  if (!imageTypes.includes(file.type)) {
    error.value = "invalidAvatarType";
    return;
  }
  if (file.size > 2 * 1024 * 1024) {
    error.value = "avatarTooLarge";
    return;
  }

  error.value = "";
  reading.value = true;
  const fileReader = new FileReader();
  reader = fileReader;
  fileReader.onload = () => {
    if (typeof fileReader.result === "string") {
      logo.value = fileReader.result;
    } else {
      error.value = "avatarReadFailed";
    }
  };
  fileReader.onerror = () => { error.value = "avatarReadFailed"; };
  fileReader.onloadend = () => {
    reader = undefined;
    reading.value = false;
  };
  fileReader.readAsDataURL(file);
}

function removeImage() {
  logo.value = "";
  error.value = "";
}
</script>

<template>
  <div class="avatar-field" role="group" :aria-label="t('providerDialog.avatar')">
    <span class="field-label">{{ t("providerDialog.avatar") }}</span>
    <div class="avatar-picker">
      <img v-if="logo" :src="logo" :alt="t('common.selectedProviderAvatarAlt')" />
      <div v-else class="avatar-placeholder" aria-hidden="true">
        <ImagePlus :size="22" :stroke-width="1.8" />
      </div>
      <input ref="fileInput" type="file" hidden :accept="imageTypes.join(',')" @change="handleImageChange" />
      <div class="avatar-actions">
        <AppButton variant="secondary" size="sm" :disabled="reading" @click="fileInput?.click()">
          <ImagePlus :size="14" :stroke-width="2" aria-hidden="true" />
          <span>{{ t(logo ? "common.replaceImage" : "common.uploadImage") }}</span>
        </AppButton>
        <AppButton v-if="logo" variant="ghost" size="icon-sm" :disabled="reading" :aria-label="t('common.removeAvatar')" @click="removeImage">
          <Trash2 :size="14" :stroke-width="2" aria-hidden="true" />
        </AppButton>
      </div>
    </div>
    <small>{{ t("providerDialog.avatarHelpOptional") }}</small>
    <p v-if="error" class="avatar-error" role="alert">{{ t(`providerDialog.errors.${error}`) }}</p>
  </div>
</template>

<style scoped>
.avatar-field { display: grid; gap: 8px; }
.field-label { color: var(--text-strong); font-size: 13px; font-weight: 600; }
.avatar-picker { display: flex; align-items: center; flex-wrap: wrap; gap: 12px; }
.avatar-picker > img,
.avatar-placeholder {
  display: grid;
  width: 52px;
  height: 52px;
  flex: 0 0 52px;
  place-items: center;
  color: var(--text-subtle);
  border: 1px solid var(--border);
  border-radius: 12px;
  background: var(--surface-app);
  object-fit: cover;
}
.avatar-actions { display: flex; align-items: center; gap: 4px; }
.avatar-field small { color: var(--text-subtle); font-size: 11.5px; }
.avatar-error { margin: 0; color: var(--danger-text); font-size: 12px; }
</style>
