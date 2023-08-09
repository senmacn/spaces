<template>
  <a-modal
    class="project-item-add-modal"
    :width="420"
    :open="visibleRef"
    :title="isAddRef ? '添加项目' : '修改项目'"
    @ok="handleComplete"
    @cancel="visibleRef = false"
  >
    <a-form
      ref="itemFormRef"
      size="small"
      :model="formModelRef"
      :label-col="{ span: 4 }"
      :wrapper-col="{ span: 20 }"
      label-align="right"
    >
      <a-form-item label="名称" name="name" :rules="[{ required: true, message: '请填写名称！' }]">
        <a-input v-model:value="formModelRef.name"></a-input>
      </a-form-item>
      <a-form-item
        label="描述"
        name="description"
        :rules="[{ required: true, message: '请填写描述！' }]"
      >
        <a-input v-model:value="formModelRef.description"></a-input>
      </a-form-item>
      <a-form-item label="路径" name="path" :rules="[{ required: true, message: '请填写路径！' }]">
        <a-input v-model:value="formModelRef.path"></a-input>
      </a-form-item>
      <a-form-item label="图标" name="favorite">
        <a-select v-model:value="formModelRef.icon" popup-class-name="icon-select">
          <a-select-option value="">默认</a-select-option>
          <a-select-option value="file">
            <img src="@/assets/images/file.png" />
          </a-select-option>
          <a-select-option value="folder">
            <img src="@/assets/images/folder.png" />
          </a-select-option>
          <a-select-option value="java">
            <img src="@/assets/images/java.png" />
          </a-select-option>
          <a-select-option value="javascript">
            <img src="@/assets/images/javascript.png" />
          </a-select-option>
          <a-select-option value="idea">
            <img src="@/assets/images/idea.png" />
          </a-select-option>
          <a-select-option value="vscode">
            <img src="@/assets/images/vscode.png" />
          </a-select-option>
        </a-select>
      </a-form-item>
      <a-form-item label="星标" name="favorite">
        <a-switch
          v-model:checked="formModelRef.favorite"
          checked-value="1"
          un-checked-value="0"
        ></a-switch>
      </a-form-item>
    </a-form>
  </a-modal>
</template>

<script setup lang="ts">
  import TauriPersist from '@/lib/tauri-persist';
  import { isNullOrUnDef } from '@/utils/is';
  import { getRandomId } from '@/utils/uuid';
  import { message } from 'ant-design-vue';

  const initData = {
    name: '',
    description: '',
    path: '',
    favorite: '0',
    icon: '',
  };
  const formModelRef = ref(Object.assign({}, initData));

  const visibleRef = ref(false);

  const isAddRef = ref(true);
  let originProjectItem: Nullable<ProjectItem> = null;
  function setItem(item?: ProjectItem) {
    visibleRef.value = true;
    if (isNullOrUnDef(item)) {
      isAddRef.value = true;
      originProjectItem = null;
      formModelRef.value = Object.assign({}, initData);
    } else {
      isAddRef.value = false;

      originProjectItem = item;
      formModelRef.value = Object.assign({}, originProjectItem);
    }
  }
  defineExpose({
    setItem,
  });

  const itemFormRef = ref();
  const persist = TauriPersist.getInstance();
  async function handleComplete() {
    try {
      await itemFormRef.value.validate();
      if (isAddRef.value) {
        const item = {
          id: getRandomId(),
          usedAt: '',
          deletedAt: null,
          defaultScheme: null,
          tags: '',
          ...formModelRef.value,
        };
        await persist.addProjectItem(item);
        message.success('添加成功！');
      } else {
        const item = {
          ...originProjectItem,
          ...formModelRef.value,
        } as ProjectItem;
        await persist.updateProjectItem(item);
        message.success('修改成功！');
      }
      visibleRef.value = false;
    } catch (e) {
      console.warn(e);
      message.error('创建失败！');
    }
  }
</script>

<style lang="less">
  .project-item-add-modal {
    .ant-modal-body {
      padding: 30px;
    }
    .ant-select {
      .ant-select-selection-item img {
        width: 20px;
        height: 20px;
      }
    }
  }
  .icon-select {
    .ant-select-item-option-content img {
      width: 30px;
      height: 30px;
    }
  }
</style>
