<template>
  <div class="project-grids">
    <template v-for="project in visibleProjects" :key="project.id">
      <project-grid-item :project="project"></project-grid-item>
    </template>
  </div>
</template>

<script setup lang="ts">
  import ProjectGridItem from './children/project-grid-item.vue';
  import { useProjectStore } from '@/store/modules/project-state';
  import { FilterType } from './common/types';

  const projectState = useProjectStore();
  const visibleProjects = computed(() => {
    let visibleProjects = projectState.getProjectItems;
    if (projectState.getSearch) {
      visibleProjects = projectState.getProjectItems.filter((p) =>
        p.name.includes(projectState.getSearch),
      );
    }
    switch (projectState.getFilter) {
      case FilterType.STAR:
        visibleProjects = visibleProjects.filter((v) => v.favorite);
        break;
      case FilterType.DELETED:
        visibleProjects = visibleProjects.filter((v) => !!v.deletedAt);
        break;
      case FilterType.TAG:
        break;
      case FilterType.ALL:
      default:
        break;
    }

    return visibleProjects;
  });
</script>

<style lang="less">
  .project-grids {
    display: flex;
    flex-wrap: wrap;
    > div {
      margin: 8px;
    }
  }
</style>
