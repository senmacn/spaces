<template>
  <a-modal
    class="project-item-add-modal"
    :title="null"
    :width="500"
    :open="visibleRef"
    :footer="null"
    @cancel="handleClose"
  >
    <a-spin :spinning="spinningRef">
      <div class="project-item-content">
        <a-descriptions :title="projectItemRef.name" layout="horizontal" :column="1">
          <a-descriptions-item label="描述">{{ projectItemRef.description }}</a-descriptions-item>
          <a-descriptions-item label="路径">{{ projectItemRef.path }}</a-descriptions-item>
          <a-descriptions-item label="标签">{{ projectItemRef.tags }}</a-descriptions-item>
          <template #extra>
            <a-button type="text" @click="handleEditProjectItem">修改</a-button>
          </template>
        </a-descriptions>
        <scheme-list :project="projectItemRef"></scheme-list>
      </div>
    </a-spin>
  </a-modal>
</template>

<script setup lang="ts">
  import { emitEditProjectEvent } from '../common/event';
  import schemeList from './scheme-list.vue';

  const projectItemRef: Ref<ProjectItem> = ref({} as ProjectItem);

  const spinningRef = ref(false);
  const visibleRef = ref(false);
  function setItem(item: ProjectItem) {
    visibleRef.value = true;
    projectItemRef.value = item;
  }
  defineExpose({
    setItem,
  });

  function handleEditProjectItem() {
    emitEditProjectEvent(projectItemRef.value);
  }

  function handleClose() {
    spinningRef.value = false;
    visibleRef.value = false;
  }
</script>

<style lang="less">
  .project-item-add-modal {
    .ant-modal-body {
      padding: 30px;
    }
    .project-item-content {
    }
    .ant-descriptions-item-container > span {
      font-size: 12px;
    }
  }
</style>
