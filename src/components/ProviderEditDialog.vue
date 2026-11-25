<script setup lang="ts">
import { ref, watch } from "vue";
import { Save, X } from "@lucide/vue";
import { useI18n } from "vue-i18n";
import AppButton from "./ui/AppButton.vue";
import ProviderAvatarField from "./ui/ProviderAvatarField.vue";
import ProviderValidationFields from "./ProviderValidationFields.vue";
import type { ProviderSummary, ProviderValidation, UpdateProviderInput } from "../types/domain";
import { normalizeProviderValidation, validateProviderValidation } from "../utils/providerValidation";

const props = defineProps<{
  open: boolean;
  provider: ProviderSummary | null;
  saving: boolean;
  saveError: string;
}>();
const { t } = useI18n();

const emit = defineEmits<{
  close: [];
  save: [payload: UpdateProviderInput];
}>();

const name = ref("");
const platformUrl = ref("");
const logo = ref("");
const readingLogo = ref(false);
const validation = ref<ProviderValidation>({ mode: "none" });
const error = ref("");

watch(() => [props.open, props.provider] as const, ([isOpen, provider]) => {
  if (!isOpen || !provider) return;
  name.value = provider.name;
  platformUrl.value = provider.platformUrl ?? "";
  logo.value = provider.logo ?? "";
  // provider 来自 Pinia 响应式状态，不能直接交给 structuredClone；这些配置只包含
  // 字符串字段，展开为普通对象即可避免编辑弹窗与列表状态互相修改。
  validation.value = { ...provider.validation };
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
  if (props.saving || readingLogo.value || !props.provider) return;
  const provider = props.provider;
  const normalizedName = name.value.trim();
  const normalizedPlatformUrl = ensureHttpsPrefix();

  if (!normalizedName) {
    error.value = t("providerDialog.errors.nameRequired");
    return;
  }

  if (!normalizedPlatformUrl && (provider.kind === "builtin" || provider.platformUrl)) {
    error.value = t("providerDialog.errors.platformUrlRequired");
    return;
  }

  if (normalizedPlatformUrl) {
    try {
      const parsedUrl = new URL(normalizedPlatformUrl);
      if (parsedUrl.protocol !== "https:" && parsedUrl.protocol !== "http:") throw new Error();
    } catch {
      error.value = t("providerDialog.errors.invalidUrl");
      return;
    }
  }

  if (provider.kind === "custom") {
    const validationError = validateProviderValidation(validation.value);
    if (validationError) {
      error.value = t(`providerDialog.errors.${validationError}`);
      return;
    }
  }

  const input: UpdateProviderInput = {
    id: provider.id,
    name: normalizedName,
    platformUrl: normalizedPlatformUrl || undefined,
    validation: provider.kind === "custom" ? normalizeProviderValidation(validation.value) : { mode: "none" },
  };
  if (provider.kind === "custom" && logo.value !== (provider.logo ?? "")) {
    input.logoUpdate = logo.value ? { action: "replace", dataUrl: logo.value } : { action: "remove" };
  }
  error.value = "";
  emit("save", input);
}
</script>

<template>
  <Teleport to="body">
    <Transition name="edit-dialog-fade">
      <div v-if="open && provider" class="dialog-backdrop" role="presentation" @click.self="!saving && emit('close')">
        <section class="provider-edit-dialog" role="dialog" aria-modal="true" aria-labelledby="provider-edit-title">
          <header class="provider-edit-header">
            <div>
              <h2 id="provider-edit-title">{{ t("providerDialog.editTitle") }}</h2>
              <p>{{ t(provider.kind === "custom" ? "providerDialog.editCustomDescription" : "providerDialog.editDescription") }}</p>
            </div>
            <AppButton variant="ghost" size="icon-sm" :disabled="saving" :aria-label="t('common.close')" @click="emit('close')">
              <X :size="15" :stroke-width="2" />
            </AppButton>
          </header>

          <form class="provider-edit-form" :aria-busy="saving" @submit.prevent="submit">
            <fieldset :disabled="saving">
              <ProviderAvatarField v-if="provider.kind === 'custom'" :key="provider.id" v-model="logo" v-model:reading="readingLogo" />
              <label>
                <span>{{ t("providerDialog.name") }}</span>
                <input v-model="name" maxlength="64" autocomplete="off" />
              </label>
              <label>
                <span>{{ t("providerDialog.platformUrl") }}</span>
                <input v-model="platformUrl" type="text" inputmode="url" placeholder="https://platform.example.com" autocomplete="url" @blur="ensureHttpsPrefix" />
              </label>
              <ProviderValidationFields v-if="provider.kind === 'custom'" v-model="validation" />
            </fieldset>
            <p v-if="error || saveError" class="provider-edit-error" role="alert">{{ error || saveError }}</p>
            <footer>
              <AppButton variant="secondary" type="button" :disabled="saving" @click="emit('close')">{{ t("common.cancel") }}</AppButton>
              <AppButton variant="primary" type="submit" :loading="saving" :disabled="readingLogo">
                <Save :size="15" :stroke-width="2" />
                <span>{{ t("common.save") }}</span>
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
  max-height: calc(100vh - 32px);
  overflow-y: auto;
  padding: 24px;
  box-sizing: border-box;
  border-radius: 16px;
  background: var(--surface-raised);
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
  color: var(--text-primary);
  font-size: 18px;
}

.provider-edit-header p {
  margin: 4px 0 0;
  color: var(--text-muted);
  font-size: 12.5px;
}

.provider-edit-form {
  display: grid;
  gap: 18px;
  margin-top: 22px;
}

.provider-edit-form > fieldset {
  display: grid;
  gap: inherit;
  min-inline-size: 0;
  margin: 0;
  padding: 0;
  border: 0;
}

.provider-edit-form label {
  display: grid;
  gap: 7px;
  color: var(--text-strong);
  font-size: 13px;
  font-weight: 600;
}

.provider-edit-form input {
  height: 42px;
  padding: 0 13px;
  color: var(--text-primary);
  font: inherit;
  font-weight: 400;
  border: 1px solid var(--border);
  border-radius: 9px;
  outline: 0;
  background: var(--surface-app);
  transition: border-color 0.15s ease, box-shadow 0.15s ease, background 0.15s ease;
}

.provider-edit-form input:focus {
  border-color: #38bdf8;
  background: var(--surface-raised);
  box-shadow: 0 0 0 3px rgba(56, 189, 248, 0.15);
}

.provider-edit-error {
  margin: -6px 0 0;
  color: var(--danger-text);
  font-size: 12px;
}

.provider-edit-form footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  margin-top: 2px;
}
</style>
