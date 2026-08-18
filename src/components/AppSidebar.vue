<script setup lang="ts">
import {
  LayoutGrid,
  Cpu,
  SlidersHorizontal
} from "@lucide/vue";

const navigation = [
  { label: "仪表盘", to: "/", icon: LayoutGrid },
  { label: "供应商", to: "/providers", icon: Cpu },
  { label: "设置", to: "/settings", icon: SlidersHorizontal },
];
</script>

<template>
  <aside class="app-sidebar">
    <div class="sidebar-top">
      <nav class="sidebar-nav" aria-label="主导航">
        <RouterLink
          v-for="item in navigation"
          :key="item.to"
          :to="item.to"
          class="nav-item"
        >
          <component :is="item.icon" :size="17" :stroke-width="1.9" class="nav-icon" />
          <span>{{ item.label }}</span>
        </RouterLink>
      </nav>
    </div>
  </aside>
</template>

<style scoped>
.app-sidebar {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  padding: 24px 14px 18px;
  background: transparent;
  border: none;
  box-shadow: none;
}

.sidebar-top {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.sidebar-nav {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

/* 导航项：轻快丝滑悬浮与切换动画（0.15s） */
.nav-item {
  position: relative;
  display: flex;
  align-items: center;
  gap: 12px;
  height: 40px;
  padding: 0 14px;
  color: #64748b;
  font-size: 13.5px;
  font-weight: 500;
  text-decoration: none;
  border: none;
  border-radius: 9px;
  transition: color 0.15s cubic-bezier(0.16, 1, 0.3, 1),
              background 0.15s cubic-bezier(0.16, 1, 0.3, 1),
              transform 0.12s ease;
}

.nav-icon {
  flex-shrink: 0;
  transition: transform 0.15s cubic-bezier(0.16, 1, 0.3, 1),
              color 0.15s ease;
}

.nav-item:hover {
  color: #0f172a;
  background: rgba(15, 23, 42, 0.04);
}

.nav-item:hover .nav-icon {
  transform: scale(1.06);
}

.nav-item:active {
  transform: scale(0.98);
}

.nav-item.router-link-active {
  color: #0f172a;
  background: rgba(15, 23, 42, 0.07);
  font-weight: 600;
}

.nav-item.router-link-active .nav-icon {
  color: #0f172a;
}

</style>
