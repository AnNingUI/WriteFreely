<template>
  <label :class="['switch', { 'switch--open': isOpen, 'switch--closed': isClosed }]">
    <input type="checkbox" :checked="isOpen" @change="handleChange" />
    <span class="slider"></span>
  </label>
</template>

<script setup lang="ts">
import { defineEmits, defineProps } from 'vue';

// 定义props
defineProps({
  isOpen: Boolean,  // 组件当前的状态（开关）
  isClosed: Boolean // 是否处于关闭状态（你可以将它与isOpen反转使用）
});

// 定义emits
const emit = defineEmits(['onChange', 'change']);

// 处理开关的改变
const handleChange = (event: Event) => {
  const isSwitchOpen = (event.target as HTMLInputElement).checked;
  const isSwitchClosed = !isSwitchOpen;

  // 触发自定义事件，并将状态和原始事件传递出去
  emit('onChange', {
    event,
    isOpen: isSwitchOpen,
    isClosed: isSwitchClosed,
  });

  emit('change', {
    event,
    isOpen: isSwitchOpen,
    isClosed: isSwitchClosed,
  });
};
</script>

<style scoped>
.switch {
  display: inline-flex;
  align-items: center;
  cursor: pointer;
  position: relative;
}

input[type="checkbox"] {
  display: none; /* 隐藏原始复选框 */
}

.slider {
  width: 50px;
  height: 24px;
  background-color: #ccc;
  border-radius: 12px;
  transition: background-color 0.3s;
  position: relative;
}

.slider::before {
  content: '';
  position: absolute;
  width: 20px;
  height: 20px;
  background-color: white;
  border-radius: 50%;
  top: 2px;
  left: 2px;
  transition: transform 0.3s;
}

.switch--open .slider {
  background-color: #4caf50; /* 打开时的背景颜色 */
}

.switch--open .slider::before {
  transform: translateX(26px); /* 打开时滑块移动 */
}

.switch--closed .slider {
  background-color: #ccc; /* 关闭时的背景颜色 */
}

.switch--closed .slider::before {
  transform: translateX(0); /* 关闭时滑块回到起始位置 */
}
</style>

