import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import { router } from "./router";
import { i18n, initializeLocale } from "./i18n";
import { useSettingsStore } from "./stores/settings";
import { describeError, installGlobalErrorHandlers, logClientEvent } from "./utils/runtimeLogger";
import "./styles/global.css";

installGlobalErrorHandlers();

async function bootstrap(): Promise<void> {
  initializeLocale();
  const pinia = createPinia();
  try {
    await useSettingsStore(pinia).load();
  } catch {
    // 设置读取失败时保留已解析的系统语言，应用仍可正常启动。
  }

  const app = createApp(App);
  app.config.errorHandler = (error, _instance, info) => {
    logClientEvent("ERROR", "vue_error", `info=${info} ${describeError(error)}`);
  };
  app.config.warnHandler = (message, _instance, trace) => {
    logClientEvent("WARN", "vue_warning", `message=${message} trace=${trace}`);
  };
  app.use(pinia).use(router).use(i18n).mount("#app");
  logClientEvent("INFO", "frontend_started");
}

void bootstrap();
