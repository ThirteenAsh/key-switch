<script setup lang="ts">
import { computed, ref, onUnmounted, nextTick } from "vue";
import {
  ChevronDown,
  ChevronRight,
  Clipboard,
  Trash2,
  Check,
  Info,
  KeyRound,
  Cpu,
  Plus,
  Search,
  CheckCircle2,
  GripVertical,
  RefreshCw
} from "@lucide/vue";
import { openUrl } from "@tauri-apps/plugin-opener";
import AppButton from "../components/ui/AppButton.vue";
import ProviderAvatar from "../components/ProviderAvatar.vue";
import ProviderConfigDialog from "../components/ProviderConfigDialog.vue";
import ApiKeyDialog from "../components/ApiKeyDialog.vue";
import ConfirmDialog from "../components/ConfirmDialog.vue";
import StatusBadge from "../components/StatusBadge.vue";
import { useDashboardStore } from "../stores/dashboard";
import { copyApiKey } from "../api/app";
import type { ProviderSummary } from "../types/domain";

const store = useDashboardStore();
const notice = ref("");
const configDialogOpen = ref(false);
const copiedKeyId = ref<string | null>(null);
const checkingProviderId = ref<string | null>(null);
const keyDialogProvider = ref<ProviderSummary | null>(null);
const deleteTarget = ref<{ providerId: string; keyId: string } | null>(null);

const hasResults = computed(() => store.filteredProviders.length > 0);
const hasConfiguredProviders = computed(() => store.providers.length > 0);

function notify(message: string) {
  notice.value = message;
  window.setTimeout(() => {
    notice.value = "";
  }, 2800);
}

async function handleCopy(keyId: string) {
  try {
    await copyApiKey(keyId);
    copiedKeyId.value = keyId;
    notify("已复制到剪贴板");
    setTimeout(() => { if (copiedKeyId.value === keyId) copiedKeyId.value = null; }, 2000);
  } catch { notify("复制失败"); }
}

async function handleRefreshProvider(providerId: string, event: MouseEvent) {
  event.stopPropagation();
  checkingProviderId.value = providerId;
  notify("正在检测该供应商下所有 Key 状态...");
  try { await store.checkKeys(providerId); notify("状态检测完成"); }
  catch { notify("检测失败，请检查检测地址和网络"); }
  finally { checkingProviderId.value = null; }
}

async function addKey(payload: { remark: string; value: string }) {
  if (!keyDialogProvider.value) return;
  try { await store.addKey({ providerId: keyDialogProvider.value.id, ...payload }); keyDialogProvider.value = null; notify("API Key 已保存"); }
  catch { notify("保存 API Key 失败"); }
}

function requestDeleteKey(providerId: string, keyId: string) { deleteTarget.value = { providerId, keyId }; }
async function deleteKey() {
  if (!deleteTarget.value) return;
  const target = deleteTarget.value;
  try { await store.deleteKey(target.providerId, target.keyId); notify("API Key 已删除"); }
  catch { notify("删除 API Key 失败"); }
  finally { deleteTarget.value = null; }
}

// ======================= 类似 cc-switch 的精准拖拽排序引擎 =======================
interface DragState {
  fromIndex: number;
  currentIndex: number;
  startY: number;
  currentY: number;
  isActivated: boolean; // 拖拽激活阈值（移动 > 4px 激活）
  pointerId: number;
  handleElement: HTMLElement;
  cardOffsets: number[];
}

const dragState = ref<DragState | null>(null);
const isCommitting = ref(false);
const providerListRef = ref<HTMLElement | null>(null);

function getProviderPanels(): HTMLElement[] {
  if (!providerListRef.value) return [];
  return Array.from(providerListRef.value.children).filter(
    (element): element is HTMLElement => element instanceof HTMLElement,
  );
}

function getCardOffsets(fromIndex: number, toIndex: number, panels: HTMLElement[]): number[] {
  const offsets = panels.map(() => 0);

  if (fromIndex < toIndex) {
    for (let index = fromIndex + 1; index <= toIndex; index += 1) {
      offsets[index] = panels[index - 1].offsetTop - panels[index].offsetTop;
    }
  } else if (fromIndex > toIndex) {
    for (let index = toIndex; index < fromIndex; index += 1) {
      offsets[index] = panels[index + 1].offsetTop - panels[index].offsetTop;
    }
  }

  return offsets;
}

function handleHandlePointerDown(index: number, event: PointerEvent) {
  if (event.button !== 0) return;

  const handleElement = event.currentTarget;
  if (!(handleElement instanceof HTMLElement)) return;

  // 1. 如果有卡片处于展开状态，立即自动收起
  if (store.expandedProviderId) {
    store.expandedProviderId = "";
  }

  handleElement.setPointerCapture(event.pointerId);

  dragState.value = {
    fromIndex: index,
    currentIndex: index,
    startY: event.clientY,
    currentY: event.clientY,
    isActivated: false,
    pointerId: event.pointerId,
    handleElement,
    cardOffsets: [],
  };

  window.addEventListener("pointermove", onPointerMove, { passive: false });
  // pointer capture 会改变事件目标。结束事件使用捕获阶段监听，避免被 WebView
  // 或中间节点截断；mouseup 和 lostpointercapture 作为桌面端兜底。
  document.addEventListener("pointerup", onPointerUp, true);
  document.addEventListener("pointercancel", onPointerCancel, true);
  document.addEventListener("mouseup", onMouseUp, true);
  handleElement.addEventListener("pointerup", onPointerUp);
  handleElement.addEventListener("pointercancel", onPointerCancel);
  handleElement.addEventListener("lostpointercapture", onLostPointerCapture);
  window.addEventListener("blur", onWindowBlur);

  event.preventDefault();
  event.stopPropagation();
}

function onPointerMove(event: PointerEvent) {
  if (!dragState.value || event.pointerId !== dragState.value.pointerId) return;

  if (updateDragPosition(event.clientY)) {
    event.preventDefault();
  }
}

function updateDragPosition(clientY: number): boolean {
  const state = dragState.value;
  if (!state) return false;

  state.currentY = clientY;
  const deltaY = clientY - state.startY;

  // 距离阈值检测：移动超过 4px 才激活拖拽
  if (!state.isActivated) {
    if (Math.abs(deltaY) > 4) {
      state.isActivated = true;
    } else {
      return false;
    }
  }

  const panels = getProviderPanels();
  const draggedPanel = panels[state.fromIndex];
  if (!draggedPanel) return false;

  // offsetTop 不受 transform 影响，因此可以稳定地用真实卡片尺寸计算落点。
  // 这也能正确处理边框、网格间距及展开卡片收起后的高度变化。
  const draggedCenter = draggedPanel.offsetTop + draggedPanel.offsetHeight / 2 + deltaY;
  let targetIndex = state.fromIndex;
  let nearestDistance = Number.POSITIVE_INFINITY;

  panels.forEach((panel, index) => {
    const panelCenter = panel.offsetTop + panel.offsetHeight / 2;
    const distance = Math.abs(draggedCenter - panelCenter);
    if (distance < nearestDistance) {
      nearestDistance = distance;
      targetIndex = index;
    }
  });

  if (targetIndex !== state.currentIndex) {
    state.currentIndex = targetIndex;
  }

  state.cardOffsets = getCardOffsets(
    state.fromIndex,
    state.currentIndex,
    panels,
  );

  return true;
}

function onPointerUp(event: PointerEvent) {
  if (dragState.value && event.pointerId !== dragState.value.pointerId) return;
  updateDragPosition(event.clientY);
  finishDrag(true);
}

function onPointerCancel(event: PointerEvent) {
  if (dragState.value && event.pointerId !== dragState.value.pointerId) return;
  // WebView 在指针捕获结束时可能以 pointercancel 代替 pointerup；
  // 此时应提交最后一次有效落点，而不是只清除拖拽状态。
  finishDrag(true);
}

function onMouseUp(event: MouseEvent) {
  if (event.button !== 0) return;
  updateDragPosition(event.clientY);
  finishDrag(true);
}

function onLostPointerCapture(event: PointerEvent) {
  if (dragState.value && event.pointerId !== dragState.value.pointerId) return;
  finishDrag(true);
}

function onWindowBlur() {
  // 窗口失焦时无法判断鼠标是否仍按下，取消本次排序，但必须清除悬浮状态。
  finishDrag(false);
}

function captureProviderRects(): Map<string, DOMRect> {
  const rects = new Map<string, DOMRect>();
  const panels = getProviderPanels();

  store.filteredProviders.forEach((provider, index) => {
    const panel = panels[index];
    if (panel) rects.set(provider.id, panel.getBoundingClientRect());
  });

  return rects;
}

function animateProviderDrop(previousRects: Map<string, DOMRect>, movedProviderId: string) {
  const reduceMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  const panels = getProviderPanels();

  if (!reduceMotion) {
    store.filteredProviders.forEach((provider, index) => {
      const panel = panels[index];
      const previousRect = previousRects.get(provider.id);
      if (!panel || !previousRect) return;

      const currentRect = panel.getBoundingClientRect();
      const previousCenter = previousRect.top + previousRect.height / 2;
      const currentCenter = currentRect.top + currentRect.height / 2;
      const deltaY = previousCenter - currentCenter;
      const scaleX = previousRect.width / currentRect.width;
      const scaleY = previousRect.height / currentRect.height;
      const isMovedPanel = provider.id === movedProviderId;

      if (!isMovedPanel && Math.abs(deltaY) < 0.5) return;

      const settledBorderColor = getComputedStyle(panel).borderColor;
      const startFrame: Keyframe = {
        transform: `translateY(${deltaY}px) scale(${scaleX}, ${scaleY})`,
      };
      const endFrame: Keyframe = {
        transform: "translateY(0px) scale(1, 1)",
      };

      if (isMovedPanel) {
        startFrame.boxShadow = "0 12px 28px rgba(15, 23, 42, 0.14)";
        startFrame.borderColor = "#38bdf8";
        endFrame.boxShadow = "0 0 0 rgba(15, 23, 42, 0)";
        endFrame.borderColor = settledBorderColor;
      }

      panel.animate([startFrame, endFrame], {
        duration: isMovedPanel ? 260 : 220,
        easing: "cubic-bezier(0.22, 1, 0.36, 1)",
      });
    });
  }

  isCommitting.value = false;
}

function finishDrag(commitOrder: boolean) {
  const state = dragState.value;
  if (!state) return;

  // 先清空响应式状态，确保后续即使排序提交异常，卡片也不会残留悬浮样式。
  dragState.value = null;
  cleanupPointerListeners(state);

  const { fromIndex, currentIndex, isActivated } = state;
  if (!commitOrder || !isActivated || fromIndex === currentIndex) return;

  const fromProvider = store.filteredProviders[fromIndex];
  const toProvider = store.filteredProviders[currentIndex];
  if (!fromProvider || !toProvider) return;

  const fromRealIndex = store.providers.findIndex((provider) => provider.id === fromProvider.id);
  const toRealIndex = store.providers.findIndex((provider) => provider.id === toProvider.id);
  if (fromRealIndex === -1 || toRealIndex === -1) return;

  // 先记录松手时的视觉位置；DOM 换序后用 FLIP 从旧位置过渡到新槽位。
  const previousRects = captureProviderRects();
  isCommitting.value = true;
  store.reorderProviders(fromRealIndex, toRealIndex);

  nextTick(() => {
    animateProviderDrop(previousRects, fromProvider.id);
  });
}

function cleanupPointerListeners(state: DragState | null = dragState.value) {
  window.removeEventListener("pointermove", onPointerMove);
  document.removeEventListener("pointerup", onPointerUp, true);
  document.removeEventListener("pointercancel", onPointerCancel, true);
  document.removeEventListener("mouseup", onMouseUp, true);
  window.removeEventListener("blur", onWindowBlur);

  state?.handleElement.removeEventListener("lostpointercapture", onLostPointerCapture);
  state?.handleElement.removeEventListener("pointerup", onPointerUp);
  state?.handleElement.removeEventListener("pointercancel", onPointerCancel);
  if (state?.handleElement.hasPointerCapture(state.pointerId)) {
    state.handleElement.releasePointerCapture(state.pointerId);
  }
}

onUnmounted(() => {
  const state = dragState.value;
  dragState.value = null;
  cleanupPointerListeners(state);
});

const ACCORDION_TRANSITION = "height 240ms cubic-bezier(0.22, 1, 0.36, 1), padding-top 240ms cubic-bezier(0.22, 1, 0.36, 1), padding-bottom 240ms cubic-bezier(0.22, 1, 0.36, 1), border-top-color 180ms ease-out, opacity 180ms ease-out, transform 240ms cubic-bezier(0.22, 1, 0.36, 1)";

function applyCollapsedAccordionStyles(accordion: HTMLElement) {
  accordion.style.paddingTop = "0px";
  accordion.style.paddingBottom = "0px";
  accordion.style.borderTopColor = "transparent";
}

function applyExpandedAccordionStyles(accordion: HTMLElement) {
  accordion.style.paddingTop = "10px";
  accordion.style.paddingBottom = "14px";
  accordion.style.borderTopColor = "#e2e8f0";
}

function resetAccordionStyles(element: Element) {
  const accordion = element as HTMLElement;
  accordion.style.removeProperty("height");
  accordion.style.removeProperty("opacity");
  accordion.style.removeProperty("overflow");
  accordion.style.removeProperty("transform");
  accordion.style.removeProperty("transition");
  accordion.style.removeProperty("padding-top");
  accordion.style.removeProperty("padding-bottom");
  accordion.style.removeProperty("border-top-color");
}

function waitForAccordionTransition(accordion: HTMLElement, done: () => void) {
  const finish = (event: TransitionEvent) => {
    if (event.target !== accordion || event.propertyName !== "height") return;
    accordion.removeEventListener("transitionend", finish);
    resetAccordionStyles(accordion);
    done();
  };
  accordion.addEventListener("transitionend", finish);
}

function beforeAccordionEnter(element: Element) {
  const accordion = element as HTMLElement;
  accordion.style.height = "0px";
  accordion.style.opacity = "0";
  accordion.style.overflow = "hidden";
  accordion.style.transform = "translateY(-4px)";
  applyCollapsedAccordionStyles(accordion);
}

function enterAccordion(element: Element, done: () => void) {
  const accordion = element as HTMLElement;
  // 先以展开态测量，得到包含内边距和分隔线的真实高度；再还原为收起态。
  applyExpandedAccordionStyles(accordion);
  accordion.style.height = "auto";
  const targetHeight = accordion.offsetHeight;
  accordion.style.height = "0px";
  applyCollapsedAccordionStyles(accordion);
  waitForAccordionTransition(accordion, done);

  requestAnimationFrame(() => {
    accordion.style.transition = ACCORDION_TRANSITION;
    accordion.style.height = `${targetHeight}px`;
    accordion.style.opacity = "1";
    accordion.style.transform = "translateY(0)";
    applyExpandedAccordionStyles(accordion);
  });
}

function beforeAccordionLeave(element: Element) {
  const accordion = element as HTMLElement;
  accordion.style.height = `${accordion.offsetHeight}px`;
  accordion.style.overflow = "hidden";
}

function leaveAccordion(element: Element, done: () => void) {
  const accordion = element as HTMLElement;
  waitForAccordionTransition(accordion, done);

  requestAnimationFrame(() => {
    accordion.style.transition = ACCORDION_TRANSITION;
    accordion.style.height = "0px";
    accordion.style.opacity = "0";
    accordion.style.transform = "translateY(-4px)";
    applyCollapsedAccordionStyles(accordion);
  });
}

// 计算卡片实时物理位置
function getCardTransform(index: number): { transform: string } {
  if (!dragState.value || !dragState.value.isActivated) {
    return { transform: "translateY(0px)" };
  }

  const { fromIndex, startY, currentY, cardOffsets } = dragState.value;

  if (index === fromIndex) {
    const deltaY = currentY - startY;
    return { transform: `translateY(${deltaY}px) scale(1.01)` };
  }

  return { transform: `translateY(${cardOffsets[index] ?? 0}px)` };
}

function getProviderEndpoint(provider: ProviderSummary): string {
  return provider.platformUrl || "未配置平台管理地址";
}

async function openProviderPlatform(url: string) {
  try {
    const parsedUrl = new URL(url);
    if (parsedUrl.protocol !== "https:" && parsedUrl.protocol !== "http:") {
      throw new Error("不支持的链接协议");
    }

    if ("__TAURI_INTERNALS__" in window) {
      await openUrl(parsedUrl);
      return;
    }

    window.open(parsedUrl.href, "_blank", "noopener,noreferrer");
  } catch {
    notify("无法打开该供应商的平台管理地址");
  }
}

function getAvailableCount(provider: ProviderSummary): number {
  return provider.keys.filter(k => k.status === 'valid').length;
}

async function addBuiltinProvider(providerId: string) {
  if (!await store.addBuiltinProvider(providerId)) {
    notify("该供应商已配置");
    return;
  }
  configDialogOpen.value = false;
  notify("已新增供应商配置");
}

async function addCustomProvider(name: string, platformUrl: string, logo?: string) {
  if (!await store.addCustomProvider(name, platformUrl, logo)) {
    notify("供应商名称已存在");
    return;
  }
  configDialogOpen.value = false;
  notify("已新增自定义供应商配置");
}
</script>

<template>
  <section class="dashboard-view">
    <div class="view-toolbar">
      <h1>仪表盘</h1>
      <div class="toolbar-actions">
        <label class="search-field">
          <Search :size="15" :stroke-width="2" aria-hidden="true" />
          <input v-model="store.query" type="search" placeholder="搜索供应商或 Key 备注" aria-label="搜索供应商或 Key 备注" />
        </label>
        <AppButton variant="primary" @click="configDialogOpen = true">
          <Plus :size="15" :stroke-width="2.2" />
          <span>新增配置</span>
        </AppButton>
      </div>
    </div>

    <!-- 顶部统计卡片 -->
    <div class="stat-grid" aria-label="Key 统计">
      <article class="stat-card">
        <span class="stat-icon stat-icon--blue">
          <Cpu :size="20" :stroke-width="1.9" />
        </span>
        <div><p>供应商数量</p><strong>{{ store.summary.providerCount }}</strong></div>
      </article>
      <article class="stat-card">
        <span class="stat-icon stat-icon--green">
          <KeyRound :size="20" :stroke-width="1.9" />
        </span>
        <div><p>Key 总数</p><strong>{{ store.summary.keyCount }}</strong></div>
      </article>
      <article class="stat-card">
        <span class="stat-icon stat-icon--emerald">
          <CheckCircle2 :size="20" :stroke-width="1.9" />
        </span>
        <div><p>可用 Key</p><strong>{{ store.summary.availableKeyCount }}</strong></div>
      </article>
    </div>

    <!-- 供应商卡片列表 -->
    <div v-if="hasResults">
      <div ref="providerListRef" class="provider-list" :class="{ 'is-committing': isCommitting }">
        <article
          v-for="(provider, index) in store.filteredProviders"
          :key="provider.id"
          class="provider-panel"
          :class="{
            'provider-panel--active': store.expandedProviderId === provider.id,
            'is-dragging': dragState?.isActivated && dragState?.fromIndex === index
          }"
          :style="getCardTransform(index)"
        >
          <div class="provider-summary" @click="store.toggleProvider(provider.id)">
            <!-- 左侧：拖拽手柄 + 图标 + 名称与链接 -->
            <div class="provider-main-info">
              <span
                class="drag-handle"
                title="按住拖拽调整顺序"
                @pointerdown="handleHandlePointerDown(index, $event)"
                @click.stop
              >
                <GripVertical :size="16" :stroke-width="2" />
              </span>
              <ProviderAvatar :provider="provider" />
              <div class="provider-text-meta">
                <div class="provider-title-row">
                  <strong class="provider-name-text">{{ provider.name }}</strong>
                  <span class="provider-tag" :class="provider.kind === 'builtin' ? 'provider-tag--builtin' : 'provider-tag--custom'">
                    {{ provider.kind === 'builtin' ? '官方' : '自定义' }}
                  </span>
                </div>
                <a v-if="provider.platformUrl" :href="provider.platformUrl" class="provider-endpoint-link" @click.stop.prevent="openProviderPlatform(provider.platformUrl)">
                  {{ getProviderEndpoint(provider) }}
                </a>
                <span v-else class="provider-endpoint-link provider-endpoint-link--empty">{{ getProviderEndpoint(provider) }}</span>
              </div>
            </div>

            <!-- 右侧：状态 + 刷新 + 操作按钮 -->
            <div class="provider-right-meta">
              <div class="provider-status-column">
                <div class="provider-quota-summary">
                  <span>Key: </span>
                  <strong class="quota-highlight">{{ provider.keys.length }}</strong>
                  <span class="quota-available">（{{ getAvailableCount(provider) }} 可用）</span>
                </div>
              </div>

              <!-- 刷新检测按钮 -->
              <button
                class="refresh-btn"
                :class="{ 'is-spinning': checkingProviderId === provider.id }"
                title="重新检测状态"
                type="button"
                @click="handleRefreshProvider(provider.id, $event)"
              >
                <RefreshCw :size="16" :stroke-width="2" />
              </button>

              <div class="provider-actions" @click.stop>
                <AppButton
                  v-if="store.expandedProviderId === provider.id"
                  variant="secondary"
                  size="sm"
                  @click="keyDialogProvider = provider"
                >
                  <Plus :size="13" :stroke-width="2.2" />
                  <span>添加 Key</span>
                </AppButton>
                <AppButton
                  variant="ghost"
                  size="icon-sm"
                  :aria-label="store.expandedProviderId === provider.id ? '收起' : '展开'"
                  @click="store.toggleProvider(provider.id)"
                >
                  <ChevronDown v-if="store.expandedProviderId === provider.id" :size="16" :stroke-width="2" />
                  <ChevronRight v-else :size="16" :stroke-width="2" />
                </AppButton>
              </div>
            </div>
          </div>

          <!-- 展开后的表格列表 -->
          <Transition
            :css="false"
            @before-enter="beforeAccordionEnter"
            @enter="enterAccordion"
            @before-leave="beforeAccordionLeave"
            @leave="leaveAccordion"
          >
            <div v-if="store.expandedProviderId === provider.id" class="key-table-wrap">
              <table class="key-table">
                <thead>
                  <tr>
                    <th>备注</th>
                    <th>API Key（部分隐藏）</th>
                    <th>状态</th>
                    <th>操作</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="key in provider.keys" :key="key.id">
                    <td>{{ key.remark }}</td>
                    <td class="masked-key">
                      <code>{{ key.maskedValue }}</code>
                    </td>
                    <td><StatusBadge :status="key.status" /></td>
                    <td>
                      <AppButton
                        :variant="copiedKeyId === key.id ? 'success' : 'ghost'"
                        size="icon-sm"
                        :title="copiedKeyId === key.id ? '已复制' : '复制 Key'"
                        :aria-label="copiedKeyId === key.id ? '已复制' : '复制 Key'"
                        @click="handleCopy(key.id)"
                      >
                        <Check v-if="copiedKeyId === key.id" :size="14" :stroke-width="2.2" />
                        <Clipboard v-else :size="14" :stroke-width="1.9" />
                      </AppButton>
                      <AppButton
                        variant="danger"
                        size="icon-sm"
                        title="删除 Key"
                        aria-label="删除 Key"
                        @click="requestDeleteKey(provider.id, key.id)"
                      >
                        <Trash2 :size="14" :stroke-width="1.9" />
                      </AppButton>
                    </td>
                  </tr>
                </tbody>
              </table>
              <p class="key-disclosure">
                <Info :size="14" :stroke-width="1.8" />
                <span>仅显示部分 Key；完整密钥不会在列表中返回。</span>
              </p>
            </div>
          </Transition>
        </article>
      </div>
    </div>

    <div v-else class="empty-state">
      <Search :size="32" :stroke-width="1.7" />
      <h2>{{ hasConfiguredProviders ? "没有找到匹配的供应商或备注" : "还没有供应商配置" }}</h2>
      <p>{{ hasConfiguredProviders ? "尝试使用其他关键词搜索。" : "从内置供应商开始，或创建一个自定义供应商。" }}</p>
      <AppButton variant="secondary" @click="hasConfiguredProviders ? store.query = '' : configDialogOpen = true">
        {{ hasConfiguredProviders ? "清除搜索" : "新增配置" }}
      </AppButton>
    </div>

    <Transition name="toast"><p v-if="notice" class="toast" role="status">{{ notice }}</p></Transition>
    <ProviderConfigDialog :open="configDialogOpen" :configured-provider-ids="store.providers.map((provider) => provider.id)" @close="configDialogOpen = false" @add-builtin="addBuiltinProvider" @add-custom="addCustomProvider" />
    <ApiKeyDialog :open="Boolean(keyDialogProvider)" :provider-name="keyDialogProvider?.name ?? ''" @close="keyDialogProvider = null" @save="addKey" />
    <ConfirmDialog :open="Boolean(deleteTarget)" title="删除 API Key" message="确定删除此 API Key 吗？此操作无法撤销。" @close="deleteTarget = null" @confirm="deleteKey" />
  </section>
</template>

<style scoped>
.provider-panel {
  position: relative;
  touch-action: none;
  user-select: none;
  will-change: transform;
  transition: transform 0.22s cubic-bezier(0.16, 1, 0.3, 1),
              border-color 0.24s cubic-bezier(0.22, 1, 0.36, 1),
              box-shadow 0.24s cubic-bezier(0.22, 1, 0.36, 1);
  z-index: 1;
}

.provider-panel::after {
  position: absolute;
  z-index: 2;
  inset: 0;
  padding: 1px;
  pointer-events: none;
  content: "";
  border-radius: inherit;
  background: linear-gradient(120deg, #38bdf8 0%, #2563eb 52%, #8b5cf6 100%);
  opacity: 0;
  transition: opacity 0.24s cubic-bezier(0.22, 1, 0.36, 1);
  -webkit-mask: linear-gradient(#000 0 0) content-box, linear-gradient(#000 0 0);
  -webkit-mask-composite: xor;
  mask-composite: exclude;
}

.provider-panel:hover::after {
  opacity: 1;
}

.provider-panel.is-dragging::after {
  opacity: 0;
}

/* 提交排序瞬间禁用过渡，防止 DOM 变更与 transform 动画叠加导致位置错误反弹 */
.provider-list.is-committing .provider-panel {
  transition: none !important;
}

/* 拖拽中的卡片高亮与层级提升 */
.provider-panel.is-dragging {
  z-index: 50 !important;
  box-shadow: 0 12px 28px rgba(15, 23, 42, 0.14) !important;
  border-color: #38bdf8 !important;
  cursor: grabbing !important;
  transition: none !important;
}

.drag-handle {
  display: flex;
  align-items: center;
  justify-content: center;
  color: #cbd5e1;
  cursor: grab;
  padding: 6px 3px;
  border-radius: 4px;
  touch-action: none;
  transition: color 0.15s ease, background 0.15s ease;
}

.drag-handle:hover {
  color: #64748b;
  background: rgba(15, 23, 42, 0.05);
}

.drag-handle:active {
  cursor: grabbing;
}

.is-spinning {
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
</style>
