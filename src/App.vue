<template>
  <a-config-provider :locale="zhCN">
    <a-layout class="spaces">
      <a-layout-header>
        <spaces-header></spaces-header>
      </a-layout-header>
      <a-layout>
        <a-layout-sider width="160px">
          <spaces-sider></spaces-sider>
        </a-layout-sider>
        <a-layout-content>
          <router-view />
        </a-layout-content>
      </a-layout>
    </a-layout>
  </a-config-provider>
</template>

<script setup lang="ts">
  import SpacesHeader from '@/components/spaces-header/spaces-header.vue';
  import SpacesSider from '@/components/spaces-sider/spaces-sider.vue';
  import zhCN from 'ant-design-vue/es/locale/zh_CN';
</script>

<style lang="less" scoped>
  .spaces {
    height: 100vh;
    width: 100vw;
  }
</style>
