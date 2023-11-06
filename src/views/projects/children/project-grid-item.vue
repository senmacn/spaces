<template>
  <a-card class="grid-item">
    <template #actions>
      <a-tooltip title="默认启动">
        <play-circle-outlined @click.stop="handleOpenByDefault" />
      </a-tooltip>
      <a-tooltip title="星标">
        <star-filled
          v-if="Number(project.favorite)"
          class="active-item"
          @click.stop="handleSetStar(false)"
        ></star-filled>
        <star-outlined v-else @click.stop="handleSetStar(true)"></star-outlined>
      </a-tooltip>
      <a-dropdown trigger="click">
        <ellipsis-outlined @click.stop />
        <template #overlay>
          <a-menu>
            <a-menu-item>
              <a-button type="text" @click="handleOpenProjectDirectory">打开目录</a-button>
            </a-menu-item>
            <a-menu-item>
              <a-button type="text" @click="handleEditProjectItem">编辑</a-button>
            </a-menu-item>
            <a-menu-item>
              <a-button type="text" @click="() => {}">提醒</a-button>
            </a-menu-item>
            <a-menu-item>
              <a-button type="text" @click="handleDeleteProjectItem">删除</a-button>
            </a-menu-item>
          </a-menu>
        </template>
      </a-dropdown>
    </template>
    <a-card-meta
      :title="project.name"
      :description="project.description"
      @dblclick="handleShowProject"
    >
      <template #avatar>
        <a-avatar v-if="project.icon" :src="`src/assets/images/${project.icon}.png`"></a-avatar>
        <a-avatar v-else :alt="project.name"></a-avatar>
      </template>
    </a-card-meta>
  </a-card>
</template>

<script setup lang="ts">
  import TauriPersist from '@/lib/tauri-persist';
  import {
    PlayCircleOutlined,
    StarOutlined,
    EllipsisOutlined,
    StarFilled,
  } from '@ant-design/icons-vue';
  import { Modal, message } from 'ant-design-vue';
  import { emitEditProjectEvent, emitShowProjectEvent } from '../common/event';
  import { tauriExecuteCommand } from '@/lib/tauri-handler';
  import { shell } from '@tauri-apps/api';

  const props = defineProps({
    project: {
      type: Object as PropType<ProjectItem>,
      default: () => ({} as ProjectItem),
    },
  });

  const persist = TauriPersist.getInstance();

  function handleShowProject() {
    emitShowProjectEvent(props.project);
  }

  async function handleOpenByDefault() {
    if (!props.project.defaultScheme) {
      message.warning('未找到默认启动配置！');
      return;
    }
    try {
      const defaultScheme = await persist.getSchemeById(props.project.defaultScheme);
      await tauriExecuteCommand(defaultScheme.program, props.project);
    } catch (e: any) {
      message.error(e.message);
    }
  }

  function handleSetStar(favorite) {
    persist.updateProjectItemProperty(props.project.id, {
      favorite,
    });
  }

  function handleOpenProjectDirectory() {
    shell.open(props.project.path);
  }

  function handleEditProjectItem() {
    emitEditProjectEvent(props.project);
  }

  function handleDeleteProjectItem() {
    Modal.confirm({
      title: '删除',
      content: '请确认删除项目[' + props.project.name + ']?',
      type: 'warning',
      okText: '确定',
      cancelText: '取消',
      onOk: async () => {
        await persist.deleteProjectItem(props.project.id);
        message.success('删除成功！');
      },
    });
  }
</script>

<style lang="less">
  .ant-card.grid-item {
    width: 180px;
    height: 120px;
    border-radius: 6px;
    background: rgba(0, 0, 0, 0.5);
    transition: background-position 0.5s ease-in-out;
    cursor: pointer;
    &:hover {
      background: linear-gradient(90deg, rgba(0, 0, 0, 0.5), rgba(0, 0, 0, 0.5), #97d0ff, #97d0ff);
      background-size: 200%;
      animation: grow 2s infinite;
      .ant-card-actions {
        border-color: #d2e7f7;
      }
    }
    @keyframes grow {
      0% {
        background-position: 200%;
      }
      100% {
        background-position: 0%;
      }
    }
    &.active {
      border-color: #97d0ff;
    }

    .ant-card-body {
      height: calc(100% - 36px);
      padding: 12px;
      background: rgba(0, 0, 0, 0.5);
      border-radius: 6px 6px 0 0;
    }

    .ant-card-meta-title {
      font-size: 14px;
    }
    .ant-card-actions {
      border-radius: 0 0 4px 4px;
      border-top: none;
      li {
        margin: 6px 0;
        border: none;
      }
    }
    .ant-card-meta-description {
      height: 40px;
      font-size: 12px;
      overflow: hidden;
      text-overflow: ellipsis;
      user-select: none;
      display: -webkit-box; // 将对象作为弹性伸缩盒子模型展示
      -webkit-line-clamp: 2; // 设置显示的行数
      -webkit-box-orient: vertical; // 设置伸缩盒子的子元素排列方式: 从上到下垂直排列子元素
    }
  }
</style>
