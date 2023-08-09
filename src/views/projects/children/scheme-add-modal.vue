<template>
  <a-modal
    class="scheme-add-modal"
    :width="600"
    :open="visibleRef"
    :title="isAddRef ? '添加启动配置' : '修改启动配置'"
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
      <a-form-item
        label="配置名称"
        name="name"
        :rules="[{ required: true, message: '请填写名称！' }]"
      >
        <a-input v-model:value="formModelRef.name"></a-input>
      </a-form-item>
      <a-form-item
        label="启动命令"
        name="program"
        :rules="[{ required: true, message: '请填写命令！' }]"
      >
        <a-input v-model:value="formModelRef.program"></a-input>
      </a-form-item>
      <a-form-item label="启动参数" name="path">
        <a-input v-model:value="formModelRef.args"></a-input>
      </a-form-item>
    </a-form>
  </a-modal>
</template>

<script setup lang="ts">
  import TauriPersist from '@/lib/tauri-persist';
  import { isNullOrUnDef } from '@/utils/is';
  import { getRandomId } from '@/utils/uuid';
  import { message } from 'ant-design-vue';

  const emits = defineEmits<{
    (e: 'complete'): void;
  }>();

  const props = defineProps({
    projectId: {
      type: String,
      default: '',
    },
  });

  const initData = {
    name: '',
    program: '',
    args: '',
  };
  const formModelRef = ref(Object.assign({}, initData));

  const visibleRef = ref(false);

  const isAddRef = ref(true);
  let originStartScheme: Nullable<StartScheme> = null;
  function setItem(item?: StartScheme) {
    visibleRef.value = true;
    if (isNullOrUnDef(item)) {
      isAddRef.value = true;
      originStartScheme = null;
      formModelRef.value = Object.assign({}, initData);
    } else {
      isAddRef.value = false;

      originStartScheme = item;
      formModelRef.value = Object.assign({}, originStartScheme);
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
          projectId: props.projectId,
          ...formModelRef.value,
        };
        await persist.addStartScheme(item);
        message.success('添加成功！');
      } else {
        const item = {
          ...originStartScheme,
          ...formModelRef.value,
        } as StartScheme;
        await persist.updateStartScheme(item);
        message.success('修改成功！');
      }
      visibleRef.value = false;
      emits('complete');
    } catch (e) {}
  }
</script>

<style lang="less">
  .scheme-add-modal {
    .ant-modal-body {
      padding: 30px;
    }
  }
</style>