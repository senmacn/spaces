<template>
  <div class="projects-index">
    <div class="projects-title">
      <project-title-tools @add-item="handleAddProjectItem"></project-title-tools>
    </div>
    <hr />
    <div class="projects-content">
      <keep-alive>
        <component :is="viewerRef === Viewer.Grid ? ProjectGrids : ProjectList"></component>
      </keep-alive>
    </div>
  </div>
  <project-item-add-modal ref="addModalRef"></project-item-add-modal>
  <project-item-show-modal ref="showModalRef"></project-item-show-modal>
</template>

<script setup lang="ts">
  import ProjectTitleTools from './children/project-title-tools.vue';
  import ProjectItemAddModal from './children/project-item-add-modal.vue';
  import ProjectItemShowModal from './children/project-item-show-modal.vue';
  import ProjectGrids from './project-grids.vue';
  import ProjectList from './project-list.vue';
  import { useToggle } from '@vueuse/core';
  import { onEditProjectEvent, onShowProjectEvent } from './common/event';

  enum Viewer {
    Grid,
    List,
  }

  const [viewerRef, toggleViewer] = useToggle(Viewer.Grid, {
    truthyValue: Viewer.Grid,
    falsyValue: Viewer.List,
  });

  const addModalRef = ref();
  function handleAddProjectItem() {
    addModalRef.value.setItem();
  }
  onEditProjectEvent((_, item) => {
    addModalRef.value.setItem(item);
  });

  const showModalRef = ref();
  onShowProjectEvent((_, item) => {
    showModalRef.value.setItem(item);
  });
</script>

<style lang="less" scoped>
  .projects-index {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    hr {
      margin: 8px;
      border: none;
      height: 1px;
      background-color: #f5f5f5;
    }
  }
  .projects-title {
    display: flex;
    flex-direction: row;
    justify-content: center;
    height: 42px;
    padding: 5px;
  }
  .projects-content {
    flex: 1;
    margin: 5px;
  }
</style>
