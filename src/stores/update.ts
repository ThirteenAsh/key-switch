import { computed, ref } from "vue";
import { defineStore } from "pinia";
import { installAppUpdate } from "../api/app";

export type UpdateInstallStatus = "idle" | "downloading" | "timeout" | "failed";

export const useUpdateStore = defineStore("update", () => {
  const installStatus = ref<UpdateInstallStatus>("idle");
  const installing = computed(() => installStatus.value === "downloading");
  let statusTimer: number | undefined;

  function setTemporaryStatus(status: Exclude<UpdateInstallStatus, "idle" | "downloading">) {
    installStatus.value = status;
    window.clearTimeout(statusTimer);
    statusTimer = window.setTimeout(() => { installStatus.value = "idle"; }, 5000);
  }

  async function install(releaseTag: string) {
    if (installing.value) return;
    window.clearTimeout(statusTimer);
    installStatus.value = "downloading";
    try {
      await installAppUpdate(releaseTag);
    } catch (error) {
      const message = String(error);
      setTemporaryStatus(message.includes("更新下载超时") ? "timeout" : "failed");
      throw error;
    }
  }

  return { installStatus, installing, install };
});
