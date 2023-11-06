import { createApp } from 'vue';
import App from './App.vue';
import Antd from 'ant-design-vue';
import { setupStore } from './store';
import router from './routes';
import './style.less';
import './style.less';

async function bootstrap() {
  const app = createApp(App);

  setupStore(app);
  app.use(router);
  app.use(Antd);

  app.mount('#app');
}

bootstrap();
