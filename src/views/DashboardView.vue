<script setup lang="ts">
import { computed, ref } from "vue";
import { ChevronDown, ChevronUp, Clipboard, Eye, Info, KeyRound, Layers3, Plus, Search, ShieldCheck } from "@lucide/vue";
import ProviderAvatar from "../components/ProviderAvatar.vue";
import StatusBadge from "../components/StatusBadge.vue";
import { useDashboardStore } from "../stores/dashboard";

const store = useDashboardStore();
const notice = ref("");
const hasResults = computed(() => store.filteredProviders.length > 0);

function notify(message: string) {
  notice.value = message;
  window.setTimeout(() => {
    notice.value = "";
  }, 2800);
}

function handleCopy() {
  notify("安全复制将在本地密钥存储接入后启用");
}
</script>

<template>
  <section class="dashboard-view">
    <div class="view-toolbar">
      <h1>仪表盘</h1>
      <div class="toolbar-actions">
        <label class="search-field">
          <Search :size="18" aria-hidden="true" />
          <input v-model="store.query" type="search" placeholder="搜索供应商或 Key 备注" aria-label="搜索供应商或 Key 备注" />
        </label>
        <button class="button button--primary" type="button" @click="notify('供应商创建表单将在数据层接入后启用')">
          <Plus :size="18" /> 添加供应商
        </button>
      </div>
    </div>

    <div class="stat-grid" aria-label="Key 统计">
      <article class="stat-card">
        <span class="stat-icon stat-icon--blue"><Layers3 :size="24" /></span>
        <div><p>供应商数量</p><strong>{{ store.summary.providerCount }}</strong></div>
      </article>
      <article class="stat-card">
        <span class="stat-icon stat-icon--green"><KeyRound :size="24" /></span>
        <div><p>Key 总数</p><strong>{{ store.summary.keyCount }}</strong></div>
      </article>
      <article class="stat-card">
        <span class="stat-icon stat-icon--green"><ShieldCheck :size="24" /></span>
        <div><p>可用 Key</p><strong>{{ store.summary.availableKeyCount }}</strong></div>
      </article>
    </div>

    <div v-if="hasResults" class="provider-list">
      <article v-for="provider in store.filteredProviders" :key="provider.id" class="provider-panel" :class="{ 'provider-panel--expanded': store.expandedProviderId === provider.id }">
        <div class="provider-summary" @click="store.toggleProvider(provider.id)">
          <div class="provider-name">
            <ProviderAvatar :provider="provider" />
            <strong>{{ provider.name }}</strong>
            <span v-if="store.expandedProviderId === provider.id" class="key-count">{{ provider.keys.length }} 个 Key</span>
          </div>
          <div class="provider-actions" @click.stop>
            <button v-if="store.expandedProviderId === provider.id" class="button button--secondary button--small" type="button" @click="notify('Key 创建表单将在数据层接入后启用')"><Plus :size="19" /> 添加 Key</button>
            <button class="icon-button" :aria-label="store.expandedProviderId === provider.id ? '收起' : '展开'" type="button" @click="store.toggleProvider(provider.id)">
              <ChevronUp v-if="store.expandedProviderId === provider.id" :size="20" />
              <ChevronDown v-else :size="20" />
            </button>
          </div>
        </div>

        <div v-if="store.expandedProviderId === provider.id" class="key-table-wrap">
          <table class="key-table">
            <thead><tr><th>备注</th><th>API Key（部分隐藏）</th><th>状态</th><th>操作</th></tr></thead>
            <tbody>
              <tr v-for="key in provider.keys.slice(0, 3)" :key="key.id">
                <td>{{ key.remark }}</td>
                <td class="masked-key"><code>{{ key.maskedValue }}</code><button class="table-icon" type="button" aria-label="临时查看完整 Key" @click="notify('完整 Key 仅会由本地安全存储按需提供')"><Eye :size="18" /></button></td>
                <td><StatusBadge :status="key.status" /></td>
                <td><button class="button button--secondary button--compact" type="button" @click="handleCopy"><Clipboard :size="17" />复制</button></td>
              </tr>
            </tbody>
          </table>
          <p class="key-disclosure"><Info :size="18" />仅显示部分 Key；完整密钥不会在列表中返回。</p>
        </div>
      </article>
    </div>

    <div v-else class="empty-state">
      <Search :size="34" />
      <h2>没有找到匹配的供应商或备注</h2>
      <p>尝试使用其他关键词搜索。</p>
      <button class="button button--secondary" type="button" @click="store.query = ''">清除搜索</button>
    </div>

    <Transition name="toast"><p v-if="notice" class="toast" role="status">{{ notice }}</p></Transition>
  </section>
</template>
