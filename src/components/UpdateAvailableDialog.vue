<script setup lang="ts">
import { Download, ExternalLink, Sparkles, X } from "@lucide/vue";
import type { UpdateInfo } from "../api/app";
import AppButton from "./ui/AppButton.vue";

defineProps<{ open: boolean; update: UpdateInfo | null }>();
const emit = defineEmits<{ close: []; download: [] }>();
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
                <h2 id="update-dialog-title">发现新版本 v{{ update.latestVersion }}</h2>
                <span v-if="update.prerelease" class="update-dialog__tag">Alpha</span>
              </div>
              <p>当前版本 v{{ update.currentVersion }}</p>
            </div>
            <AppButton variant="ghost" size="icon-sm" aria-label="关闭" @click="emit('close')"><X :size="16" /></AppButton>
          </header>

          <div class="update-dialog__body">
            <strong>{{ update.title }}</strong>
            <p v-if="update.notes" class="update-dialog__notes">{{ update.notes }}</p>
            <p class="update-dialog__hint">Alpha 版本暂不支持应用内自动安装，请前往 GitHub Release 下载最新安装包。</p>
          </div>

          <footer>
            <AppButton variant="secondary" @click="emit('close')">稍后</AppButton>
            <AppButton variant="primary" @click="emit('download')">
              <Download :size="15" :stroke-width="2" />
              前往下载
              <ExternalLink :size="13" :stroke-width="2" />
            </AppButton>
          </footer>
        </section>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.update-dialog-fade-enter-active,.update-dialog-fade-leave-active{transition:opacity .18s ease}.update-dialog-fade-enter-active .update-dialog,.update-dialog-fade-leave-active .update-dialog{transition:transform .22s cubic-bezier(.22,1,.36,1),opacity .18s ease}.update-dialog-fade-enter-from,.update-dialog-fade-leave-to{opacity:0}.update-dialog-fade-enter-from .update-dialog{opacity:0;transform:translateY(10px) scale(.97)}.update-dialog-fade-leave-to .update-dialog{opacity:0;transform:translateY(6px) scale(.98)}
.update-dialog{width:480px;max-width:calc(100vw - 32px);padding:22px;border:1px solid rgba(226,232,240,.9);border-radius:18px;background:#fff;box-shadow:0 28px 70px -18px rgba(15,23,42,.3)}
header{display:flex;align-items:flex-start;gap:12px}.update-dialog__icon{display:grid;place-items:center;flex:0 0 40px;width:40px;height:40px;color:#4f46e5;border-radius:12px;background:#eef2ff}.update-dialog__heading{min-width:0;flex:1}.update-dialog__title-row{display:flex;align-items:center;gap:8px}h2,p{margin:0}h2{color:#0f172a;font-size:17px}.update-dialog__heading>p{margin-top:4px;color:#64748b;font-size:12.5px}.update-dialog__tag{padding:2px 6px;color:#b45309;border-radius:5px;background:#fffbeb;font-size:10px;font-weight:700;text-transform:uppercase}
.update-dialog__body{display:grid;gap:10px;margin-top:20px;padding:14px;border:1px solid #e2e8f0;border-radius:12px;background:#f8fafc}.update-dialog__body strong{color:#334155;font-size:13px}.update-dialog__notes{max-height:112px;overflow:auto;color:#64748b;font-size:12.5px;line-height:1.65;white-space:pre-wrap}.update-dialog__hint{color:#475569;font-size:12px;line-height:1.6}footer{display:flex;justify-content:flex-end;gap:10px;margin-top:20px}
</style>
