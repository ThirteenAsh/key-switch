import type { ProviderValidation } from "../types/domain";

export type ProviderValidationError =
  | "validationUrlRequired"
  | "validationHttpsRequired"
  | "validationUrlUnsafe"
  | "validationHeaderRequired"
  | "validationHeaderInvalid";

const blockedHeaders = new Set([
  "authorization",
  "cookie",
  "content-length",
  "host",
  "proxy-authorization",
  "set-cookie",
]);

export function createEmptyValidation(): ProviderValidation {
  return { mode: "none" };
}

export function validateProviderValidation(
  validation: ProviderValidation,
): ProviderValidationError | null {
  if (validation.mode === "none") return null;

  const rawUrl = validation.mode === "openai-compatible"
    ? validation.baseUrl.trim()
    : validation.endpoint.trim();
  if (!rawUrl) return "validationUrlRequired";

  let parsed: URL;
  try {
    parsed = new URL(rawUrl);
  } catch {
    return "validationUrlRequired";
  }
  if (parsed.protocol !== "https:") return "validationHttpsRequired";
  if (
    parsed.username
    || parsed.password
    || parsed.search
    || parsed.hash
    || !parsed.hostname
    || parsed.hostname === "localhost"
    || parsed.hostname.endsWith(".localhost")
    || parsed.hostname.endsWith(".local")
    || parsed.hostname.endsWith(".internal")
  ) {
    return "validationUrlUnsafe";
  }

  if (validation.mode === "api-key-header") {
    const headerName = validation.headerName.trim();
    if (!headerName) return "validationHeaderRequired";
    if (!/^[!#$%&'*+.^_`|~0-9A-Za-z-]+$/.test(headerName) || blockedHeaders.has(headerName.toLowerCase())) {
      return "validationHeaderInvalid";
    }
  }
  return null;
}

export function normalizeProviderValidation(validation: ProviderValidation): ProviderValidation {
  switch (validation.mode) {
    case "none":
      return { mode: "none" };
    case "openai-compatible":
      return { mode: validation.mode, baseUrl: validation.baseUrl.trim().replace(/\/+$/, "") };
    case "bearer":
      return { mode: validation.mode, endpoint: validation.endpoint.trim() };
    case "api-key-header":
      return {
        mode: validation.mode,
        endpoint: validation.endpoint.trim(),
        headerName: validation.headerName.trim(),
      };
  }
}
