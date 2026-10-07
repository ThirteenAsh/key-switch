import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { test } from "node:test";
import { runInNewContext } from "node:vm";
import ts from "typescript";
import * as vue from "vue";
import * as pinia from "pinia";

async function loadModule(path, dependencies, globals = {}) {
  const source = await readFile(new URL(path, import.meta.url), "utf8");
  const { outputText } = ts.transpileModule(source, { compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2020 } });
  const exports = {};
  runInNewContext(outputText, {
    exports, crypto: globalThis.crypto, window: { setTimeout: () => 1, clearTimeout: () => {} },
    Error, console, ...globals,
    require(specifier) {
      assert.ok(specifier in dependencies, `Missing dependency: ${specifier}`);
      return dependencies[specifier];
    },
  });
  return exports;
}

async function store(api) {
  const events = [];
  const module = await loadModule("../src/stores/update.ts", {
    vue, pinia, "../api/app": api,
    "../i18n/errors": { getAppErrorCode: (error) => error?.code ?? "UNKNOWN" },
    "../utils/runtimeLogger": { logClientEvent: (...args) => events.push(args) },
  });
  return { updateStore: module.useUpdateStore(pinia.createPinia()), events };
}

const update = { releaseTag: "v1.1.2", networkMode: "auto", operationId: "test-operation" };
const proxyError = { code: "UPDATE_PROXY_FAILED" };
async function settle() { for (let i = 0; i < 5; i++) await Promise.resolve(); }

test("代理失败先等待确认，取消不会发起直连，也阻止重复更新", async () => {
  const calls = [];
  const { updateStore, events } = await store({ installAppUpdate: async (...args) => { calls.push(args); throw proxyError; } });
  const pending = updateStore.install(update);
  await settle();
  assert.equal(updateStore.confirmingDirect, true);
  assert.equal(updateStore.busy, true);
  assert.equal(calls.length, 1);
  await updateStore.install(update);
  await updateStore.check();
  assert.equal(calls.length, 1);
  updateStore.chooseDirect(false);
  await pending;
  assert.equal(updateStore.installStatus, "idle");
  assert.equal(updateStore.busy, false);
  assert.match(events[0][2], /choice=cancel/);
});

test("同意后只重试一次直连，保留操作 ID，下一次仍优先代理", async () => {
  const calls = [];
  const { updateStore } = await store({ installAppUpdate: async (...args) => {
    calls.push(args);
    if (args[1] === "auto") throw proxyError;
  } });
  const pending = updateStore.install(update);
  await settle();
  updateStore.chooseDirect(true);
  await pending;
  assert.deepEqual(calls.map((call) => call.slice(1)), [["auto", "test-operation"], ["direct", "test-operation"]]);
  assert.equal(update.networkMode, "auto");
  const next = updateStore.install(update);
  await settle();
  assert.equal(calls[2][1], "auto");
  updateStore.chooseDirect(false);
  await next;
});

test("直连仍超时使用原有超时状态，不再次询问", async () => {
  const directError = { code: "UPDATE_DOWNLOAD_TIMEOUT" };
  const { updateStore } = await store({ installAppUpdate: async (_tag, mode) => { throw mode === "auto" ? proxyError : directError; } });
  const pending = updateStore.install(update);
  await settle();
  updateStore.chooseDirect(true);
  await assert.rejects(pending, (error) => error === directError);
  assert.equal(updateStore.installStatus, "timeout");
  assert.equal(updateStore.confirmingDirect, false);
  assert.equal(updateStore.busy, false);
});

test("签名或更新数据失败直接使用原有失败提示", async () => {
  const error = { code: "UPDATE_DOWNLOAD_FAILED" };
  const { updateStore } = await store({ installAppUpdate: async () => { throw error; } });
  await assert.rejects(updateStore.install(update), (caught) => caught === error);
  assert.equal(updateStore.installStatus, "failed");
  assert.equal(updateStore.confirmingDirect, false);
});

test("手动检测同意直连后，安装沿用直连；下次检测重新优先代理", async () => {
  const calls = [];
  const { updateStore } = await store({
    checkForAppUpdates: async (mode, operationId) => {
      calls.push(["check", mode, operationId]);
      if (mode === "auto") throw proxyError;
      return { ...update, networkMode: mode, operationId };
    },
    installAppUpdate: async (_tag, mode, operationId) => { calls.push(["install", mode, operationId]); },
  });
  const pending = updateStore.check();
  await settle();
  updateStore.chooseDirect(true);
  const result = await pending;
  await updateStore.install(result);
  assert.deepEqual(calls.map((call) => call[1]), ["auto", "direct", "direct"]);
  assert.equal(calls[0][2], calls[2][2]);
  const next = updateStore.check();
  await settle();
  updateStore.chooseDirect(false);
  assert.equal(await next, undefined);
  assert.equal(calls[3][1], "auto");
});

test("启动检测失败保持安静，取消和没有新版本可区分", async () => {
  const { updateStore } = await store({ checkForAppUpdates: async () => { throw proxyError; } });
  await assert.rejects(updateStore.check(false), (error) => error === proxyError);
  assert.equal(updateStore.confirmingDirect, false);
  assert.equal(updateStore.checking, false);
  const quiet = await store({ checkForAppUpdates: async () => null });
  assert.equal(await quiet.updateStore.check(), null);
});

test("直连检测失败只返回原错误，不重复询问", async () => {
  const directError = { code: "UPDATE_CHECK_FAILED" };
  const { updateStore } = await store({ checkForAppUpdates: async (mode) => { throw mode === "auto" ? proxyError : directError; } });
  const pending = updateStore.check();
  await settle();
  updateStore.chooseDirect(true);
  await assert.rejects(pending, (error) => error === directError);
  assert.equal(updateStore.checking, false);
  assert.equal(updateStore.confirmingDirect, false);
});

test("前端日志遮盖 Bearer、JSON 密码和带凭据的 URL", async () => {
  const logger = await loadModule("../src/utils/runtimeLogger.ts", { "../api/app": { writeClientLog: async () => {} } });
  const secret = "example-sensitive-value";
  for (const input of [`Authorization: Bearer ${secret}`, `Bearer ${secret}`, `"password": "${secret}"`, `https://user:${secret}@example.com/${secret}?token=${secret}`]) {
    assert.ok(!logger.sanitizeDetail(input).includes(secret));
  }
  assert.equal(logger.sanitizeDetail("abc\ndef"), "abc\\ndef");
});

test("资源错误使用捕获监听，日志写入失败不泄漏详情且不会阻塞后续记录", async () => {
  const listeners = [];
  const warnings = [];
  const entries = [];
  let fail = true;
  const logger = await loadModule("../src/utils/runtimeLogger.ts", { "../api/app": { writeClientLog: async (entry) => {
    if (fail) throw new Error("sensitive-error-detail");
    entries.push(entry);
  } } }, {
    window: { addEventListener: (...args) => listeners.push(args) },
    console: { warn: (...args) => warnings.push(args) },
  });
  logger.installGlobalErrorHandlers();
  logger.installGlobalErrorHandlers();
  assert.equal(listeners.length, 2);
  assert.equal(listeners[0][2], true);
  logger.logClientEvent("ERROR", "first", "test");
  await new Promise((resolve) => setImmediate(resolve));
  assert.equal(warnings.length, 1);
  assert.ok(!String(warnings).includes("sensitive-error-detail"));
  fail = false;
  logger.logClientEvent("INFO", "next", "test");
  await new Promise((resolve) => setImmediate(resolve));
  assert.equal(entries[0].event, "next");
});
