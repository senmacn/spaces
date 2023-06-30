<template>
  <div class="projects-index">
    <div class="projects-title"> aaa </div>
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
</script>

<style lang="less" scoped>
  .projects-index {
    width: 100%;
    height: 100%;
  }
  .projects-index {
    display: flex;
    flex-direction: row;
    justify-content: center;
    height: 100px;
  }
</style>
