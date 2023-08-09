import { onUnmounted } from 'vue';
import { EventBusListener, useEventBus } from '@vueuse/core';

const editProjectEvent = Symbol('editProject');
const editBus = useEventBus<symbol>(editProjectEvent);
const onEditProjectEvent = (listener: EventBusListener) => {
  editBus.on(listener);
  onUnmounted(() => editBus.off(listener));
};
const emitEditProjectEvent = (payload) => editBus.emit(editProjectEvent, payload);

const showProjectEvent = Symbol('showProject');
const showBus = useEventBus<symbol>(showProjectEvent);
const onShowProjectEvent = (listener: EventBusListener) => {
  showBus.on(listener);
  onUnmounted(() => showBus.off(listener));
};
const emitShowProjectEvent = (payload) => showBus.emit(showProjectEvent, payload);

export { onEditProjectEvent, emitEditProjectEvent, onShowProjectEvent, emitShowProjectEvent };
