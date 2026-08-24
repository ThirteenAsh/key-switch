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
import xiaomiMimoLogo from "../assets/xiaomimimo.svg";
import type { BuiltinProviderOption } from "../types/domain";
import { isChineseLocale, type AppLocale } from "../i18n/locale";

export const builtinProviderCatalog: BuiltinProviderOption[] = [
  { id: "openai", nameZh: "OpenAI", nameEn: "OpenAI", abbreviation: "OP", tone: "blue", logo: openaiLogo, platformUrl: "https://platform.openai.com" },
  { id: "claude", nameZh: "Claude", nameEn: "Claude", abbreviation: "CL", tone: "orange", logo: claudeLogo, platformUrl: "https://console.anthropic.com" },
  { id: "gemini", nameZh: "Gemini", nameEn: "Gemini", abbreviation: "GE", tone: "indigo", logo: geminiLogo, platformUrl: "https://aistudio.google.com" },
  { id: "deepseek", nameZh: "DeepSeek", nameEn: "DeepSeek", abbreviation: "DS", tone: "violet", logo: deepseekLogo, platformUrl: "https://platform.deepseek.com" },
  { id: "opencode", nameZh: "OpenCode", nameEn: "OpenCode", abbreviation: "OC", tone: "gray", logo: opencodeLogo, platformUrl: "https://opencode.ai" },
  { id: "qwen", nameZh: "通义千问", nameEn: "Qwen", abbreviation: "QW", tone: "indigo", logo: qwenLogo, platformUrl: "https://bailian.console.aliyun.com" },
  { id: "kimi", nameZh: "Kimi", nameEn: "Kimi", abbreviation: "KM", tone: "blue", logo: kimiLogo, platformUrl: "https://platform.moonshot.cn" },
  { id: "zhipu", nameZh: "智谱 AI", nameEn: "Zhipu AI", abbreviation: "ZP", tone: "blue", logo: zhipuLogo, platformUrl: "https://open.bigmodel.cn" },
  { id: "mimo", nameZh: "小米 MiMo", nameEn: "Xiaomi MiMo", abbreviation: "MM", tone: "orange", logo: xiaomiMimoLogo, platformUrl: "https://platform.xiaomimimo.com" },
  { id: "grok", nameZh: "Grok", nameEn: "Grok", abbreviation: "GK", tone: "gray", logo: grokLogo, platformUrl: "https://console.x.ai" },
  { id: "meta", nameZh: "Meta AI", nameEn: "Meta AI", abbreviation: "MT", tone: "blue", logo: metaLogo, platformUrl: "https://llama.developer.meta.com" },
  { id: "openrouter", nameZh: "OpenRouter", nameEn: "OpenRouter", abbreviation: "OR", tone: "gray", logo: openRouterLogo, platformUrl: "https://openrouter.ai" },
  { id: "minimax", nameZh: "MiniMax", nameEn: "MiniMax", abbreviation: "MM", tone: "orange", logo: minimaxLogo, platformUrl: "https://platform.minimaxi.com" },
  { id: "doubao", nameZh: "豆包", nameEn: "Doubao", abbreviation: "DB", tone: "orange", logo: doubaoLogo, platformUrl: "https://console.volcengine.com/ark" },
  { id: "hunyuan", nameZh: "腾讯混元", nameEn: "Tencent Hunyuan", abbreviation: "HY", tone: "indigo", logo: hunyuanLogo, platformUrl: "https://console.cloud.tencent.com/maas" },
  { id: "qianfan", nameZh: "百度千帆", nameEn: "Baidu Qianfan", abbreviation: "QF", tone: "blue", logo: baiduCloudLogo, platformUrl: "https://console.bce.baidu.com/qianfan" },
  { id: "anthropic", nameZh: "Anthropic", nameEn: "Anthropic", abbreviation: "AN", tone: "gray", logo: anthropicLogo, platformUrl: "https://console.anthropic.com" },
  { id: "aistudio", nameZh: "AI Studio", nameEn: "AI Studio", abbreviation: "AI", tone: "indigo", logo: aiStudioLogo, platformUrl: "https://aistudio.google.com" },
];

export function getBuiltinProviderName(provider: BuiltinProviderOption, locale: AppLocale): string {
  return isChineseLocale(locale) ? provider.nameZh : provider.nameEn;
}
