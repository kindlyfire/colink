<template>
	<ASidebar class="w-[17rem]">
		<ul class="menu w-full">
			<li>
				<NuxtLink to="/" @click="viewsStore.setViewSelection({ type: 'all-posts' })">
					All Posts
				</NuxtLink>
			</li>
			<li><a>Tags</a></li>
			<template v-if="qViews.data">
				<li>
					<div class="menu-title pt-2">Views</div>
				</li>
				<li v-for="view in qViews.data.value" :key="view.id" class="relative">
					<button
						class="flex items-center justify-between"
						@click="viewsStore.setViewSelection({ type: 'view', id: view.id })"
					>
						{{ view.name }}
					</button>
					<button
						@click="
							() => {
								editView = view
								editViewModalOpen = true
							}
						"
						class="absolute right-0"
					>
						Edit
					</button>
				</li>
				<li>
					<button
						class="btn btn-sm"
						@click="
							() => {
								editView = null
								editViewModalOpen = true
							}
						"
					>
						+
					</button>
				</li>
			</template>
		</ul>
	</ASidebar>
	<div class="navbar bg-base-100 shadow-sm fixed">
		<div class="navbar-start">
			<NuxtLink to="/" class="btn btn-ghost text-xl">Colink</NuxtLink>
		</div>
		<div class="navbar-end">
			<div>
				<button
					class="btn btn-sm"
					popovertarget="popover-user"
					style="anchor-name: --anchor-user-popover"
				>
					{{ authData?.user.username }}
				</button>
				<ul
					class="dropdown menu w-52 rounded-box bg-base-100 shadow-sm"
					popover
					id="popover-user"
					style="position-anchor: --anchor-user-popover"
				>
					<li>
						<button @click="mLogout.mutate({})" :disabled="mLogout.isPending.value">
							Logout
						</button>
					</li>
				</ul>
			</div>
		</div>
	</div>
	<div class="navbar invisible"></div>
	<div class="pl-[17rem]">
		<div class="p-4 pl-0">
			<slot />
			<div class="h-4"></div>
		</div>
	</div>

	<EditViewModal v-model:open="editViewModalOpen" :view="editView" />
</template>

<script lang="ts" setup>
import { useWebsocket } from '~/stores/ws'
import { useViewsQuery } from '~/utils/vue-query-queries'
import EditViewModal from '~/components/EditViewModal/index.vue'
import type { IView } from '~~/server/db/schema'

const authData = useAuthData()
const mLogout = useLogoutMutation()
const qViews = useViewsQuery()
useWebsocket()

const editViewModalOpen = ref(false)
const editView = ref<SerializeObject<IView> | null>(null)

if (import.meta.server) {
	await qViews.suspense()
}

const viewsStore = useViewsStore()
</script>
