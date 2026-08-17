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
  kind: "builtin" | "custom";
  keys: ApiKeySummary[];
}

export interface DashboardSummary {
  providerCount: number;
  keyCount: number;
  availableKeyCount: number;
}
