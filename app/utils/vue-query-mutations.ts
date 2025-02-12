export function useCreatePostMutation() {
	const queryClient = useQueryClient()
	return useMutation({
		async mutationFn(post: { url: string }) {
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
