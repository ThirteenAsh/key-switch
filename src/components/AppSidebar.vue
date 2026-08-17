<script setup lang="ts">
import { Database, LayoutDashboard, Settings, Boxes } from "@lucide/vue";

const navigation = [
  { label: "仪表盘", to: "/", icon: LayoutDashboard },
  { label: "供应商", to: "/providers", icon: Boxes },
  { label: "设置", to: "/settings", icon: Settings },
];
</script>

<template>
  <aside class="app-sidebar">
    <nav aria-label="主导航">
      <RouterLink v-for="item in navigation" :key="item.to" :to="item.to" class="nav-item">
        <component :is="item.icon" :size="18" stroke-width="2" />
        <span>{{ item.label }}</span>
      </RouterLink>
    </nav>

    <section class="storage-card" aria-label="本地存储状态">
      <Database :size="17" />
      <div>
        <strong>本地存储</strong>
        <p>数据已安全保存在本地</p>
      </div>
      <span class="storage-ok" aria-label="存储正常">✓</span>
    </section>
  </aside>
</template>
