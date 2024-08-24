import { createRouter, createWebHistory } from 'vue-router'
import LoginView from '../views/LoginView.vue'
import useUser from '@/stores/currentUser'


const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'login',
      component: LoginView,
    },
    {
      path: '/init',
      name: 'init',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import('../views/InitView.vue'),
    },
    {
      path: '/effects',
      name: 'effects',
      component: () => import('../views/EffectsView.vue'),
    },
    {
      path: '/info',
      name: 'info',
      component: () => import('../views/InfoView.vue'),
    },
    {
      path: '/encounter',
      name: 'encounter',
      component: () => import('../views/MockEncounterCreator.vue'),
    }
  ],
  linkActiveClass: 'active',
  linkExactActiveClass: 'exact-active'
})

router.beforeEach((to, from) => {
  let user = useUser()
  if (!user.isLoggedIn && to.name != "login") {
    return {name: 'login'} 
  }
  return true;
})

export default router
