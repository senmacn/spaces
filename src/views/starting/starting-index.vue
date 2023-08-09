<template>
  <div class="starting-wrapper">
    <div class="loading">Spaces初始化</div>
  </div>
</template>

<script setup lang="ts">
  import { window } from '@tauri-apps/api';
  import { useRouter } from 'vue-router';

  const router = useRouter();
  onMounted(() => {
    setTimeout(() => {
      router.push('/');
      window
        .getCurrent()
        .innerSize()
        .then((size) => {
          size.width = 800;
          size.height = 600;
          window.getCurrent().setSize(size);
          window.getCurrent().setMinSize(size);
        });
    }, 1000);
  });
</script>

<style lang="less">
  .starting-wrapper {
    margin-top: 72px;
    text-align: center;
  }
  .loading {
    display: inline-block;
    font-size: 28px;
    font-family: Arial, Helvetica, sans-serif;
    font-weight: bold;
    color: rgb(120, 120, 120);
    letter-spacing: 2px;
    position: relative;
    user-select: none;
  }

  .loading::after {
    content: 'Spaces初始化';
    position: absolute;
    left: 1px;
    top: 0;
    color: #fed128;
    width: 100%;
    height: 100%;
    overflow: hidden;
    white-space: nowrap;
    animation: loading-animation 2s linear infinite;
  }

  @keyframes loading-animation {
    0% {
      width: 0%;
    }
    100% {
      width: 100%;
    }
  }
</style>
