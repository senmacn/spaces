import { createRouter, createWebHashHistory, RouteRecordRaw } from 'vue-router';
import Home from '../views/home/home-index.vue';
import Projects from '../views/projects/projects-index.vue';
import Starting from '../views/starting/starting-index.vue';
import Spaces from '../Spaces.vue';

const routes: RouteRecordRaw[] = [
  {
    name: 'starting',
    path: '/starting',
    component: Starting,
  },
  {
    name: 'spaces',
    path: '/spaces',
    component: Spaces,
    children: [
      {
        path: '/',
        redirect: '/projects',
      },
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
        name: 'setting',
        path: '/setting',
        component: Home,
      },
    ],
  },
  {
    path: '/:pathMatch(.*)',
    redirect: '/spaces',
  },
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
