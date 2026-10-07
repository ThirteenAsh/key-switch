import { computed, ref } from "vue";
import { defineStore } from "pinia";
import { checkForAppUpdates, installAppUpdate, type UpdateInfo, type UpdateNetworkMode } from "../api/app";
import { getAppErrorCode } from "../i18n/errors";
import { logClientEvent } from "../utils/runtimeLogger";

export type UpdateInstallStatus = "idle" | "downloading" | "confirming" | "timeout" | "failed";

export const useUpdateStore = defineStore("update", () => {
  const installStatus = ref<UpdateInstallStatus>("idle");
  const checking = ref(false);
  const confirmingDirect = ref(false);
  const installing = computed(() => installStatus.value === "downloading" || installStatus.value === "confirming");
  const busy = computed(() => checking.value || installing.value);
  let statusTimer: ReturnType<typeof window.setTimeout> | undefined;
  let resolveDirect: ((accepted: boolean) => void) | undefined;

  function chooseDirect(accepted: boolean) {
    const resolve = resolveDirect;
    resolveDirect = undefined;
    confirmingDirect.value = false;
    resolve?.(accepted);
  }

  function askForDirect(operationId: string): Promise<boolean> {
    confirmingDirect.value = true;
    return new Promise((resolve) => {
      resolveDirect = (accepted) => {
        logClientEvent("INFO", "update_network_choice", `operation=${operationId} choice=${accepted ? "direct" : "cancel"}`);
        resolve(accepted);
      };
    });
  }

  async function check(interactive = true): Promise<UpdateInfo | null | undefined> {
    if (busy.value) return undefined;
    checking.value = true;
    const operationId = crypto.randomUUID();
    try {
      try {
        return await checkForAppUpdates("auto", operationId);
      } catch (error) {
        if (!interactive || getAppErrorCode(error) !== "UPDATE_PROXY_FAILED") throw error;
        if (!await askForDirect(operationId)) return undefined;
        return await checkForAppUpdates("direct", operationId);
      }
    } finally {
      checking.value = false;
    }
  }

  function setTemporaryStatus(status: "timeout" | "failed") {
    installStatus.value = status;
    window.clearTimeout(statusTimer);
    statusTimer = window.setTimeout(() => { installStatus.value = "idle"; }, 5000);
  }

  async function install(update: UpdateInfo) {
    if (busy.value) return;
    window.clearTimeout(statusTimer);
    installStatus.value = "downloading";
    const operationId = update.operationId || crypto.randomUUID();
    const mode: UpdateNetworkMode = update.networkMode ?? "auto";
    try {
      try {
        await installAppUpdate(update.releaseTag, mode, operationId);
      } catch (error) {
        if (mode === "direct" || getAppErrorCode(error) !== "UPDATE_PROXY_FAILED") throw error;
        installStatus.value = "confirming";
        if (!await askForDirect(operationId)) {
          installStatus.value = "idle";
          return;
        }
        installStatus.value = "downloading";
        await installAppUpdate(update.releaseTag, "direct", operationId);
      }
      installStatus.value = "idle";
    } catch (error) {
      setTemporaryStatus(getAppErrorCode(error) === "UPDATE_DOWNLOAD_TIMEOUT" ? "timeout" : "failed");
      throw error;
    }
  }

  return { installStatus, installing, checking, busy, confirmingDirect, chooseDirect, check, install };
});
