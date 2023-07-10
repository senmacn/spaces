<template>
  <div class="projects-index">
    <div class="projects-title">
      <a-input></a-input>
    </div>
    <div class="projects-content">
      <keep-alive>
        <component :is="viewerRef === Viewer.Grid ? ProjectGrids : ProjectList"></component>
      </keep-alive>
    </div>
  </div>
</template>

<script setup lang="ts">
  import ProjectGrids from './project-grids.vue';
  import ProjectList from './project-list.vue';
  import { useToggle } from '@vueuse/core';

  enum Viewer {
    Grid,
    List,
  }

  const [viewerRef, toggleViewer] = useToggle(Viewer.Grid, {
    truthyValue: Viewer.Grid,
    falsyValue: Viewer.List,
  });

  const projectList: Ref<ProjectItem[]> = ref([
    { uuid: '', name: 'aaa', description: 'aaa' },
    { uuid: '', name: 'bbb', description: 'bbb' },
  ]);
  provide('projectList', projectList);
</script>

<style lang="less" scoped>
  .projects-index {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
  }
  .projects-title {
    display: flex;
    flex-direction: row;
    justify-content: center;
    height: 50px;
    padding: 5px;
  }
  .projects-content {
    flex: 1;
    margin: 5px;
  }
</style>
