import { acceptHMRUpdate, defineStore } from 'pinia'
import z from 'zod'

const zViewSelectionAllPosts = z.object({
	type: z.literal('all-posts'),
})

const zViewSelectionView = z.object({
	type: z.literal('view'),
	id: z.string(),
})

const zViewSelection = z.discriminatedUnion('type', [zViewSelectionAllPosts, zViewSelectionView])

type ViewSelection = z.infer<typeof zViewSelection>
type ViewSelectionView = z.infer<typeof zViewSelectionView>

export const useViewsStore = defineStore('views', () => {
	const viewSelection = useCookie<ViewSelection>('app-state:selected-view', {
		default() {
			return { type: 'all-posts' }
		},
	})
	const qViews = useViewsQuery()

	if (import.meta.server) {
		// Make sure we're never passed invalid data
		try {
			viewSelection.value = zViewSelection.parse(viewSelection.value)
		} catch (e) {
			viewSelection.value = { type: 'all-posts' }
		}
	}

	const selectedView = computed(() => {
		return viewSelection.value.type === 'view'
			? qViews.data.value.find(
					view => view.id === (viewSelection.value as ViewSelectionView).id
			  ) || null
			: null
	})

	return {
		qViews,
		views: qViews.data,
		selectedView,
		viewSelectionType: computed(() => viewSelection.value.type),
		setViewSelection(viewSelection_: ViewSelection) {
			viewSelection.value = viewSelection_
		},
	}
})

if (import.meta.hot) {
	import.meta.hot.accept(acceptHMRUpdate(useViewsStore, import.meta.hot))
}
