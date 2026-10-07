import { writeClientLog, type ClientLogLevel } from "../api/app";

const MAX_DETAIL_LENGTH = 2400;
let writeQueue: Promise<void> = Promise.resolve();
let handlersInstalled = false;
let writeFailureReported = false;

export function sanitizeDetail(value: string): string {
  return value
    .replace(/\b(?:proxy-authorization|authorization|api[-_ ]?key|access[-_ ]?token|refresh[-_ ]?token|secret|password)\b["']?\s*[:=]\s*(?:"[^"]*"|'[^']*'|(?:Bearer|Basic)\s+[^\s,;]+|[^\s,;]+)/gi, "credential=[REDACTED]")
    .replace(/\b(?:Bearer|Basic)\s+[A-Za-z0-9._~+/=\-]+/gi, "credential=[REDACTED]")
    .replace(/(?:https?|socks5h?):\/\/[^\s"'<>]+/gi, "[REDACTED_URL]")
    .replace(/\bsk-[A-Za-z0-9_\-]{8,}/g, "[REDACTED]")
    .replace(/[\r\n]+/g, "\\n")
    .slice(0, MAX_DETAIL_LENGTH);
}

export function describeError(error: unknown): string {
  if (error instanceof Error) {
    const stack = error.stack && error.stack !== `${error.name}: ${error.message}`
      ? ` stack=${error.stack}`
      : "";
    return sanitizeDetail(`${error.name}: ${error.message}${stack}`);
  }
  if (typeof error === "string") return sanitizeDetail(error);
  if (typeof error === "object" && error !== null && "code" in error) {
    const code = (error as { code?: unknown }).code;
    if (typeof code === "string") return `code=${sanitizeDetail(code)}`;
  }
  return sanitizeDetail(Object.prototype.toString.call(error));
}

export function logClientEvent(level: ClientLogLevel, event: string, detail?: string): void {
  writeQueue = writeQueue
    .catch(() => undefined)
    .then(async () => {
      await writeClientLog({ level, event, detail: detail ? sanitizeDetail(detail) : undefined });
      writeFailureReported = false;
    })
    .catch(() => {
      if (!writeFailureReported) console.warn("[key-switch] log_write_failed");
      writeFailureReported = true;
    });
}

export function installGlobalErrorHandlers(): void {
  if (handlersInstalled) return;
  handlersInstalled = true;
  window.addEventListener("error", (event) => {
    const target = event.target instanceof Element ? event.target : null;
    const detail = target
      ? `resource=${target.tagName} source=${target.getAttribute("src") ?? target.getAttribute("href") ?? "unknown"}`
      : event.error
      ? describeError(event.error)
      : `message=${event.message || "resource_error"} source=${event.filename || "unknown"} line=${event.lineno} column=${event.colno}`;
    logClientEvent("ERROR", "window_error", detail);
  }, true);

  window.addEventListener("unhandledrejection", (event) => {
    logClientEvent("ERROR", "unhandled_rejection", describeError(event.reason));
  });
}
