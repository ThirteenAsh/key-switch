import { createRouter, createWebHashHistory } from "vue-router";
import DashboardView from "../views/DashboardView.vue";
import ProvidersView from "../views/ProvidersView.vue";
import SettingsView from "../views/SettingsView.vue";
import { settingsNavigation } from "../data/settingsNavigation";

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: "/", name: "dashboard", component: DashboardView },
    { path: "/providers", name: "providers", component: ProvidersView },
    {
      path: "/settings",
      redirect: "/settings/general",
      children: settingsNavigation.map((item) => ({
        path: item.id,
        name: `settings-${item.id}`,
        component: SettingsView,
        meta: { settingsCategory: item.id },
      })),
    },
  ],
});
