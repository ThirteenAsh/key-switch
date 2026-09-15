<script setup lang="ts">
import { computed } from "vue";
import { Download, ExternalLink, Sparkles, X } from "@lucide/vue";
import { useI18n } from "vue-i18n";
import type { UpdateInfo } from "../api/app";
import AppButton from "./ui/AppButton.vue";

const props = defineProps<{ open: boolean; update: UpdateInfo | null; installing?: boolean }>();
const emit = defineEmits<{ close: []; install: []; release: [] }>();
const { t } = useI18n();
const channelLabel = computed(() => {
  if (!props.update?.prerelease) return t("update.stable");
  const identifier = props.update.latestVersion.split("-", 2)[1]?.split(".", 1)[0] ?? "RC";
  return identifier.toUpperCase();
});
</script>

<template>
  <Teleport to="body">
    <Transition name="update-dialog-fade">
      <div v-if="open && update" class="dialog-backdrop" @click.self="emit('close')">
        <section class="update-dialog" role="dialog" aria-modal="true" aria-labelledby="update-dialog-title">
          <header>
            <span class="update-dialog__icon"><Sparkles :size="20" :stroke-width="2" /></span>
            <div class="update-dialog__heading">
              <div class="update-dialog__title-row">
                <h2 id="update-dialog-title">{{ t("update.available", { version: update.latestVersion }) }}</h2>
                <span class="update-dialog__tag">{{ channelLabel }}</span>
              </div>
              <p>{{ t("update.current", { version: update.currentVersion }) }}</p>
            </div>
            <AppButton variant="ghost" size="icon-sm" :disabled="installing" :aria-label="t('common.close')" @click="emit('close')"><X :size="16" /></AppButton>
          </header>

          <footer>
            <AppButton variant="ghost" :disabled="installing" @click="emit('close')">{{ t("update.later") }}</AppButton>
            <AppButton variant="secondary" :disabled="installing" @click="emit('release')">
              <ExternalLink :size="14" :stroke-width="2" />
              GitHub Release
            </AppButton>
            <AppButton variant="primary" :loading="installing" @click="emit('install')">
              <Download :size="15" :stroke-width="2" />
              {{ t("update.downloadAndInstall") }}
            </AppButton>
          </footer>
        </section>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.update-dialog-fade-enter-active,.update-dialog-fade-leave-active{transition:opacity .18s ease}.update-dialog-fade-enter-active .update-dialog,.update-dialog-fade-leave-active .update-dialog{transition:transform .22s cubic-bezier(.22,1,.36,1),opacity .18s ease}.update-dialog-fade-enter-from,.update-dialog-fade-leave-to{opacity:0}.update-dialog-fade-enter-from .update-dialog{opacity:0;transform:translateY(10px) scale(.97)}.update-dialog-fade-leave-to .update-dialog{opacity:0;transform:translateY(6px) scale(.98)}
.update-dialog{width:480px;max-width:calc(100vw - 32px);padding:22px;border:1px solid var(--border);border-radius:18px;background:var(--surface-raised);box-shadow:0 28px 70px -18px rgba(15,23,42,.3)}
header{display:flex;align-items:flex-start;gap:12px}.update-dialog__icon{display:flex;flex:0 0 auto;padding-top:2px;color:var(--accent-text)}.update-dialog__heading{min-width:0;flex:1}.update-dialog__title-row{display:flex;align-items:center;gap:8px}h2,p{margin:0}h2{color:var(--text-primary);font-size:17px}.update-dialog__heading>p{margin-top:4px;color:var(--text-muted);font-size:12.5px}.update-dialog__tag{padding:2px 6px;color:var(--warning-text);border-radius:5px;background:var(--warning-surface);font-size:10px;font-weight:700;text-transform:uppercase}
footer{display:flex;justify-content:flex-end;gap:10px;margin-top:24px}
</style>
