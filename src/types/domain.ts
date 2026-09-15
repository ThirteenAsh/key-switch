export type KeyStatus = "valid" | "untested" | "invalid" | "unsupported" | "error" | "checking";

export type KeyCheckErrorCode =
  | "timeout"
  | "network"
  | "rateLimited"
  | "serverUnavailable"
  | "endpointInvalid"
  | "unexpectedStatus";

export type ProviderValidation =
  | { mode: "none" }
  | { mode: "openai-compatible"; baseUrl: string }
  | { mode: "bearer"; endpoint: string }
  | { mode: "api-key-header"; endpoint: string; headerName: string };

export interface ApiKeySummary {
  id: string;
  providerId: string;
  remark: string;
  maskedValue: string;
  status: KeyStatus;
  lastCheckedAt?: string;
  checkErrorCode?: KeyCheckErrorCode;
}

export interface ProviderSummary {
  id: string;
  name: string;
  abbreviation: string;
  tone: "blue" | "violet" | "orange" | "indigo" | "gray";
  logo?: string;
  kind: "builtin" | "custom";
  platformUrl?: string;
  validation: ProviderValidation;
  validationSupported: boolean;
  keys: ApiKeySummary[];
}

export interface BuiltinProviderOption {
  id: string;
  nameZh: string;
  nameEn: string;
  abbreviation: string;
  tone: ProviderSummary["tone"];
  logo: string;
  platformUrl: string;
}

export interface DashboardSummary {
  providerCount: number;
  keyCount: number;
  availableKeyCount: number;
}
