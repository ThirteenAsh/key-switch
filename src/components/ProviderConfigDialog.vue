<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { Check, ImagePlus, Plus, Trash2, X } from "@lucide/vue";
import AppButton from "./ui/AppButton.vue";
import { builtinProviderCatalog } from "../data/providerCatalog";

const props = defineProps<{
  open: boolean;
  configuredProviderIds: string[];
}>();

const emit = defineEmits<{
  close: [];
  addBuiltin: [providerId: string];
  addCustom: [name: string, platformUrl: string, logo?: string];
}>();

const mode = ref<"builtin" | "custom">("builtin");
const selectedProviderId = ref("");
const customName = ref("");
const customPlatformUrl = ref("");
const customLogo = ref("");
const avatarInput = ref<HTMLInputElement | null>(null);
const error = ref("");

const availableProviders = computed(() => builtinProviderCatalog.map((provider) => ({
  ...provider,
  configured: props.configuredProviderIds.includes(provider.id),
})));

watch(() => props.open, (isOpen) => {
  if (!isOpen) return;
  error.value = "";
  selectedProviderId.value = "";
  customName.value = "";
  customPlatformUrl.value = "";
  customLogo.value = "";
  if (avatarInput.value) avatarInput.value.value = "";
  mode.value = "builtin";
});

function submitBuiltin() {
  if (!selectedProviderId.value) {
    error.value = "请选择一个内置供应商";
    return;
  }
  emit("addBuiltin", selectedProviderId.value);
}

function submitCustom() {
  const name = customName.value.trim();
  if (!name) {
    error.value = "请输入供应商名称";
    return;
  }
  emit("addCustom", name, ensureHttpsPrefix(), customLogo.value || undefined);
}

function ensureHttpsPrefix() {
  const value = customPlatformUrl.value.trim();
  if (value && !/^https?:\/\//i.test(value)) {
    customPlatformUrl.value = `https://${value}`;
  }
  return customPlatformUrl.value.trim();
}

function selectAvatar() {
  avatarInput.value?.click();
}

function handleAvatarChange(event: Event) {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0];
  if (!file) return;

  if (!["image/png", "image/jpeg", "image/webp"].includes(file.type)) {
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
    customLogo.value = typeof reader.result === "string" ? reader.result : "";
    error.value = "";
  };
  reader.onerror = () => {
    error.value = "读取头像失败，请重新选择图片";
  };
  reader.readAsDataURL(file);
}

function removeAvatar() {
  customLogo.value = "";
  if (avatarInput.value) avatarInput.value.value = "";
}
</script>

<template>
  <Teleport to="body">
    <Transition name="dialog-fade">
      <div v-if="open" class="dialog-backdrop" role="presentation" @click.self="emit('close')">
        <section class="provider-config-dialog" role="dialog" aria-modal="true" aria-labelledby="provider-config-title">
          <header class="dialog-header">
            <div>
              <h2 id="provider-config-title">新增配置</h2>
              <p class="dialog-subtitle">选择内置供应商快速接入，或配置自定义供应商。</p>
            </div>
            <AppButton variant="ghost" size="icon-sm" aria-label="关闭" @click="emit('close')">
              <X :size="15" :stroke-width="2" />
            </AppButton>
          </header>

          <!-- 现代分段选择器（轻快滑动） -->
          <div class="dialog-segmented-tabs" role="tablist" aria-label="配置类型">
            <div
              class="tab-indicator"
              :style="{
                transform: mode === 'builtin' ? 'translateX(0%)' : 'translateX(100%)'
              }"
            />
            <button
              :class="{ active: mode === 'builtin' }"
              type="button"
              role="tab"
              :aria-selected="mode === 'builtin'"
              @click="mode = 'builtin'; error = ''"
            >
              内置供应商
            </button>
            <button
              :class="{ active: mode === 'custom' }"
              type="button"
              role="tab"
              :aria-selected="mode === 'custom'"
              @click="mode = 'custom'; error = ''"
            >
              自定义供应商
            </button>
          </div>

          <!-- 窗口内容区：绝对尺寸锁定，零晃动零跳跃 -->
          <div class="dialog-body">
            <Transition name="tab-crossfade" mode="out-in">
              <div v-if="mode === 'builtin'" key="builtin" class="tab-pane provider-scroll-wrap">
                <div class="provider-option-grid">
                  <button
                    v-for="provider in availableProviders"
                    :key="provider.id"
                    class="provider-option"
                    :class="{ selected: selectedProviderId === provider.id, configured: provider.configured }"
                    :disabled="provider.configured"
                    type="button"
                    @click="selectedProviderId = provider.id; error = ''"
                  >
                    <img :src="provider.logo" :alt="`${provider.name} 图标`" />
                    <span class="provider-title">{{ provider.name }}</span>
                    <Check v-if="provider.configured" :size="13" :stroke-width="2.5" class="configured-check" aria-label="已配置" />
                  </button>
                </div>
              </div>

              <div v-else key="custom" class="tab-pane custom-provider-form">
                <div class="custom-avatar-field">
                  <div class="form-label-row">
                    <span>供应商头像</span>
                    <span class="form-optional">可选</span>
                  </div>
                  <div class="custom-avatar-picker">
                    <img v-if="customLogo" :src="customLogo" alt="已选择的供应商头像" />
                    <ImagePlus v-else :size="20" :stroke-width="1.8" aria-hidden="true" />
                    <input ref="avatarInput" class="custom-avatar-file" type="file" accept="image/png,image/jpeg,image/webp" @change="handleAvatarChange" />
                    <AppButton variant="secondary" size="sm" type="button" @click="selectAvatar">
                      <ImagePlus :size="13" :stroke-width="2" />
                      <span>{{ customLogo ? "更换图片" : "上传图片" }}</span>
                    </AppButton>
                    <AppButton v-if="customLogo" variant="ghost" size="icon-sm" type="button" aria-label="移除头像" @click="removeAvatar">
                      <Trash2 :size="14" :stroke-width="2" />
                    </AppButton>
                    <small>PNG、JPG、WebP，最大 2MB</small>
                  </div>
                </div>
                <div class="form-group">
                  <div class="form-label-row">
                    <label for="custom-name">供应商名称</label>
                    <span class="form-required">必填</span>
                  </div>
                  <input
                    id="custom-name"
                    v-model="customName"
                    maxlength="64"
                    placeholder="例如：公司内部大模型网关 / OneAPI"
                    autofocus
                  />
                </div>

                <div class="form-group">
                  <div class="form-label-row">
                    <label for="custom-url">平台管理地址</label>
                    <span class="form-optional">可选</span>
                  </div>
                  <input
                    id="custom-url"
                    v-model="customPlatformUrl"
                    type="text"
                    inputmode="url"
                    placeholder="https://platform.example.com"
                    @blur="ensureHttpsPrefix"
                  />
                </div>
              </div>
            </Transition>
          </div>

          <!-- 预留错误提示位，防止出现报错时撑开对话框 -->
          <div class="dialog-error-slot">
            <p v-if="error" class="dialog-error" role="alert">{{ error }}</p>
          </div>

          <footer class="dialog-footer">
            <AppButton variant="secondary" @click="emit('close')">取消</AppButton>
            <AppButton variant="primary" @click="mode === 'builtin' ? submitBuiltin() : submitCustom()">
              <Plus :size="15" :stroke-width="2.2" />
              <span>新增配置</span>
            </AppButton>
          </footer>
        </section>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
/* 弹窗整体淡入淡出动画 */
.dialog-fade-enter-active,
.dialog-fade-leave-active {
  transition: opacity 0.15s ease-out;
}

.dialog-fade-enter-from,
.dialog-fade-leave-to {
  opacity: 0;
}

.dialog-fade-enter-active .provider-config-dialog,
.dialog-fade-leave-active .provider-config-dialog {
  transition: transform 0.16s cubic-bezier(0.16, 1, 0.3, 1);
}

.dialog-fade-enter-from .provider-config-dialog,
.dialog-fade-leave-to .provider-config-dialog {
  transform: translateY(8px) scale(0.985);
}

/* 弹窗主体尺寸锁定：宽高严格固定，彻底消除任何抖动与尺寸微变 */
.provider-config-dialog {
  width: 580px;
  max-width: calc(100vw - 32px);
  padding: 24px 26px;
  border-radius: 16px;
  background: #ffffff;
  box-shadow: 0 25px 60px -15px rgba(15, 23, 42, 0.25);
  box-sizing: border-box;
}

.dialog-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
}

.dialog-header h2 {
  margin: 0;
  font-size: 18px;
  font-weight: 700;
  letter-spacing: -0.02em;
  color: #0f172a;
}

.dialog-subtitle {
  margin: 4px 0 0;
  color: #64748b;
  font-size: 12.5px;
  line-height: 1.4;
}

/* 分段指示器轻快滑动切换 */
.dialog-segmented-tabs {
  position: relative;
  display: grid;
  grid-template-columns: 1fr 1fr;
  margin-top: 18px;
  padding: 3px;
  background: #f1f5f9;
  border-radius: 9px;
  user-select: none;
  box-sizing: border-box;
}

.tab-indicator {
  position: absolute;
  top: 3px;
  left: 3px;
  width: calc(50% - 3px);
  height: calc(100% - 6px);
  background: #ffffff;
  border-radius: 7px;
  box-shadow: 0 1px 3px rgba(15, 23, 42, 0.08), 0 1px 2px rgba(15, 23, 42, 0.04);
  transition: transform 0.18s cubic-bezier(0.16, 1, 0.3, 1);
  pointer-events: none;
  z-index: 1;
}

.dialog-segmented-tabs button {
  position: relative;
  z-index: 2;
  height: 32px;
  color: #64748b;
  font-size: 13px;
  font-weight: 500;
  border-radius: 7px;
  cursor: pointer;
  border: none;
  background: transparent;
  transition: color 0.15s ease, font-weight 0.15s ease;
}

.dialog-segmented-tabs button.active {
  color: #0f172a;
  font-weight: 600;
}

/* 核心内容区容器：严格锁定宽高，防止过渡期间产生高度塌陷抖动 */
.dialog-body {
  width: 100%;
  height: 280px;
  min-height: 280px;
  max-height: 280px;
  margin: 16px 0 10px;
  position: relative;
  overflow: hidden;
  box-sizing: border-box;
}

.tab-pane {
  width: 100%;
  height: 100%;
  box-sizing: border-box;
}

/* 快速丝滑的内容切换过渡 */
.tab-crossfade-enter-active,
.tab-crossfade-leave-active {
  transition: opacity 0.14s ease-out,
              transform 0.14s cubic-bezier(0.16, 1, 0.3, 1);
  width: 100%;
  height: 100%;
}

.tab-crossfade-enter-from {
  opacity: 0;
  transform: translateY(4px);
}

.tab-crossfade-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}

/* 内置供应商滚动容器与极简精致滑条 */
.provider-scroll-wrap {
  height: 100%;
  overflow-y: scroll;
  padding-right: 4px;
  scrollbar-width: thin;
  scrollbar-color: rgba(15, 23, 42, 0.15) transparent;
}

.provider-scroll-wrap::-webkit-scrollbar {
  width: 5px;
}

.provider-scroll-wrap::-webkit-scrollbar-track {
  background: transparent;
}

.provider-scroll-wrap::-webkit-scrollbar-thumb {
  background: rgba(15, 23, 42, 0.12);
  border-radius: 999px;
  transition: background 0.15s ease;
}

.provider-scroll-wrap::-webkit-scrollbar-thumb:hover {
  background: rgba(15, 23, 42, 0.25);
}

/* 内置供应商网格排版 */
.provider-option-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 8px;
  width: 100%;
  padding-bottom: 2px;
}

.provider-option {
  position: relative;
  display: flex;
  align-items: center;
  gap: 9px;
  height: 48px;
  padding: 0 12px;
  color: #334155;
  font-size: 13px;
  text-align: left;
  border-radius: 9px;
  background: #f8fafc;
  cursor: pointer;
  transition: background 0.14s ease, transform 0.12s ease, box-shadow 0.14s ease;
}

.provider-option img {
  width: 22px;
  height: 22px;
  object-fit: contain;
  flex-shrink: 0;
}

.provider-title {
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.provider-option:hover:not(:disabled) {
  color: #0f172a;
  background: #f1f5f9;
  transform: translateY(-1px);
}

.provider-option.selected {
  color: #0f172a;
  background: #f1f5f9;
  box-shadow: inset 0 0 0 2px #0f172a;
}

.provider-option.configured {
  color: #94a3b8;
  cursor: default;
  background: #f8fafc;
  opacity: 0.65;
}

.configured-check {
  position: absolute;
  top: 6px;
  right: 6px;
  color: #10b981;
}

/* 自定义供应商表单排版：完美上下垂直居中 */
.custom-provider-form {
  display: flex;
  flex-direction: column;
  justify-content: flex-start;
  gap: 14px;
  height: 100%;
  padding: 0 4px;
}

.custom-avatar-field {
  display: grid;
  gap: 7px;
}

.custom-avatar-field > .form-label-row > span:first-child {
  color: #1e293b;
  font-size: 13px;
  font-weight: 600;
}

.custom-avatar-picker {
  display: flex;
  align-items: center;
  gap: 8px;
  min-height: 42px;
}

.custom-avatar-picker > img,
.custom-avatar-picker > svg {
  display: grid;
  width: 42px;
  height: 42px;
  flex: 0 0 42px;
  place-items: center;
  color: #94a3b8;
  border: 1px solid #e2e8f0;
  border-radius: 9px;
  background: #f8fafc;
  object-fit: cover;
}

.custom-avatar-file {
  display: none;
}

.custom-avatar-picker small {
  margin-left: auto;
  color: #94a3b8;
  font-size: 11px;
  white-space: nowrap;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 7px;
}

.form-label-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.form-label-row label {
  color: #1e293b;
  font-size: 13px;
  font-weight: 600;
}

.form-required {
  color: #64748b;
  font-size: 11.5px;
  font-weight: 400;
}

.form-optional {
  color: #94a3b8;
  font-size: 11.5px;
  font-weight: 400;
}

.custom-provider-form input {
  height: 42px;
  padding: 0 13px;
  color: #0f172a;
  font-size: 13px;
  border: 0;
  border-radius: 9px;
  outline: 0;
  background: #f1f5f9;
  transition: background 0.15s ease, box-shadow 0.15s ease;
}

.custom-provider-form input::placeholder {
  color: #94a3b8;
  font-size: 12.5px;
}

.custom-provider-form input:focus {
  background: #ffffff;
  box-shadow: 0 0 0 2px rgba(15, 23, 42, 0.2), 0 2px 8px rgba(15, 23, 42, 0.04);
}

/* 错误提示占位槽（固定高度 18px，有无报错都不会挤压导致弹窗尺寸变化） */
.dialog-error-slot {
  height: 18px;
  margin-bottom: 8px;
  display: flex;
  align-items: center;
}

.dialog-error {
  margin: 0;
  color: #ef4444;
  font-size: 12px;
  font-weight: 500;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}
</style>
