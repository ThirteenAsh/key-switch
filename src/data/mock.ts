import type { ProviderSummary } from "../types/domain";

export const mockProviders: ProviderSummary[] = [
  {
    id: "openai",
    name: "OpenAI",
    abbreviation: "OP",
    tone: "blue",
    kind: "builtin",
    keys: [
      { id: "openai-production", providerId: "openai", remark: "Production", maskedValue: "sk-proj-*****8K2x", status: "valid" },
      { id: "openai-test", providerId: "openai", remark: "Test", maskedValue: "sk-proj-*****3Fa9", status: "untested" },
      { id: "openai-backup", providerId: "openai", remark: "Backup", maskedValue: "sk-*****92Ls", status: "invalid" },
      { id: "openai-staging", providerId: "openai", remark: "Staging", maskedValue: "sk-proj-*****1Qm7", status: "valid" },
      { id: "openai-research", providerId: "openai", remark: "Research", maskedValue: "sk-proj-*****0Vb4", status: "valid" },
      { id: "openai-automation", providerId: "openai", remark: "Automation", maskedValue: "sk-proj-*****9Df2", status: "valid" },
      { id: "openai-sandbox", providerId: "openai", remark: "Sandbox", maskedValue: "sk-proj-*****4Hs8", status: "untested" },
      { id: "openai-legacy", providerId: "openai", remark: "Legacy", maskedValue: "sk-*****6Nt3", status: "invalid" },
    ],
  },
  {
    id: "deepseek",
    name: "DeepSeek",
    abbreviation: "DS",
    tone: "violet",
    kind: "builtin",
    keys: Array.from({ length: 5 }, (_, index) => ({ id: `deepseek-${index}`, providerId: "deepseek", remark: `Key ${index + 1}`, maskedValue: "sk-*****", status: "valid" as const })),
  },
  {
    id: "claude",
    name: "Claude",
    abbreviation: "CL",
    tone: "orange",
    kind: "builtin",
    keys: Array.from({ length: 4 }, (_, index) => ({ id: `claude-${index}`, providerId: "claude", remark: `Key ${index + 1}`, maskedValue: "sk-ant-*****", status: "untested" as const })),
  },
  {
    id: "gemini",
    name: "Gemini",
    abbreviation: "GE",
    tone: "indigo",
    kind: "builtin",
    keys: Array.from({ length: 3 }, (_, index) => ({ id: `gemini-${index}`, providerId: "gemini", remark: `Key ${index + 1}`, maskedValue: "AIza*****", status: "valid" as const })),
  },
  {
    id: "custom",
    name: "自定义供应商",
    abbreviation: "CU",
    tone: "gray",
    kind: "custom",
    keys: Array.from({ length: 3 }, (_, index) => ({ id: `custom-${index}`, providerId: "custom", remark: `Key ${index + 1}`, maskedValue: "••••••••", status: "untested" as const })),
  },
];
