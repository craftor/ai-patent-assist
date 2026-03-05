import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  {
    path: '/login',
    name: 'Login',
    component: () => import('@/pages/Login.vue'),
  },
  {
    path: '/',
    name: 'Layout',
    component: () => import('@/layouts/MainLayout.vue'),
    redirect: '/dashboard',
    children: [
      {
        path: 'dashboard',
        name: 'Dashboard',
        component: () => import('@/pages/Dashboard.vue'),
        meta: { title: '仪表盘' },
      },
      {
        path: 'projects',
        name: 'Projects',
        component: () => import('@/pages/Projects.vue'),
        meta: { title: '项目管理' },
      },
      {
        path: 'projects/:id',
        name: 'ProjectDetail',
        component: () => import('@/pages/ProjectDetail.vue'),
        meta: { title: '项目详情' },
      },
      {
        path: 'patents',
        name: 'Patents',
        component: () => import('@/pages/Patents.vue'),
        meta: { title: '专利文档' },
      },
      {
        path: 'patents/create',
        name: 'CreatePatent',
        component: () => import('@/pages/patents/Create.vue'),
        meta: { title: '新建专利文档' },
      },
      {
        path: 'patents/:id',
        name: 'PatentDetail',
        component: () => import('@/pages/patents/Detail.vue'),
        meta: { title: '专利详情' },
      },
      {
        path: 'patents/:id/edit',
        name: 'EditPatent',
        component: () => import('@/pages/patents/Edit.vue'),
        meta: { title: '编辑专利' },
      },
      {
        path: 'copyrights',
        name: 'Copyrights',
        component: () => import('@/pages/Copyrights.vue'),
        meta: { title: '软著文档' },
      },
      {
        path: 'copyrights/create',
        name: 'CreateCopyright',
        component: () => import('@/pages/copyrights/Create.vue'),
        meta: { title: '新建软著文档' },
      },
      {
        path: 'copyrights/:id',
        name: 'CopyrightDetail',
        component: () => import('@/pages/copyrights/Detail.vue'),
        meta: { title: '软著详情' },
      },
      {
        path: 'copyrights/:id/edit',
        name: 'EditCopyright',
        component: () => import('@/pages/copyrights/Edit.vue'),
        meta: { title: '编辑软著' },
      },
      {
        path: 'templates',
        name: 'Templates',
        component: () => import('@/pages/Templates.vue'),
        meta: { title: '模板管理' },
      },
      {
        path: 'settings',
        name: 'Settings',
        component: () => import('@/pages/Settings.vue'),
        meta: { title: '系统设置' },
      },
    ],
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

router.beforeEach((to, from, next) => {
  const token = localStorage.getItem('access_token')

  if (!token && to.path !== '/login') {
    next('/login')
  } else if (token && to.path === '/login') {
    next('/dashboard')
  } else {
    next()
  }
})

export default router
