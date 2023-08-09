<template>
  <div class="spaces-wrapper">
    <spaces-header></spaces-header>
    <a-layout class="spaces glass-container">
      <a-layout-sider class="app-sider">
        <spaces-sider></spaces-sider>
      </a-layout-sider>
      <a-layout-content class="app-content">
        <router-view />
      </a-layout-content>
    </a-layout>
  </div>
</template>

<script setup lang="ts">
  import SpacesHeader from '@/components/spaces-header/spaces-header.vue';
  import SpacesSider from '@/components/spaces-sider/spaces-sider.vue';
</script>

<style lang="less" scoped>
  .spaces-wrapper {
    height: 100vh;
  }
  .glass-container {
    backdrop-filter: blur(4px);
  }
  .app-sider {
    padding: 10px 15px;
    width: auto !important;
    flex-basis: auto !important;
    max-width: none !important;
    min-width: 50px !important;
  }
  .app-header {
    height: 42px;
    line-height: 42px;
    padding-inline: 0;
  }
  .app-content {
    margin: 3px 6px 6px 3px;
    padding-top: 42px;
    border-radius: 6px;
    border: 1px solid rgba(0, 0, 0, 0.05);
    box-shadow: 0 10px 8px rgba(0, 0, 0, 0.05);
  }
  .ant-layout,
  .ant-layout-header,
  .ant-layout-content,
  .ant-layout-sider {
    background-color: transparent;
  }
</style>
