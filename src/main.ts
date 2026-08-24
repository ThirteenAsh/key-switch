import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import { router } from "./router";
import { i18n, initializeLocale } from "./i18n";
import { useSettingsStore } from "./stores/settings";
import "./styles/global.css";

async function bootstrap(): Promise<void> {
  initializeLocale();
  const pinia = createPinia();
  try {
    await useSettingsStore(pinia).load();
  } catch {
    // 设置读取失败时保留已解析的系统语言，应用仍可正常启动。
  }

  createApp(App).use(pinia).use(router).use(i18n).mount("#app");
}

void bootstrap();
