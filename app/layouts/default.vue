<template>
	<div class="fixed top-0 left-0 bottom-0 w-[17rem] p-4 flex flex-col">
		<div class="navbar invisible"></div>
		<ul class="menu w-full bg-base-200 rounded-box grow overflow-y-auto flex-nowrap">
			<li><a>All Posts</a></li>
		</ul>
	</div>
	<div class="navbar bg-base-100 shadow-sm fixed">
		<div class="navbar-start">
			<a class="btn btn-ghost text-xl">Colink</a>
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
</template>

<script lang="ts" setup>
import { useWebsocket } from '~/stores/ws'

const authData = useAuthData()
const mLogout = useLogoutMutation()
useWebsocket()
</script>
