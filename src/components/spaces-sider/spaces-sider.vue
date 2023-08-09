<template>
  <div class="space-sider">
    <div class="title neon"> SPACES </div>
    <a-menu>
      <a-menu-item
        key="1"
        :class="[activeMenuIteRef === MenuItem.DashBoard && 'ant-menu-item-selected']"
        @click="activeMenuIteRef = MenuItem.DashBoard"
      >
        <dashboard-outlined />
        <router-link to="/home">面板</router-link>
      </a-menu-item>
      <a-menu-item
        key="2"
        :class="[activeMenuIteRef === MenuItem.Application && 'ant-menu-item-selected']"
        @click="activeMenuIteRef = MenuItem.Application"
      >
        <appstore-outlined />
        <router-link to="/projects">应用</router-link>
      </a-menu-item>
      <a-menu-item
        key="3"
        :class="[activeMenuIteRef === MenuItem.Setting && 'ant-menu-item-selected']"
        @click="activeMenuIteRef = MenuItem.Setting"
      >
        <setting-outlined />
        <router-link to="/setting">配置</router-link>
      </a-menu-item>
    </a-menu>
  </div>
</template>

<script setup lang="ts">
  import { DashboardOutlined, AppstoreOutlined, SettingOutlined } from '@ant-design/icons-vue';

  enum MenuItem {
    DashBoard = 'home',
    Application = 'projects',
    Setting = 'setting',
  }
  const activeMenuIteRef = ref(MenuItem.Application);
  onMounted(() => {
    const hash = location.hash;
    if (hash.includes(MenuItem.DashBoard)) {
      activeMenuIteRef.value = MenuItem.DashBoard;
    }
    if (hash.includes(MenuItem.Application)) {
      activeMenuIteRef.value = MenuItem.Application;
    }
    if (hash.includes(MenuItem.Setting)) {
      activeMenuIteRef.value = MenuItem.Setting;
    }
  });
</script>

<style lang="less">
  .space-sider {
    width: 130px;
    .title {
      padding: 18px;
      text-align: center;
      font-size: 24px;
      user-select: none;
    }
    .neon {
      text-shadow: 0 0 3vw white;
      animation: neon 3s ease-in-out infinite;
    }
    @keyframes neon {
      0%,
      100% {
        text-shadow: 0 0 1vw #fa1c16, 0 0 3vw #fa1c16, 0 0 10vw #fa1c16, 0 0 10vw #fa1c16,
          0 0 0.4vw #fed128, 0.5vw 0.5vw 0.1vw #806914;
        color: #fffc00;
      }
      50% {
        text-shadow: 0 0 0.5vw #800e0b, 0 0 1.5vw #800e0b, 0 0 5vw #800e0b, 0 0 5vw #800e0b,
          0 0 0.2vw #800e0b, 0.5vw 0.5vw 0.1vw #40340a;
        color: #806914;
      }
    }
    .ant-menu {
      border-right: 0;
      background: transparent;
    }
    .ant-menu-item {
      text-align: center;
      border-radius: 8px;
      .anticon {
        display: inline-flex;
        border-radius: 6px;
        justify-content: center;
        align-items: center;
        vertical-align: middle;
        font-size: 16px;
      }
      a {
        padding: 10px 16px;
      }
    }
    .ant-menu-item-selected {
      box-shadow: 0 20px 27px rgba(0, 0, 0, 0.05);
      a {
        color: rgb(22, 119, 255);
      }
    }
  }
</style>
