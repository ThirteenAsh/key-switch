import aiStudioLogo from "../assets/aistudio.svg";
import anthropicLogo from "../assets/anthropic.svg";
import claudeLogo from "../assets/claude-color.svg";
import deepseekLogo from "../assets/deepseek-color.svg";
import doubaoLogo from "../assets/doubao-color.svg";
import geminiLogo from "../assets/gemini-color.svg";
import grokLogo from "../assets/grok.svg";
import kimiLogo from "../assets/kimi-color.svg";
import minimaxLogo from "../assets/minimax-color.svg";
import openaiLogo from "../assets/openai.svg";
import opencodeLogo from "../assets/opencode.svg";
import qwenLogo from "../assets/qwen-color.svg";
import zhipuLogo from "../assets/zhipu-color.svg";
import type { BuiltinProviderOption } from "../types/domain";

export const builtinProviderCatalog: BuiltinProviderOption[] = [
  { id: "openai", name: "OpenAI", abbreviation: "OP", tone: "blue", logo: openaiLogo },
  { id: "deepseek", name: "DeepSeek", abbreviation: "DS", tone: "violet", logo: deepseekLogo },
  { id: "claude", name: "Claude", abbreviation: "CL", tone: "orange", logo: claudeLogo },
  { id: "gemini", name: "Gemini", abbreviation: "GE", tone: "indigo", logo: geminiLogo },
  { id: "grok", name: "Grok", abbreviation: "GK", tone: "gray", logo: grokLogo },
  { id: "kimi", name: "Kimi", abbreviation: "KM", tone: "blue", logo: kimiLogo },
  { id: "minimax", name: "MiniMax", abbreviation: "MM", tone: "orange", logo: minimaxLogo },
  { id: "qwen", name: "通义千问", abbreviation: "QW", tone: "indigo", logo: qwenLogo },
  { id: "zhipu", name: "智谱 AI", abbreviation: "ZP", tone: "blue", logo: zhipuLogo },
  { id: "doubao", name: "豆包", abbreviation: "DB", tone: "orange", logo: doubaoLogo },
  { id: "aistudio", name: "AI Studio", abbreviation: "AI", tone: "indigo", logo: aiStudioLogo },
  { id: "anthropic", name: "Anthropic", abbreviation: "AN", tone: "gray", logo: anthropicLogo },
  { id: "opencode", name: "OpenCode", abbreviation: "OC", tone: "gray", logo: opencodeLogo },
];
