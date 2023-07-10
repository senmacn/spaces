<template>
  <a-card class="grid-item">
    <template #actions>
      <heart-outlined></heart-outlined>
      <setting-outlined />
      <ellipsis-outlined />
    </template>
    <a-card-meta :title="project.name" :description="project.description">
      <template #avatar>
        <a-avatar src="https://joeschmoe.io/api/v1/random" />
      </template>
    </a-card-meta>
  </a-card>
</template>

<script setup lang="ts">
  import { HeartOutlined, SettingOutlined, EllipsisOutlined } from '@ant-design/icons-vue';

  const props = defineProps({
    project: {
      type: Object as PropType<ProjectItem>,
      default: () => ({} as ProjectItem),
    },
  });
</script>

<style lang="less">
  .grid-item {
    width: 200px;
    height: 200px;
  }
</style>
