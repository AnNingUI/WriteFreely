<template>
  <div class="home">
    <h1 class="title">
      <div>
        安宁的随心写
        <div class="AnNingUI">
          <div class="textImg-text">
            <div class="text"></div>
            <img class="textImg-img" :src="AnNingUI"></img>
            <div class="text"></div>
          </div>
        </div>
      </div>
      <div :class="isMobile">
        <button @click="toggleTheme" class="toggle-button">
          <div class="toggle-button-text">{{ isDarkTheme ? '🌙' : '☀️' }}</div>
          <div :class="{ 'toggle-button-icon': isDarkTheme, 'toggle-button-icon-dark': !isDarkTheme }">
            <img :src="isDarkTheme ? sun : moon" alt="切换主题" class="toggle-icon"
              style="width: 20px; height: 20px; justify-content: center; align-items: center; display: flex;" />
          </div>
        </button>
      </div>
    </h1>
    <div class="grid-container">
      <div v-for="item in items" :key="item.id" class="card">
        <router-link :to="item.routePath" class="card-link" aria-label="Go to {{ item.name }}">
          <img v-if="item.imgUrl" :src="item.imageSrc" alt="项目图片" @error="onImageError" class="card-image" />
          <div v-else class="default-image"></div>
          <h2 class="card-name">{{ item.name }}</h2>
          <p class="card-description">{{ item.described }}</p>
        </router-link>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from 'vue';

import MobileDetect from 'mobile-detect';

import sun from '../assets/sun.png';

import moon from '../assets/moon.png';

import AnNingUI from '../assets/&0.svg'

const md = new MobileDetect(window.navigator.userAgent);

const isMobile = ref(md.mobile() && window.innerWidth < 520 ? "theme-toggle mobile" : "theme-toggle");

const domRef = ref(0);

const items = ref([
  {
    routePath: "/Css100Love",
    described: "这是B站Up青云杂货铺的css100天挑战第13天项目",
    name: "Css 100 Love",
    id: "1",
    imgUrl: "css100love.jpg", // 本地图片名称
    imageSrc: "", // 初始化为空
  },
  {
    routePath: "/NBody",
    described: "这是我做的一个天体运动的简易模型",
    name: "NBody in Space",
    id: "2",
    imgUrl: "NBody.jpg", // 远程图片 URL
    imageSrc: "", // 初始化为空
  },
  {
    routePath: "/RustNBody",
    described: "这是我做的一个天体运动的简易模型(但计算通过rust)",
    name: "NBody in Space (by rust)",
    id: "3",
    imgUrl: "NBody.jpg", // 远程图片 URL
    imageSrc: "", // 初始化为空
  },
  {
    routePath: "/PixiTest",
    described: "...",
    name: "...",
    id: "4",
    imgUrl: "NBody.jpg", // 远程图片 URL
    imageSrc: "", // 初始化为空
  },
  {
    routePath: "/VueWebC",
    described: "这是我的Vue结合Web Component的尝试项目",
    name: "Vue WebComponent",
    id: "5",
    imgUrl: "1-1.avif", // 远程图片 URL
    imageSrc: "", // 初始化为空
  }
  // 添加更多项目...
]);

interface ImgItem {
  routePath: string;
  described: string;
  name: string;
  id: string;
  imgUrl: string;
  imageSrc: string;
}
const loadImage = async (item: ImgItem) => {
  if (item.imgUrl && !item.imgUrl.startsWith("http")) {
    const lastDotIndex = item.imgUrl.lastIndexOf('.');
    const result = item.imgUrl.substring(0, lastDotIndex);
    const extension = item.imgUrl.split('.').pop();
    item.imageSrc = (await import(`../assets/${result}.${extension}`)).default;
  } else {
    item.imageSrc = item.imgUrl; // 直接使用远程图片 URL
  }
};

const changeButtonClass = () => {
  domRef.value = window.innerWidth;

  isMobile.value = md.mobile() && window.innerWidth < 520 ? "mobile theme-toggle" : "theme-toggle"
}

onMounted(() => {
  isMobile.value = md.mobile() && window.innerWidth < 520 ? "mobile theme-toggle" : "theme-toggle"
  document.body.style.backgroundColor = 'var(--background-color)';
  window.addEventListener('resize', changeButtonClass);
});

onBeforeUnmount(() => {
  document.body.style.backgroundColor = ''; // 清除样式
});



const onImageError = (event: Event) => {
  const target = event.target as HTMLImageElement;
  target.src = ""; // 清空图片以显示默认图像
};


const isDarkTheme = ref(false);

const toggleTheme = (e: MouseEvent) => {
  isDarkTheme.value = !isDarkTheme.value;
  const transition = document.startViewTransition(() => {
    document.body.classList.toggle('dark-theme', isDarkTheme.value);
    document.body.classList.toggle('light-theme', !isDarkTheme.value);
  })

  const _anfun = () => {
    const { clientX: x, clientY: y } = e;

    const tragetRadius = Math.hypot(
      Math.max(x, window.innerWidth - x),
      Math.max(y, window.innerHeight - y)
    );

    transition.ready.then(() => {
      document.documentElement.animate(
        {
          clipPath: [`circle(0% at ${x}px ${y}px)`, `circle(${tragetRadius}px at ${x}px ${y}px)`],
        }, {
        duration: 400,
        pseudoElement: '::view-transition-new(root)',
      }
      )
    });
  }

  if (!md.mobile()) {
    _anfun();
  }
};

onMounted(async () => {
  for (const item of items.value) {
    await loadImage(item);
  }
});

// createElementVNode
// h

</script>

<style>
@import url(../assets/css/Home.css);

::view-transition-new(root),
::view-transition-old(root) {
  animation: none;
}

a {
  background: rgba(77, 10, 10, 0.104);
}
</style>


<!-- 太阳 月亮 -->
