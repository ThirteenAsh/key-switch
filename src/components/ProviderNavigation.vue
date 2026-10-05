<script setup lang="ts">
import { nextTick, onUnmounted, ref, watch } from "vue";
import { useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { GripVertical } from "@lucide/vue";
import ProviderAvatar from "./ProviderAvatar.vue";
import { useDashboardStore } from "../stores/dashboard";
import { translateAppError } from "../i18n/errors";

const store = useDashboardStore();
const router = useRouter();
const { t } = useI18n();
const notice = ref("");
let noticeTimer: number | undefined;
function notify(message: string) {
  notice.value = message;
  window.clearTimeout(noticeTimer);
  noticeTimer = window.setTimeout(() => { notice.value = ""; }, 2800);
}
function selectProvider(id: string) {
  store.selectedProviderId = id;
  void router.push("/");
}
async function moveProvider(id: string, event: KeyboardEvent) {
  if (!event.altKey || !["ArrowUp", "ArrowDown"].includes(event.key)) return;
  event.preventDefault();
  const visibleIndex = store.providers.findIndex((provider) => provider.id === id);
  const target = store.providers[visibleIndex + (event.key === "ArrowUp" ? -1 : 1)];
  if (!target) return;
  try {
    await store.reorderProviders(store.providers.findIndex((provider) => provider.id === id), store.providers.findIndex((provider) => provider.id === target.id));
  } catch (error) { notify(translateAppError(error)); }
}
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
  if (event.button !== 0 || store.sorting || dragState.value) return;

  const handleElement = event.currentTarget;
  if (!(handleElement instanceof HTMLElement)) return;

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

  store.providers.forEach((provider, index) => {
    const panel = panels[index];
    if (panel) rects.set(provider.id, panel.getBoundingClientRect());
  });

  return rects;
}

function animateProviderDrop(previousRects: Map<string, DOMRect>, movedProviderId: string) {
  const reduceMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  const panels = getProviderPanels();

  if (!reduceMotion) {
    store.providers.forEach((provider, index) => {
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

  const fromProvider = store.providers[fromIndex];
  const toProvider = store.providers[currentIndex];
  if (!fromProvider || !toProvider) return;

  const fromRealIndex = store.providers.findIndex((provider) => provider.id === fromProvider.id);
  const toRealIndex = store.providers.findIndex((provider) => provider.id === toProvider.id);
  if (fromRealIndex === -1 || toRealIndex === -1) return;

  // 先记录松手时的视觉位置；DOM 换序后用 FLIP 从旧位置过渡到新槽位。
  const previousRects = captureProviderRects();
  isCommitting.value = true;
  void store.reorderProviders(fromRealIndex, toRealIndex).catch((error) => notify(translateAppError(error)));

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


watch(() => store.providers.map((provider) => provider.id).join(","), () => finishDrag(false));
onUnmounted(() => window.clearTimeout(noticeTimer));
</script>

<template>
  <div class="provider-navigation">
    <nav class="provider-navigation-scroll" :aria-label="t('navigation.providerList')" :aria-busy="store.loading || store.sorting">
      <p v-if="store.loading" class="sidebar-message" role="status">{{ t('dashboard.loading') }}</p>
      <p v-else-if="store.loadError" class="sidebar-message">{{ t('dashboard.loadFailed') }}</p>
      <p v-else-if="!store.providers.length" class="sidebar-message">{{ t('dashboard.empty.noProvidersTitle') }}</p>
      <div v-else ref="providerListRef" class="provider-nav-list" :class="{ 'is-committing': isCommitting }">
        <div v-for="(provider, index) in store.providers" :key="provider.id" class="provider-nav-row"
          :class="{ 'is-selected': $route.path === '/' && store.selectedProvider?.id === provider.id, 'is-dragging': dragState?.isActivated && dragState.fromIndex === index }" :style="getCardTransform(index)">
          <button type="button" class="provider-nav-button" :aria-current="$route.path === '/' && store.selectedProvider?.id === provider.id ? 'page' : undefined" :title="provider.name" @click="selectProvider(provider.id)">
            <ProviderAvatar :provider="provider" />
            <span class="provider-nav-name">{{ provider.name }}</span>
          </button>
          <button type="button" class="drag-handle" :disabled="store.sorting" :title="t('dashboard.reorderHelp')" :aria-label="t('dashboard.reorderProvider', { name: provider.name })"
            @pointerdown="handleHandlePointerDown(index, $event)" @keydown="moveProvider(provider.id, $event)">
            <GripVertical :size="15" aria-hidden="true" />
          </button>
        </div>
      </div>
    </nav>
    <Transition name="toast"><p v-if="notice" class="toast" role="status">{{ notice }}</p></Transition>
  </div>
</template>

<style scoped>
.provider-navigation { contain: inline-size; display: flex; flex: 1; min-height: 0; flex-direction: column; gap: 12px; }
.provider-navigation-scroll { flex: 1; min-height: 0; overflow-y: auto; overflow-x: hidden; scrollbar-width: thin; padding: 2px; }
.provider-nav-list { position: relative; display: grid; gap: 5px; }
.provider-nav-row { position: relative; display: flex; align-items: center; border: 1px solid transparent; border-radius: 10px; transition: transform .22s cubic-bezier(.16, 1, .3, 1), background .15s ease; }
.provider-nav-row:hover { background: var(--interactive-hover); }
.provider-nav-row.is-selected { background: var(--nav-active); color: var(--nav-active-text); }
.provider-nav-button { display: flex; flex: 1; min-width: 0; align-items: center; gap: 8px; min-height: 34px; padding: 6px 0 6px 9px; text-align: left; color: var(--text-primary); background: transparent; cursor: pointer; border-radius: 9px; }
.provider-nav-name { flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; font-size: 12px; font-weight: 500; }
.is-selected .provider-nav-name { font-weight: 600; }
.provider-nav-button :deep(.provider-avatar) { width: 20px; height: 20px; flex-basis: 20px; border-radius: 5px; }
.provider-nav-button :deep(.provider-avatar img:not(.provider-avatar__custom-image)) { width: 16px; height: 16px; }
.provider-nav-button :deep(.provider-abbr) { font-size: 10px; }
.drag-handle { display: grid; place-items: center; width: 25px; align-self: stretch; border-radius: 6px; color: var(--text-subtle); background: transparent; cursor: grab; touch-action: none; }
.drag-handle:hover { color: var(--text-primary); }
.drag-handle:disabled { cursor: wait; opacity: .4; }
.provider-nav-row.is-dragging { z-index: 50; background: var(--surface-sidebar); border-color: var(--border-strong); box-shadow: 0 10px 24px rgba(15, 23, 42, .14); transition: none; }
.is-committing .provider-nav-row { transition: none; }
.sidebar-message { padding: 12px 7px; margin: 0; color: var(--text-muted); font-size: 12px; line-height: 1.7; }
</style>
