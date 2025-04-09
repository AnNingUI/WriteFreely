<template>
  <div>
    <RouterView></RouterView>
    <!-- <css100Love/> -->
  </div>
</template>

<script setup lang="ts">
import { RouterView } from 'vue-router';
</script>

<style>
* {
  -webkit-user-select: none;
  user-select: none;
  -webkit-tap-highlight-color: rgba(0, 0, 0, 0);
}
</style>
