<template>
  <div class="app-shell">
    <div class="app-body">
      <AppSidebar />
      <main class="page-content">
        <RouterView v-slot="{ Component, route }">
          <Transition name="page-fade" mode="out-in">
            <component :is="Component" :key="route.path" />
          </Transition>
        </RouterView>
      </main>
    </div>
    <AppFooter />
  </div>
</template>

<script setup lang="ts">
import { onMounted } from "vue";
import AppFooter from "./components/AppFooter.vue";
import AppSidebar from "./components/AppSidebar.vue";
import { useDashboardStore } from "./stores/dashboard";

const store = useDashboardStore();
onMounted(() => { void store.load(); });
</script>

<style scoped>
/* 页面级轻快丝滑过渡动画（0.15s） */
.page-fade-enter-active,
.page-fade-leave-active {
  transition: opacity 0.15s ease-out,
              transform 0.15s cubic-bezier(0.16, 1, 0.3, 1);
}

.page-fade-enter-from {
  opacity: 0;
  transform: translateY(4px);
}

.page-fade-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}
</style>
