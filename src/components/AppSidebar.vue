<script setup lang="ts">
import { computed } from "vue";
import { useRoute } from "vue-router";
import { ArrowLeft, Cpu, Settings } from "@lucide/vue";
import { useI18n } from "vue-i18n";
import appIcon from "../assets/key-switch.svg";
import AppFooter from "./AppFooter.vue";
import ProviderNavigation from "./ProviderNavigation.vue";
import { settingsNavigation } from "../data/settingsNavigation";
import { builtinProviderCatalog } from "../data/providerCatalog";

const { t } = useI18n();
const route = useRoute();
const inSettings = computed(() => route.path.startsWith("/settings/"));
</script>

<template>
  <aside class="app-sidebar">
    <RouterLink to="/" class="sidebar-brand" :aria-label="t('navigation.dashboard')">
      <img :src="appIcon" alt="" /><span>{{ t('common.appName') }}</span>
    </RouterLink>
    <!-- 不占高度的宽度基准：内置中英文名称 + 图标、间距、手柄和内边距。 -->
    <div class="sidebar-width-guide" aria-hidden="true">
      <template v-for="provider in builtinProviderCatalog" :key="provider.id">
        <span>{{ provider.nameZh }}</span><span>{{ provider.nameEn }}</span>
      </template>
    </div>
    <template v-if="inSettings">
      <RouterLink to="/" class="nav-item sidebar-back"><ArrowLeft :size="16" aria-hidden="true" /><span>{{ t('common.back') }}</span></RouterLink>
      <p class="sidebar-section-label">{{ t('settings.title') }}</p>
      <nav class="sidebar-nav settings-navigation" :aria-label="t('settings.title')">
        <RouterLink v-for="item in settingsNavigation" :key="item.id" :to="item.to" class="nav-item" active-class="is-active">
          <component :is="item.icon" :size="16" aria-hidden="true" /><span>{{ t(item.labelKey) }}</span>
        </RouterLink>
      </nav>
    </template>
    <ProviderNavigation v-else />
    <div class="sidebar-bottom">
      <AppFooter />
      <nav v-if="!inSettings" class="sidebar-bottom-nav" :aria-label="t('navigation.main')">
        <RouterLink to="/providers" class="nav-item" active-class="is-active"><Cpu :size="16" aria-hidden="true" /><span>{{ t('navigation.providers') }}</span></RouterLink>
        <RouterLink to="/settings" class="nav-item" active-class="is-active"><Settings :size="16" aria-hidden="true" /><span>{{ t('navigation.settings') }}</span></RouterLink>
      </nav>
    </div>
  </aside>
</template>

<style scoped>
.app-sidebar { display: flex; flex-direction: column; min-height: 0; padding: 0 12px 14px; background: var(--surface-sidebar); border-right: 1px solid var(--border); }
.sidebar-brand { display: flex; align-items: center; gap: 8px; padding: 0 8px; height: var(--app-heading-height); margin-bottom: 12px; color: var(--text-primary); font-size: 14px; line-height: 1.4; font-weight: 600; text-decoration: none; flex-shrink: 0; }
.sidebar-brand img { width: 24px; height: 24px; }
.sidebar-width-guide { height: 0; width: max-content; padding-inline: 34px; overflow: hidden; visibility: hidden; font-size: 12px; font-weight: 600; white-space: nowrap; }
.sidebar-width-guide span { display: block; }
.sidebar-nav { display: grid; gap: 5px; }
.nav-item { display: flex; align-items: center; gap: 8px; min-height: 34px; min-width: 0; padding: 6px 9px; color: var(--text-muted); font-size: 12px; font-weight: 500; text-decoration: none; border-radius: 9px; transition: color .15s ease, background .15s ease; }
.nav-item svg { flex-shrink: 0; }
.nav-item span { min-width: 0; overflow-wrap: anywhere; }
.nav-item:hover { color: var(--text-primary); background: var(--interactive-hover); }
.nav-item.is-active { color: var(--nav-active-text); background: var(--nav-active); font-weight: 600; }
.sidebar-back { margin-bottom: 20px; }
.sidebar-section-label { padding: 0 10px; margin: 0 0 9px; color: var(--text-subtle); font-size: 12px; }
.settings-navigation { overflow-y: auto; contain: inline-size; }
.sidebar-bottom { contain: inline-size; flex-shrink: 0; margin-top: auto; padding-top: 16px; }
.sidebar-bottom-nav { display: flex; flex-wrap: wrap; gap: 4px; padding-top: 10px; margin-top: 12px; border-top: 1px solid var(--border); }
.sidebar-bottom-nav .nav-item { flex: 1 0 auto; max-width: 100%; gap: 6px; padding: 6px 8px; font-size: 12px; }
.sidebar-bottom :deep(.app-footer) { align-items: flex-start; flex-direction: column; gap: 8px; padding: 0 10px; line-height: 1.6; }
</style>
