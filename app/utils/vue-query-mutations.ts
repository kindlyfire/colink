export function useCreatePostMutation() {
	const queryClient = useQueryClient()
	return useMutation({
		async mutationFn(post: { url: string }) {
			if (!post.url) return
			const res = await $fetch('/api/posts/create', {
				method: 'POST',
				body: {
					url: post.url,
				},
			})
			queryClient.invalidateQueries()
			return res
		},
	})
}

export function useScrapePostMutation() {
	const queryClient = useQueryClient()
	return useMutation({
		async mutationFn(post: { id: string }) {
			const res = await $fetch(`/api/posts/${post.id}/scrape`, {
				method: 'POST',
			})
			queryClient.invalidateQueries()
			return res
		},
	})
}

export function useDeletePostMutation() {
	const queryClient = useQueryClient()
	return useMutation({
		async mutationFn(post: { id: string }) {
			const res = await $fetch(`/api/posts/${post.id}/delete`, {
				method: 'POST',
			})
			queryClient.invalidateQueries()
			return res
		},
	})
}

export function useUpdatePostMutation() {
	const queryClient = useQueryClient()
	return useMutation({
		async mutationFn(post: { id: string; title: string; tags: string[] }) {
			const res = await $fetch(`/api/posts/${post.id}/update`, {
				method: 'POST',
				body: post,
			})
			queryClient.invalidateQueries()
			return res
		},
	})
}

export function useCreateViewMutation() {
	const queryClient = useQueryClient()
	return useMutation({
		async mutationFn(view: { name: string; filters: { tagIds: string[] } }) {
			const res = await $fetch('/api/views/create', {
				method: 'POST',
				body: view,
			})
			queryClient.invalidateQueries()
			return res
		},
	})
}

export function useDeleteViewMutation() {
	const queryClient = useQueryClient()
	return useMutation({
		async mutationFn(view: { id: string }) {
			const res = await $fetch(`/api/views/${view.id}/delete`, {
				method: 'POST',
			})
			queryClient.invalidateQueries()
			return res
		},
	})
}

export function useUpdateViewMutation() {
	const queryClient = useQueryClient()
	return useMutation({
		async mutationFn(view: { id: string; name: string; filters: { tagIds: string[] } }) {
			const res = await $fetch(`/api/views/${view.id}/update`, {
				method: 'POST',
				body: view,
			})
			queryClient.invalidateQueries()
			return res
		},
	})
}
