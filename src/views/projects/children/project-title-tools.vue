<template>
  <div class="project-title-tools">
    <div class="search-tool">
      <a-input-search
        v-model:value="searchValueRef"
        allow-clear
        placeholder="搜索"
        @search="handleSearchChange"
      />
    </div>
    <div>
      <a-tooltip title="全部">
        <a-button
          type="text"
          :class="[filterRef === FilterType.ALL && 'active']"
          @click="handleChangeFilterType(FilterType.ALL)"
        >
          <table-outlined></table-outlined>
        </a-button>
      </a-tooltip>
      <a-tooltip
        title="标签"
        :class="[filterRef === FilterType.TAG && 'active']"
        @click="handleChangeFilterType(FilterType.TAG)"
      >
        <a-button type="text">
          <tags-outlined></tags-outlined>
        </a-button>
      </a-tooltip>
      <a-tooltip
        title="加星"
        :class="[filterRef === FilterType.STAR && 'active']"
        @click="handleChangeFilterType(FilterType.STAR)"
      >
        <a-button type="text">
          <star-outlined></star-outlined>
        </a-button>
      </a-tooltip>
      <a-tooltip
        title="回收站"
        :class="[filterRef === FilterType.DELETED && 'active']"
        @click="handleChangeFilterType(FilterType.DELETED)"
      >
        <a-button type="text">
          <rest-outlined></rest-outlined>
        </a-button>
      </a-tooltip>
    </div>
    <div class="divider">
      <a-divider type="vertical" orientation="center" style="height: 22px; top: 4px"></a-divider>
    </div>
    <div>
      <a-tooltip title="添加" @click="handleProjectItemAdd">
        <a-button type="text">
          <appstore-add-outlined></appstore-add-outlined>
        </a-button>
      </a-tooltip>
    </div>
  </div>
</template>

<script setup lang="ts">
  import {
    TableOutlined,
    TagsOutlined,
    StarOutlined,
    RestOutlined,
    AppstoreAddOutlined,
  } from '@ant-design/icons-vue';
  import { FilterType } from '../common/types';
  import { useProjectStore } from '@/store/modules/project-state';

  const emits = defineEmits<{
    (e: 'add-item');
  }>();

  const state = useProjectStore();

  const filterRef = ref<FilterType>(FilterType.ALL);
  function handleChangeFilterType(type: FilterType) {
    filterRef.value = type;
    state.setFilter(type);
  }

  const searchValueRef = ref('');
  function handleSearchChange() {
    state.setSearch(searchValueRef.value);
  }

  function handleProjectItemAdd() {
    emits('add-item');
  }
</script>

<style lang="less">
  .project-title-tools {
    width: 100%;
    display: flex;
    > div {
      padding: 5px 0;
    }
    .search-tool {
      flex: 1;
      padding-right: 10px;
    }
    .divider {
      padding-right: 10px;
      padding-left: 10px;
    }
    button.active {
      color: #40a9ff;
      background: rgba(0, 0, 0, 0.018);
      border-color: transparent;
    }
  }
</style>
