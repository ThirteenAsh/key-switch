<script setup lang="ts">
import {
  computed,
  nextTick,
  onBeforeUnmount,
  onMounted,
  ref,
  useId,
  watch,
} from "vue";
import { Check, ChevronDown } from "@lucide/vue";

export interface AppSelectOption {
  value: string;
  label: string;
}

const props = defineProps<{
  modelValue: string;
  options: AppSelectOption[];
  label: string;
  fullWidth?: boolean;
  placement?: "top" | "bottom";
}>();

const emit = defineEmits<{
  "update:modelValue": [value: string];
}>();

const root = ref<HTMLElement | null>(null);
const trigger = ref<HTMLButtonElement | null>(null);
const open = ref(false);
const activeIndex = ref(-1);
const componentId = `app-select-${useId().replace(/:/g, "")}`;
const listboxId = `${componentId}-listbox`;

const selectedIndex = computed(() => (
  props.options.findIndex((option) => option.value === props.modelValue)
));
const selectedOption = computed(() => props.options[selectedIndex.value]);
const activeOptionId = computed(() => (
  open.value && activeIndex.value >= 0 ? optionId(activeIndex.value) : undefined
));

function optionId(index: number): string {
  return `${componentId}-option-${index}`;
}

function openMenu(preferredIndex = selectedIndex.value): void {
  if (props.options.length === 0) return;
  activeIndex.value = preferredIndex >= 0 ? preferredIndex : 0;
  open.value = true;
}

function closeMenu(restoreFocus = false): void {
  open.value = false;
  if (restoreFocus) void nextTick(() => trigger.value?.focus());
}

function toggleMenu(): void {
  if (open.value) closeMenu();
  else openMenu();
}

function moveActive(delta: number): void {
  const count = props.options.length;
  if (count === 0) return;
  const current = activeIndex.value >= 0 ? activeIndex.value : selectedIndex.value;
  activeIndex.value = (current + delta + count) % count;
}

function selectIndex(index: number): void {
  const option = props.options[index];
  if (!option) return;
  emit("update:modelValue", option.value);
  closeMenu(true);
}

function selectByPrefix(prefix: string): void {
  const normalizedPrefix = prefix.toLocaleLowerCase();
  const start = activeIndex.value >= 0 ? activeIndex.value + 1 : 0;
  const orderedOptions = [
    ...props.options.slice(start),
    ...props.options.slice(0, start),
  ];
  const match = orderedOptions.find((option) => (
    option.label.toLocaleLowerCase().startsWith(normalizedPrefix)
  ));
  if (!match) return;
  activeIndex.value = props.options.indexOf(match);
  if (!open.value) openMenu(activeIndex.value);
}

function handleKeydown(event: KeyboardEvent): void {
  switch (event.key) {
    case "ArrowDown":
      event.preventDefault();
      if (open.value) moveActive(1);
      else openMenu(selectedIndex.value >= 0 ? selectedIndex.value : 0);
      break;
    case "ArrowUp":
      event.preventDefault();
      if (open.value) moveActive(-1);
      else openMenu(selectedIndex.value >= 0 ? selectedIndex.value : props.options.length - 1);
      break;
    case "Home":
      if (!open.value) return;
      event.preventDefault();
      activeIndex.value = 0;
      break;
    case "End":
      if (!open.value) return;
      event.preventDefault();
      activeIndex.value = props.options.length - 1;
      break;
    case "Enter":
    case " ":
      event.preventDefault();
      if (open.value) selectIndex(activeIndex.value);
      else openMenu();
      break;
    case "Escape":
      if (!open.value) return;
      event.preventDefault();
      closeMenu(true);
      break;
    case "Tab":
      closeMenu();
      break;
    default:
      if (event.key.length === 1 && !event.ctrlKey && !event.metaKey && !event.altKey) {
        selectByPrefix(event.key);
      }
  }
}

function handleDocumentPointerDown(event: PointerEvent): void {
  if (root.value?.contains(event.target as Node)) return;
  closeMenu();
}

watch(() => props.modelValue, () => {
  if (!open.value) activeIndex.value = selectedIndex.value;
});

watch(() => props.options.length, (length) => {
  if (length === 0) closeMenu();
  else if (activeIndex.value >= length) activeIndex.value = length - 1;
});

onMounted(() => {
  document.addEventListener("pointerdown", handleDocumentPointerDown, true);
});

onBeforeUnmount(() => {
  document.removeEventListener("pointerdown", handleDocumentPointerDown, true);
});
</script>

<template>
  <div ref="root" class="app-select" :class="{ 'is-open': open, 'app-select--full': fullWidth, 'app-select--top': placement === 'top' }">
    <button
      ref="trigger"
      class="app-select__trigger"
      type="button"
      role="combobox"
      aria-haspopup="listbox"
      :aria-label="label"
      :aria-expanded="open"
      :aria-controls="listboxId"
      :aria-activedescendant="activeOptionId"
      @click="toggleMenu"
      @keydown="handleKeydown"
    >
      <span class="app-select__value-viewport">
        <Transition name="app-select-value" mode="out-in">
          <span :key="modelValue" class="app-select__value">
            {{ selectedOption?.label ?? "" }}
          </span>
        </Transition>
      </span>
      <ChevronDown class="app-select__chevron" :size="16" :stroke-width="2" aria-hidden="true" />
    </button>

    <Transition name="app-select-menu">
      <div v-if="open" class="app-select__popover">
        <ul :id="listboxId" class="app-select__menu" role="listbox" :aria-label="label">
          <li
            v-for="(option, index) in options"
            :id="optionId(index)"
            :key="option.value"
            class="app-select__option"
            :class="{
              'is-active': activeIndex === index,
              'is-selected': modelValue === option.value,
            }"
            role="option"
            :aria-selected="modelValue === option.value"
            @pointerenter="activeIndex = index"
            @pointerdown.prevent
            @click="selectIndex(index)"
          >
            <span class="app-select__check" aria-hidden="true">
              <Transition name="app-select-check">
                <Check v-if="modelValue === option.value" :size="14" :stroke-width="2.4" />
              </Transition>
            </span>
            <span class="app-select__option-label">{{ option.label }}</span>
          </li>
        </ul>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.app-select {
  position: relative;
  width: 190px;
  flex: 0 0 190px;
}

.app-select--full {
  width: 100%;
  flex-basis: auto;
}

.app-select__trigger {
  display: flex;
  align-items: center;
  width: 100%;
  height: 38px;
  padding: 0 11px 0 13px;
  color: #334155;
  font-family: inherit;
  font-size: 13px;
  font-weight: 500;
  text-align: left;
  border: 1px solid #cbd5e1;
  border-radius: 10px;
  outline: none;
  background: rgba(255, 255, 255, 0.84);
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.75);
  cursor: pointer;
  user-select: none;
  transition:
    color 0.16s ease,
    border-color 0.18s ease,
    background 0.18s ease,
    box-shadow 0.2s ease,
    transform 0.18s cubic-bezier(0.16, 1, 0.3, 1);
}

.app-select__trigger:hover {
  color: #0f172a;
  border-color: #94a3b8;
  background: #ffffff;
  transform: translateY(-1px);
}

.app-select__trigger:active {
  transform: translateY(0) scale(0.99);
}

.app-select__trigger:focus-visible,
.app-select.is-open .app-select__trigger {
  color: #0f172a;
  border-color: #60a5fa;
  background: #ffffff;
  box-shadow:
    0 0 0 3px rgba(59, 130, 246, 0.13),
    0 4px 12px rgba(15, 23, 42, 0.06),
    inset 0 1px 0 rgba(255, 255, 255, 0.9);
}

.app-select.is-open .app-select__trigger {
  transform: translateY(-1px);
}

.app-select__value-viewport {
  display: flex;
  min-width: 0;
  height: 20px;
  flex: 1;
  align-items: center;
  overflow: hidden;
}

.app-select__value {
  display: block;
  max-width: 100%;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.app-select__chevron {
  flex: 0 0 auto;
  margin-left: 10px;
  color: #64748b;
  transition:
    color 0.16s ease,
    transform 0.22s cubic-bezier(0.16, 1, 0.3, 1);
}

.app-select.is-open .app-select__chevron {
  color: #2563eb;
  transform: rotate(180deg);
}

.app-select__popover {
  position: absolute;
  z-index: 50;
  top: calc(100% + 7px);
  right: 0;
  width: 100%;
  padding: 6px;
  border: 1px solid rgba(203, 213, 225, 0.9);
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.97);
  box-shadow:
    0 18px 42px -16px rgba(15, 23, 42, 0.28),
    0 7px 18px -10px rgba(15, 23, 42, 0.18),
    inset 0 1px 0 rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(16px);
  transform-origin: top right;
  box-sizing: border-box;
}

.app-select--top .app-select__popover {
  top: auto;
  bottom: calc(100% + 7px);
  transform-origin: bottom right;
}

.app-select__menu {
  display: grid;
  gap: 2px;
  margin: 0;
  padding: 0;
  list-style: none;
}

.app-select__option {
  display: grid;
  grid-template-columns: 16px minmax(0, 1fr);
  align-items: center;
  min-height: 36px;
  gap: 7px;
  padding: 0 9px;
  color: #475569;
  font-size: 13px;
  font-weight: 500;
  border-radius: 8px;
  cursor: pointer;
  user-select: none;
  transition:
    color 0.14s ease,
    background 0.16s ease,
    transform 0.16s cubic-bezier(0.16, 1, 0.3, 1);
}

.app-select__option.is-active {
  color: #0f172a;
  background: rgba(15, 23, 42, 0.055);
  transform: translateX(1px);
}

.app-select__option.is-selected {
  color: #1d4ed8;
  background: rgba(59, 130, 246, 0.09);
  font-weight: 600;
}

.app-select__option.is-active.is-selected {
  background: rgba(59, 130, 246, 0.14);
}

.app-select__check {
  position: relative;
  display: grid;
  width: 16px;
  height: 16px;
  color: #2563eb;
  place-items: center;
}

.app-select__option-label {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.app-select-menu-enter-active {
  transition:
    opacity 0.16s ease-out,
    transform 0.22s cubic-bezier(0.16, 1, 0.3, 1);
}

.app-select-menu-leave-active {
  transition:
    opacity 0.12s ease-in,
    transform 0.16s ease-in;
}

.app-select-menu-enter-from,
.app-select-menu-leave-to {
  opacity: 0;
  transform: translateY(-7px) scale(0.975);
}

.app-select--top .app-select-menu-enter-from,
.app-select--top .app-select-menu-leave-to {
  transform: translateY(7px) scale(0.975);
}

.app-select-value-enter-active,
.app-select-value-leave-active {
  transition:
    opacity 0.12s ease,
    transform 0.16s cubic-bezier(0.16, 1, 0.3, 1);
}

.app-select-value-enter-from {
  opacity: 0;
  transform: translateY(4px);
}

.app-select-value-leave-to {
  opacity: 0;
  transform: translateY(-3px);
}

.app-select-check-enter-active,
.app-select-check-leave-active {
  transition:
    opacity 0.12s ease,
    transform 0.16s cubic-bezier(0.16, 1, 0.3, 1);
}

.app-select-check-enter-from,
.app-select-check-leave-to {
  opacity: 0;
  transform: scale(0.55);
}

@media (prefers-reduced-motion: reduce) {
  .app-select__trigger,
  .app-select__chevron,
  .app-select__option,
  .app-select-menu-enter-active,
  .app-select-menu-leave-active,
  .app-select-value-enter-active,
  .app-select-value-leave-active,
  .app-select-check-enter-active,
  .app-select-check-leave-active {
    transition-duration: 0.01ms;
  }
}
</style>
