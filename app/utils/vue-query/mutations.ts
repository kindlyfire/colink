export function useLogoutMutation() {
	const router = useRouter()
	const queryClient = useQueryClient()
	return useMutation({
		async mutationFn(options?: { redirect?: boolean }) {
			await $fetch('/api/auth/logout', {
				method: 'POST',
			})
			queryClient.resetQueries({
				queryKey: ['auth-data'],
				exact: true,
			})
			if (options?.redirect ?? true) {
				router.push('/auth/login')
			}
			return true
		},
	})
}

export function useCreatePostMutation() {
	const queryClient = useQueryClient()
	return useMutation({
		async mutationFn(post: { url: string }) {
			if (!post.url) return
			const res = await $fetch('/api/posts', {
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
			const res = await $fetch(`/api/posts/${post.id}`, {
				method: 'DELETE',
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
			const res = await $fetch(`/api/posts/${post.id}`, {
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
			const res = await $fetch('/api/views', {
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
			const res = await $fetch(`/api/views/${view.id}`, {
				method: 'DELETE',
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
			const res = await $fetch(`/api/views/${view.id}`, {
				method: 'POST',
				body: view,
			})
			queryClient.invalidateQueries()
			return res
		},
	})
}

export function useCreateTagMutation() {
	const queryClient = useQueryClient()
	return useMutation({
		async mutationFn(tag: { name: string }) {
			const res = await $fetch('/api/tags', {
				method: 'POST',
				body: tag,
			})
			queryClient.invalidateQueries()
			return res
		},
	})
}

export function useUpdateTagMutation() {
	const queryClient = useQueryClient()
	return useMutation({
		async mutationFn(tag: { id: string; name: string }) {
			const res = await $fetch(`/api/tags/${tag.id}`, {
				method: 'POST',
				body: tag,
			})
			queryClient.invalidateQueries()
			return res
		},
	})
}

export function useDeleteTagMutation() {
	const queryClient = useQueryClient()
	return useMutation({
		async mutationFn(tag: { id: string }) {
			const res = await $fetch(`/api/tags/${tag.id}`, {
				method: 'DELETE',
			})
			queryClient.invalidateQueries()
			return res
		},
	})
}
