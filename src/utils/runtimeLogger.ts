import { writeClientLog, type ClientLogLevel } from "../api/app";

const MAX_DETAIL_LENGTH = 2400;
let writeQueue: Promise<void> = Promise.resolve();

function sanitizeDetail(value: string): string {
  return value
    .replace(/(api[-_ ]?key|access[-_ ]?token|refresh[-_ ]?token|secret|password|authorization|bearer)(\s*[:=]\s*)([^\s,;]+)/gi, "$1$2[REDACTED]")
    .replace(/([?&](?:api[-_ ]?key|access[-_ ]?token|refresh[-_ ]?token|secret|password|authorization)=)[^&#\s]+/gi, "$1[REDACTED]")
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
    .then(() => writeClientLog({ level, event, detail: detail ? sanitizeDetail(detail) : undefined }))
    .catch(() => undefined);
}

export function installGlobalErrorHandlers(): void {
  window.addEventListener("error", (event) => {
    const detail = event.error
      ? describeError(event.error)
      : `message=${event.message || "resource_error"} source=${event.filename || "unknown"} line=${event.lineno} column=${event.colno}`;
    logClientEvent("ERROR", "window_error", detail);
  });

  window.addEventListener("unhandledrejection", (event) => {
    logClientEvent("ERROR", "unhandled_rejection", describeError(event.reason));
  });
}
