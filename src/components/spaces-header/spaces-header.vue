<template>
  <div class="spaces-header">
    <div class="header-control">
      <a-button type="text">
        <question-circle-outlined />
      </a-button>
      <a-dropdown trigger="click">
        <a-button type="text">
          <a-badge>
            <bell-outlined />
          </a-badge>
        </a-button>
        <template #overlay>
          <a-menu></a-menu>
        </template>
      </a-dropdown>
      <a-button type="text">
        <template #icon>
          <user-outlined></user-outlined>
        </template>
        Sign In
      </a-button>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { BellOutlined, UserOutlined, QuestionCircleOutlined } from '@ant-design/icons-vue';
</script>

<style lang="less">
  .spaces-header {
    position: fixed;
    top: 0;
    right: 140px;
    user-select: none;
    line-height: 42px;
    z-index: 1003;
    .header-control {
      position: relative;
      height: 42px;
      display: flex;
      flex-direction: row-reverse;
      align-items: center;
      .ant-badge {
        margin: 0 7px;
        cursor: pointer;
      }
    }
  }
</style>
