import {
	QueryClient,
	VueQueryPlugin,
	type DehydratedState,
	dehydrate,
	hydrate,
} from '@tanstack/vue-query'

export default defineNuxtPlugin(nuxtApp => {
	const vueQueryState = useState<DehydratedState | null>('vue-query')

	const queryClient = new QueryClient({
		defaultOptions: {
			queries: {
				retry: false,
				refetchOnMount: false,
				refetchOnWindowFocus: false,
			},
		},
	})

	nuxtApp.vueApp.use(VueQueryPlugin, { queryClient })

	if (import.meta.server) {
		nuxtApp.hooks.hook('app:rendered', () => {
			vueQueryState.value = dehydrate(queryClient)
		})
	}

	if (import.meta.client) {
		nuxtApp.hooks.hook('app:created', () => {
			hydrate(queryClient, vueQueryState.value)
		})
	}
})
