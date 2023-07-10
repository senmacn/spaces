<template>
  <a-row :gutter="8">
    <a-col>
      <a-card class="grid-item"> + </a-card>
    </a-col>
    <a-col v-for="project in projectList" :key="project.uuid">
      <project-grid-item :project="project"></project-grid-item>
    </a-col>
  </a-row>
</template>

<script setup lang="ts">
  import ProjectGridItem from './children/project-grid-item.vue';

  const projectList: ProjectItem[] = inject('projectList', []);
</script>

<style></style>
