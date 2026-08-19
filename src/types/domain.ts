export type KeyStatus = "valid" | "untested" | "invalid" | "error" | "checking";

export interface ApiKeySummary {
  id: string;
  providerId: string;
  remark: string;
  maskedValue: string;
  status: KeyStatus;
  lastCheckedAt?: string;
}

export interface ProviderSummary {
  id: string;
  name: string;
  abbreviation: string;
  tone: "blue" | "violet" | "orange" | "indigo" | "gray";
  logo?: string;
  kind: "builtin" | "custom";
  platformUrl?: string;
  keys: ApiKeySummary[];
}

export interface BuiltinProviderOption {
  id: string;
  name: string;
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
