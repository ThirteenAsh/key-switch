import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { test } from "node:test";
import { runInNewContext } from "node:vm";
import ts from "typescript";
import * as vue from "vue";
import * as pinia from "pinia";
import * as vueRouter from "vue-router";
import * as icons from "@lucide/vue";

// 使用现有 TypeScript 编译器与 Node 测试运行器，无需安装测试依赖。
async function loadModule(path, dependencies) {
  const source = await readFile(new URL(path, import.meta.url), "utf8");
  const { outputText } = ts.transpileModule(source, {
    compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2020 },
  });
  const exports = {};
  runInNewContext(outputText, {
    exports,
    crypto: globalThis.crypto,
    require(specifier) {
      assert.ok(specifier in dependencies, `Missing test dependency: ${specifier}`);
      return dependencies[specifier];
    },
  }, { filename: path });
  return exports;
}

function provider(id, name = id) {
  return {
    id, name, abbreviation: id, tone: "gray", kind: "custom",
    validation: { mode: "none" }, validationSupported: false, keys: [],
  };
}

function deferred() {
  let resolve;
  let reject;
  const promise = new Promise((res, rej) => { resolve = res; reject = rej; });
  return { promise, resolve, reject };
}

async function dashboard(api = {}) {
  const module = await loadModule("../src/stores/dashboard.ts", {
    vue, pinia,
    "../api/app": api,
    "../data/providerCatalog": { builtinProviderCatalog: [] },
  });
  return module.useDashboardStore(pinia.createPinia());
}

function ids(store) {
  return Array.from(store.providers, (item) => item.id);
}

test("首次加载、切换与删除均提供有效的供应商选择", async () => {
  const store = await dashboard({
    listProviders: async () => [provider("a", "甲供应商"), provider("b", "乙供应商")],
    deleteProvider: async () => {},
  });
  await store.load();
  assert.equal(store.loading, false);
  assert.equal(store.selectedProvider.id, "a");
  store.selectedProviderId = "b";
  assert.equal(store.selectedProvider.id, "b");
  await store.removeProvider("b");
  assert.equal(store.selectedProvider.id, "a");
  await store.removeProvider("a");
  assert.equal(store.selectedProvider, undefined);
});

test("加载失败可重试，重载保留仍存在的选中供应商", async () => {
  const error = { code: "DATA_READ_FAILED" };
  let fail = true;
  const store = await dashboard({ listProviders: async () => {
    if (fail) throw error;
    return [provider("a"), provider("b")];
  } });
  await assert.rejects(store.load(), (caught) => caught === error);
  assert.equal(store.loading, false);
  assert.equal(store.loadError.code, error.code);
  fail = false;
  await store.load();
  assert.equal(store.loadError, null);
  store.selectedProviderId = "b";
  await store.load();
  assert.equal(store.selectedProvider.id, "b");
});

test("排序保存完整列表、阻止重复提交并保留当前供应商", async () => {
  const saving = deferred();
  const calls = [];
  const store = await dashboard({ reorderProviders: (order) => {
    calls.push(Array.from(order));
    return saving.promise;
  } });
  store.providers = [provider("a"), provider("b"), provider("c")];
  const operation = store.reorderProviders(0, 2);
  assert.deepEqual(ids(store), ["b", "c", "a"]);
  assert.equal(store.selectedProvider.id, "a");
  assert.equal(store.sorting, true);
  await store.reorderProviders(1, 0);
  assert.equal(calls.length, 1);
  assert.deepEqual(calls[0], ["b", "c", "a"]);
  saving.resolve();
  await operation;
  assert.equal(store.sorting, false);
});

test("排序失败恢复原顺序并保留期间发生的元数据修改", async () => {
  const saving = deferred();
  const error = { code: "DATA_SAVE_FAILED" };
  const store = await dashboard({ reorderProviders: () => saving.promise });
  store.providers = [provider("a"), provider("b"), provider("c")];
  store.selectedProviderId = "b";
  const operation = store.reorderProviders(0, 2);
  store.providers.find((item) => item.id === "b").name = "更新后的名称";
  store.providers.push(provider("d"));
  saving.reject(error);
  await assert.rejects(operation, (caught) => caught === error);
  assert.deepEqual(ids(store), ["a", "b", "c", "d"]);
  assert.equal(store.selectedProvider.name, "更新后的名称");
  assert.equal(store.sorting, false);
});

test("向上排序保存完整列表并保留选中的供应商", async () => {
  let saved;
  const store = await dashboard({ reorderProviders: async (order) => { saved = Array.from(order); } });
  store.providers = [provider("a"), provider("b"), provider("c")];
  store.selectedProviderId = "c";
  await store.reorderProviders(2, 0);
  assert.deepEqual(saved, ["c", "a", "b"]);
  assert.equal(store.selectedProvider.id, "c");
});

test("设置入口重定向到通用，各分类均支持独立路由和返回主页", async () => {
  const navigation = await loadModule("../src/data/settingsNavigation.ts", { "@lucide/vue": icons });
  const { router } = await loadModule("../src/router/index.ts", {
    "vue-router": { ...vueRouter, createWebHashHistory: vueRouter.createMemoryHistory },
    "../data/settingsNavigation": navigation,
    "../views/DashboardView.vue": { default: {} },
    "../views/ProvidersView.vue": { default: {} },
    "../views/SettingsView.vue": { default: {} },
  });
  await router.push("/settings");
  assert.equal(router.currentRoute.value.path, "/settings/general");
  for (const category of navigation.settingsNavigation) {
    await router.push(category.to);
    assert.equal(router.currentRoute.value.meta.settingsCategory, category.id);
  }
  await router.push("/");
  assert.equal(router.currentRoute.value.name, "dashboard");
});
