<template>
  <div> 最近使用 </div>
  <div class="project-grids latest-used-projects">
    <template v-for="project in visibleProjects" :key="project.id">
      <a-card class="grid-item">
        <a-card-meta
          :title="project.name"
          :description="project.description"
          @dblclick="handleOpenByDefault(project)"
        >
          <template #avatar>
            <a-avatar v-if="project.icon" :src="`src/assets/images/${project.icon}.png`"></a-avatar>
            <a-avatar v-else :alt="project.name"></a-avatar>
          </template>
        </a-card-meta>
      </a-card>
    </template>
  </div>
</template>

<script setup lang="ts">
  import { tauriExecuteCommand } from '@/lib/tauri-handler';
  import TauriPersist from '@/lib/tauri-persist';
  import { useProjectStore } from '@/store/modules/project-state';
  import { message } from 'ant-design-vue';

  const projectState = useProjectStore();
  const visibleProjects = computed(() => {
    return projectState.getProjectItems
      .slice()
      .sort((a, b) => (Number(a.usedAt) < Number(b.usedAt) ? 1 : -1))
      .slice(0, 3);
  });

  const persist = TauriPersist.getInstance();
  async function handleOpenByDefault(project) {
    if (!project.defaultScheme) {
      message.warning('未找到默认启动配置！');
      return;
    }
    try {
      const defaultScheme = await persist.getSchemeById(project.defaultScheme);
      await tauriExecuteCommand(defaultScheme.program, project);
    } catch (e: any) {
      message.error(e.message);
    }
  }
</script>

<style lang="less">
  .latest-used-projects {
    .ant-card {
      height: auto;
    }
  }
</style>
