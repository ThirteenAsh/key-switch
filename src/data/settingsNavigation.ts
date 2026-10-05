import { Database, Info, SlidersHorizontal } from "@lucide/vue";

export const settingsNavigation = [
  { id: "general", to: "/settings/general", labelKey: "settings.categories.general", icon: SlidersHorizontal },
  { id: "data", to: "/settings/data", labelKey: "settings.categories.data", icon: Database },
  { id: "about", to: "/settings/about", labelKey: "settings.categories.about", icon: Info },
] as const;
