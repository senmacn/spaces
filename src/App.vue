<template>
  <a-config-provider
    :locale="zhCN"
    :theme="{
      algorithm: theme.darkAlgorithm,
    }"
  >
  <router-view v-slot="{ Component }">
  <transition name="starting">
    <component :is="Component" />
  </transition>
</router-view>
  </a-config-provider>
</template>

<script setup lang="ts">
  import zhCN from 'ant-design-vue/es/locale/zh_CN';
  import { theme } from 'ant-design-vue';
  import TauriPersist from '@/lib/tauri-persist';

  TauriPersist.getInstance().init();
</script>

<style lang="less">
  .starting-enter-active {
    transition: all 0.2s ease-out;
  }

  .starting-leave-active {
    transition: all 0.2s cubic-bezier(1, 0.5, 0.8, 1);
  }

  .starting-enter-from,
  .starting-leave-to {
    transform: translateX(20px);
    opacity: 0;
  }
</style>
