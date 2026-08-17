import { createRouter, createWebHashHistory } from "vue-router";
import DashboardView from "../views/DashboardView.vue";
import ProvidersView from "../views/ProvidersView.vue";
import SettingsView from "../views/SettingsView.vue";

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: "/", name: "dashboard", component: DashboardView },
    { path: "/providers", name: "providers", component: ProvidersView },
    { path: "/settings", name: "settings", component: SettingsView },
  ],
});
