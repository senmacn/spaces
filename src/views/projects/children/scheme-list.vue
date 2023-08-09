<template>
  <a-list class="scheme-list" size="small" header="启动选项" bordered>
    <template v-for="scheme in startSchemesRef" :key="scheme.id">
      <a-list-item>
        <a-tooltip title="双击启动">
          <div class="scheme-name" @dblclick="() => handleExecuteCommand(scheme)">
            <check-outlined
              v-if="scheme.id === project.defaultScheme"
              class="success-color"
            ></check-outlined>
            {{ scheme.name }}
          </div>
          <div class="scheme-program" @dblclick="() => handleExecuteCommand(scheme)">
            {{ scheme.program }}
          </div>
        </a-tooltip>
        <div class="scheme-options">
          <a @click="() => handleExecuteCommand(scheme)">
            <play-circle-outlined class="success-color"></play-circle-outlined>
          </a>
          <a v-if="scheme.id !== project.defaultScheme" @click="handleSetDefault(scheme)">
            <check-square-outlined />
          </a>
          <a>
            <edit-outlined @click="() => handleShowSchemeAddModal(false, scheme)"></edit-outlined>
          </a>
          <a>
            <delete-outlined @click="() => handleDeleteScheme(scheme)"></delete-outlined>
          </a>
        </div>
      </a-list-item>
    </template>
    <a-list-item>
      <a-button class="scheme-add-btn" type="text" @click="() => handleShowSchemeAddModal(true)">
        <plus-circle-outlined></plus-circle-outlined>
      </a-button>
    </a-list-item>
  </a-list>
  <scheme-add-modal
    ref="schemeAddModalRef"
    :project-id="project.id"
    @complete="handleGetStartSchemeList"
  ></scheme-add-modal>
</template>

<script setup lang="ts">
  import TauriPersist from '@/lib/tauri-persist';
  import schemeAddModal from './scheme-add-modal.vue';
  import {
    CheckOutlined,
    CheckSquareOutlined,
    PlusCircleOutlined,
    PlayCircleOutlined,
    DeleteOutlined,
    EditOutlined,
  } from '@ant-design/icons-vue';
  import { isArray } from '@/utils/is';
  import { Modal, message } from 'ant-design-vue';
  import { tauriExecuteCommand } from '@/lib/tauri-handler';

  const props = defineProps({
    project: {
      type: Object as PropType<ProjectItem>,
      default: () => ({} as ProjectItem),
    },
  });
  1;
  const startSchemesRef: Ref<StartScheme[]> = ref([]);
  const persist = TauriPersist.getInstance();
  watch(
    () => props.project.id,
    () => {
      if (props.project.id.length) {
        handleGetStartSchemeList();
      }
    },
  );
  function handleGetStartSchemeList() {
    persist.getStartSchemeList(props.project.id).then((data) => {
      if (isArray(data)) {
        startSchemesRef.value = data;
      }
    });
  }

  function handleSetDefault(scheme: StartScheme) {
    persist.updateProjectItemProperty(props.project.id, {
      // @ts-ignore
      default_scheme: scheme.id,
    });
  }

  function handleExecuteCommand(scheme: StartScheme) {
    tauriExecuteCommand(scheme.program, props.project).catch(() => {
      message.error('执行失败！');
    });
  }

  const schemeAddModalRef = ref();
  function handleShowSchemeAddModal(isAdd: boolean, scheme?: StartScheme) {
    schemeAddModalRef.value.setItem(isAdd ? null : scheme);
  }

  function handleDeleteScheme(scheme: StartScheme) {
    Modal.confirm({
      title: '删除',
      content: '请确认删除配置[' + scheme.name + ']?',
      type: 'warning',
      okText: '确定',
      cancelText: '取消',
      onOk: async () => {
        await persist.deleteStartScheme(scheme.id);
        message.success('删除成功！');
      },
    });
  }

  onMounted(() => {
    handleGetStartSchemeList();
  });
</script>

<style lang="less">
  .scheme-list {
    font-size: 12px;
    &.ant-list {
      .ant-list-item:last-child {
        padding: 0;
      }
    }
    .scheme-name {
      font-size: 12px;
      cursor: pointer;
    }
    .scheme-program {
      font-size: 12px;
      cursor: pointer;
      color: rgba(255, 255, 255, 0.6);
    }
    .scheme-options {
      width: 100px;
      display: flex;
      justify-content: space-around;
      font-size: 16px;
      a {
        display: inline-block;
      }
    }
    .scheme-add-btn {
      width: 100%;
      height: 100%;
    }
  }
</style>
