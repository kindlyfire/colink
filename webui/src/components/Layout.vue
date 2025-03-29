<template>
	<div class="bg-base-100 shadow-sm border-gray-200 border-b">
		<div class="a-container-flush">
			<div class="navbar">
				<div class="flex-1">
					<RouterLink to="/" class="btn btn-ghost text-xl">Colink</RouterLink>
				</div>
				<div class="flex-none">
					<ul class="menu menu-horizontal px-1">
						<li>
							<details class="dropdown-end">
								<summary>{{ me?.username }}</summary>
								<ul
									class="bg-base-100 border-gray-200 border p-2 !mt-1 w-[150px] dropdown-content z-50"
								>
									<li>
										<button @click="logout" :disabled="mLogout.isPending.value">
											Logout
										</button>
									</li>
								</ul>
							</details>
						</li>
					</ul>
				</div>
			</div>
		</div>
	</div>

	<div class="a-container">
		<slot />
	</div>
</template>

<script setup lang="ts">
import { useQueryClient } from '@tanstack/vue-query'
import { api } from '../utils/vue-query/api'
import { watch } from 'vue'
import { RequestError } from '../utils/fetch'
import { useRouter } from 'vue-router'

const queryClient = useQueryClient()
const qMe = api.auth.useMe()
const me = qMe.data
const mLogout = api.auth.useLogout()
const router = useRouter()

watch(
	() => qMe.error.value,
	e => {
		if (e instanceof RequestError && e.response.status === 401) {
			router.push('/auth/login')
		}
	}
)

function logout() {
	mLogout.mutateAsync().then(() => {
		queryClient.invalidateQueries()
		router.push('/auth/login')
	})
}
</script>
