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
