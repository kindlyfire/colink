import { acceptHMRUpdate, defineStore } from 'pinia'
import { WebSocket } from 'partysocket'
import type { WebsocketEvent } from '../../server/api/ws'

export const useWebsocket = defineStore('websocket', () => {
	const queryClient = useQueryClient()
	const authData = useAuthData()
	const ws = ref<WebSocket | null>(null)

	function onMessage(event: MessageEvent) {
		try {
			var data: WebsocketEvent = JSON.parse(event.data)
		} catch (e) {
			console.error('Invalid event from WebSocket:', event.data)
			return
		}

		console.log('WebSocket event:', data)

		if (data.type === 'dataChanged') {
			// This isn't much in terms of granularity, but it will do.
			// People-scale software can get away with it.
			queryClient.invalidateQueries()
		}
	}

	watch(
		() => authData.value,
		authData => {
			if (authData) {
				ws.value = new WebSocket(`/api/ws`)
				ws.value.addEventListener('open', () => {
					console.log('Connected to WebSocket')
				})
				ws.value.addEventListener('close', () => {
					console.log('Disconnected from WebSocket')
				})
				ws.value.addEventListener('message', event => {
					onMessage(event)
				})
			} else if (ws.value) {
				ws.value.close()
				ws.value = null
			}
		},
		{ immediate: true }
	)

	return {}
})

if (import.meta.hot) {
	import.meta.hot.accept(acceptHMRUpdate(useWebsocket, import.meta.hot))
}
