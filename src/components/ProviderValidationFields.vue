<script setup lang="ts">
import { computed } from "vue";
import { ShieldCheck } from "@lucide/vue";
import { useI18n } from "vue-i18n";
import type { ProviderValidation } from "../types/domain";
import AppSelect from "./ui/AppSelect.vue";
import type { AppSelectOption } from "./ui/AppSelect.vue";

const props = defineProps<{ modelValue: ProviderValidation }>();
const emit = defineEmits<{ "update:modelValue": [value: ProviderValidation] }>();
const { t } = useI18n();

const mode = computed(() => props.modelValue.mode);
const modeOptions = computed<AppSelectOption[]>(() => [
  { value: "none", label: t("providerDialog.validation.none") },
  { value: "openai-compatible", label: t("providerDialog.validation.openaiCompatible") },
  { value: "bearer", label: t("providerDialog.validation.bearer") },
  { value: "api-key-header", label: t("providerDialog.validation.apiKeyHeader") },
]);
const endpointHost = computed(() => {
  if (props.modelValue.mode === "none") return "";
  const value = props.modelValue.mode === "openai-compatible"
    ? props.modelValue.baseUrl
    : props.modelValue.endpoint;
  try { return new URL(value).hostname; }
  catch { return ""; }
});

function changeMode(value: string) {
  const nextMode = value as ProviderValidation["mode"];
  if (nextMode === "openai-compatible") emit("update:modelValue", { mode: nextMode, baseUrl: "" });
  else if (nextMode === "bearer") emit("update:modelValue", { mode: nextMode, endpoint: "" });
  else if (nextMode === "api-key-header") emit("update:modelValue", { mode: nextMode, endpoint: "", headerName: "x-api-key" });
  else emit("update:modelValue", { mode: "none" });
}

function updateUrl(value: string) {
  if (props.modelValue.mode === "openai-compatible") {
    emit("update:modelValue", { ...props.modelValue, baseUrl: value });
  } else if (props.modelValue.mode !== "none") {
    emit("update:modelValue", { ...props.modelValue, endpoint: value });
  }
}

function updateHeaderName(value: string) {
  if (props.modelValue.mode === "api-key-header") {
    emit("update:modelValue", { ...props.modelValue, headerName: value });
  }
}
</script>

<template>
  <fieldset class="validation-fields">
    <legend>
      <ShieldCheck :size="14" :stroke-width="2" aria-hidden="true" />
      {{ t("providerDialog.validation.title") }}
    </legend>
    <div class="validation-mode-field">
      <span>{{ t("providerDialog.validation.mode") }}</span>
      <AppSelect
        :model-value="mode"
        :options="modeOptions"
        :label="t('providerDialog.validation.mode')"
        full-width
        placement="top"
        @update:model-value="changeMode"
      />
    </div>

    <label v-if="modelValue.mode !== 'none'">
      <span>{{ t(modelValue.mode === "openai-compatible" ? "providerDialog.validation.baseUrl" : "providerDialog.validation.endpoint") }}</span>
      <input
        :value="modelValue.mode === 'openai-compatible' ? modelValue.baseUrl : modelValue.endpoint"
        type="url"
        inputmode="url"
        :placeholder="modelValue.mode === 'openai-compatible' ? 'https://api.example.com/v1' : 'https://api.example.com/v1/models'"
        autocomplete="off"
        @input="updateUrl(($event.target as HTMLInputElement).value)"
      />
    </label>

    <label v-if="modelValue.mode === 'api-key-header'">
      <span>{{ t("providerDialog.validation.headerName") }}</span>
      <input
        :value="modelValue.headerName"
        maxlength="64"
        placeholder="x-api-key"
        autocomplete="off"
        @input="updateHeaderName(($event.target as HTMLInputElement).value)"
      />
    </label>

    <p v-if="modelValue.mode === 'none'" class="validation-hint">
      {{ t("providerDialog.validation.noneHelp") }}
    </p>
    <p v-else class="validation-warning">
      {{ endpointHost
        ? t("providerDialog.validation.hostWarning", { host: endpointHost })
        : t("providerDialog.validation.securityWarning") }}
    </p>
  </fieldset>
</template>

<style scoped>
.validation-fields { display: grid; gap: 10px; margin: 2px 0 0; padding: 12px; border: 1px solid #e2e8f0; border-radius: 10px; }
.validation-fields legend { display: flex; align-items: center; gap: 6px; padding: 0 5px; color: #1e293b; font-size: 12.5px; font-weight: 600; }
.validation-fields label { display: grid; gap: 6px; color: #334155; font-size: 12px; font-weight: 600; }
.validation-mode-field { display: grid; gap: 6px; color: #334155; font-size: 12px; font-weight: 600; }
.validation-fields input { width: 100%; height: 38px; padding: 0 11px; color: #0f172a; font: inherit; font-weight: 400; border: 1px solid #e2e8f0; border-radius: 8px; outline: 0; background: #f8fafc; box-sizing: border-box; }
.validation-fields input:focus { border-color: #38bdf8; background: #fff; box-shadow: 0 0 0 3px rgba(56, 189, 248, .14); }
.validation-hint,
.validation-warning { margin: 0; font-size: 11.5px; line-height: 1.45; }
.validation-hint { color: #64748b; }
.validation-warning { color: #b45309; }
</style>
