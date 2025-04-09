<template>
  <div class="centered-container">
    <test-imgs
      :_canRecovery="true"
      :imgs="imgs"
      ref="testImgs"
    ></test-imgs>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import Item1 from "../assets/1-1.avif";
import Item2 from "../assets/1-2.avif";
import Item3 from "../assets/1-3.avif";
import Item4 from "../assets/1-4.avif";
import Item5 from "../assets/1-5.avif";
import { TestImgs } from '../components/webCom/test-imgs';

const testImgs = ref<TestImgs>();

const imgs = [
  { src: Item1, test: `Explore The World`, isOpen: true },
  { src: Item2, test: "Wild Forest", isOpen: false },
  { src: Item3, test: "Sunny Beach", isOpen: false },
  { src: Item4, test: "City on Winter", isOpen: false },
  { src: Item5, test: "Mountains - Clouds", isOpen: false },
];

onMounted(() => {
  document.body.style.backgroundColor = 'var(--background-color)';
});
</script>

<style scoped>
.centered-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 95vh;
}
</style>
