import { createRouter, createWebHistory } from 'vue-router';
import { useAuthStore } from '../stores/auth';

// Import views
import Dashboard from '../views/Dashboard.vue';
import Login from '../views/Login.vue';
import Register from '../views/Register.vue';
import Teams from '../views/Teams.vue';
import TeamDetail from '../views/TeamDetail.vue';
import Projects from '../views/Projects.vue';
import ProjectDetail from '../views/ProjectDetail.vue';
import Profile from '../views/Profile.vue';
import Invite from '../views/Invite.vue';
import NotFound from '../views/NotFound.vue';

const routes = [
  {
    path: '/',
    name: 'Dashboard',
    component: Dashboard,
    meta: { requiresAuth: true },
  },
  {
    path: '/login',
    name: 'Login',
    component: Login,
    meta: { requiresGuest: true },
  },
  {
    path: '/register',
    name: 'Register',
    component: Register,
    meta: { requiresGuest: true },
  },
  {
    path: '/teams',
    name: 'Teams',
    component: Teams,
    meta: { requiresAuth: true },
  },
  {
    path: '/teams/:id',
    name: 'TeamDetail',
    component: TeamDetail,
    meta: { requiresAuth: true },
  },
  {
    path: '/projects',
    name: 'Projects',
    component: Projects,
    meta: { requiresAuth: true },
  },
  {
    path: '/projects/:id',
    name: 'ProjectDetail',
    component: ProjectDetail,
    meta: { requiresAuth: true },
  },
  {
    path: '/profile',
    name: 'Profile',
    component: Profile,
    meta: { requiresAuth: true },
  },
  {
    path: '/invite/:id',
    name: 'Invite',
    component: Invite,
    meta: { requiresAuth: true },
  },
  {
    path: '/:pathMatch(.*)*',
    name: 'NotFound',
    component: NotFound,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

// Navigation guards
router.beforeEach((to, from, next) => {
  const authStore = useAuthStore();

  if (to.meta.requiresAuth && !authStore.isAuthenticated) {
    next('/login');
  } else if (to.meta.requiresGuest && authStore.isAuthenticated) {
    next('/');
  } else {
    next();
  }
});

export default router;
