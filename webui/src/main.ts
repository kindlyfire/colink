import { createApp, defineComponent, h } from 'vue'
import './assets/style.css'
import { createRouter, createWebHistory, RouterView, type RouteRecordRaw } from 'vue-router'
import { VueQueryPlugin } from '@tanstack/vue-query'
import HomePage from './pages/HomePage/HomePage.vue'
import LoginPage from './pages/auth/LoginPage.vue'

const routes: RouteRecordRaw[] = [
	{ path: '/', component: HomePage },
	{ path: '/auth/login', component: LoginPage },
]

const router = createRouter({
	history: createWebHistory(),
	routes,
})

const App = defineComponent({
	setup() {
		return () => h(RouterView)
	},
})

createApp(App).use(router).use(VueQueryPlugin).mount('#app')
