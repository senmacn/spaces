<template>
  <div class="home">
    <div class="project-dashboard-wrapper">
      <project-dashboard />
    </div>
    <div class="latest-used-projects-wrapper">
      <latest-used-projects />
    </div>
  </div>
</template>

<script setup lang="ts">
  import ProjectDashboard from './children/project-dashboard.vue';
  import LatestUsedProjects from './children/latest-used-projects.vue';
</script>

<style lang="less">
  .home {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    .project-dashboard-wrapper {
      flex: 1;
    }
    .latest-used-projects-wrapper {
    }
  }
</style>
