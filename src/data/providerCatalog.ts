import aiStudioLogo from "../assets/aistudio.svg";
import anthropicLogo from "../assets/anthropic.svg";
import baiduCloudLogo from "../assets/baiducloud-color.svg";
import claudeLogo from "../assets/claude-color.svg";
import deepseekLogo from "../assets/deepseek-color.svg";
import doubaoLogo from "../assets/doubao-color.svg";
import geminiLogo from "../assets/gemini-color.svg";
import grokLogo from "../assets/grok.svg";
import hunyuanLogo from "../assets/hunyuan-color.svg";
import kimiLogo from "../assets/kimi-color.svg";
import metaLogo from "../assets/meta-color.svg";
import minimaxLogo from "../assets/minimax-color.svg";
import openaiLogo from "../assets/openai.svg";
import opencodeLogo from "../assets/opencode.svg";
import openRouterLogo from "../assets/openrouter-color.svg";
import qwenLogo from "../assets/qwen-color.svg";
import zhipuLogo from "../assets/zhipu-color.svg";
import type { BuiltinProviderOption } from "../types/domain";

export const builtinProviderCatalog: BuiltinProviderOption[] = [
  { id: "openai", name: "OpenAI", abbreviation: "OP", tone: "blue", logo: openaiLogo, baseUrl: "https://api.openai.com/v1" },
  { id: "claude", name: "Claude", abbreviation: "CL", tone: "orange", logo: claudeLogo, baseUrl: "https://api.anthropic.com/v1" },
  { id: "gemini", name: "Gemini", abbreviation: "GE", tone: "indigo", logo: geminiLogo, baseUrl: "https://generativelanguage.googleapis.com/v1beta" },
  { id: "deepseek", name: "DeepSeek", abbreviation: "DS", tone: "violet", logo: deepseekLogo, baseUrl: "https://api.deepseek.com" },
  { id: "opencode", name: "OpenCode", abbreviation: "OC", tone: "gray", logo: opencodeLogo, baseUrl: "https://opencode.ai/zen/v1" },
  { id: "qwen", name: "通义千问", abbreviation: "QW", tone: "indigo", logo: qwenLogo, baseUrl: "https://dashscope.aliyuncs.com/compatible-mode/v1" },
  { id: "kimi", name: "Kimi", abbreviation: "KM", tone: "blue", logo: kimiLogo, baseUrl: "https://api.moonshot.cn/v1" },
  { id: "zhipu", name: "智谱 AI", abbreviation: "ZP", tone: "blue", logo: zhipuLogo, baseUrl: "https://open.bigmodel.cn/api/paas/v4" },
  { id: "grok", name: "Grok", abbreviation: "GK", tone: "gray", logo: grokLogo, baseUrl: "https://api.x.ai/v1" },
  { id: "meta", name: "Meta AI", abbreviation: "MT", tone: "blue", logo: metaLogo, baseUrl: "https://api.llama.com/compat/v1" },
  { id: "openrouter", name: "OpenRouter", abbreviation: "OR", tone: "gray", logo: openRouterLogo, baseUrl: "https://openrouter.ai/api/v1" },
  { id: "minimax", name: "MiniMax", abbreviation: "MM", tone: "orange", logo: minimaxLogo, baseUrl: "https://api.minimaxi.com/v1" },
  { id: "doubao", name: "豆包", abbreviation: "DB", tone: "orange", logo: doubaoLogo, baseUrl: "https://ark.cn-beijing.volces.com/api/v3" },
  { id: "hunyuan", name: "腾讯混元", abbreviation: "HY", tone: "indigo", logo: hunyuanLogo, baseUrl: "https://tokenhub.tencentmaas.com/v1" },
  { id: "qianfan", name: "百度千帆", abbreviation: "QF", tone: "blue", logo: baiduCloudLogo, baseUrl: "https://qianfan.baidubce.com/v2" },
  { id: "anthropic", name: "Anthropic", abbreviation: "AN", tone: "gray", logo: anthropicLogo, baseUrl: "https://api.anthropic.com/v1" },
  { id: "aistudio", name: "AI Studio", abbreviation: "AI", tone: "indigo", logo: aiStudioLogo, baseUrl: "https://generativelanguage.googleapis.com/v1beta" },
];
