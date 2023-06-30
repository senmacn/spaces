import { createRouter, createWebHashHistory, RouteRecordRaw } from 'vue-router';
import Home from '../views/home/index.vue';
import Projects from '../views/projects/projects-index.vue';

const routes: RouteRecordRaw[] = [
  {
    name: 'home',
    path: '/home',
    component: Home,
  },
  {
    name: 'projects',
    path: '/projects',
    component: Projects,
  },
  {
    path: '/:pathMatch(.*)',
    redirect: '/home'
  }
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
  scrollBehavior() {
    // Always scroll to the top
    return { top: 0 };
  },
});

export default router;
