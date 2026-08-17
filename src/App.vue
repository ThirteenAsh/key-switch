<template>
  <div class="app-shell">
    <div class="app-body">
      <AppSidebar />
      <main class="page-content">
        <RouterView />
      </main>
    </div>
    <AppFooter />
  </div>
</template>

<script setup lang="ts">
import AppFooter from "./components/AppFooter.vue";
import AppSidebar from "./components/AppSidebar.vue";
</script>
