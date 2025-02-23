import type { IView } from '~~/server/db/schema'

export function useAuthData() {
	return useAuthDataQuery().data
}

export function useAuthDataQuery() {
	const requestFetch = useRequestFetch()
	return useQuery({
		queryKey: ['auth-data'],
		queryFn: async () => {
			const res = await requestFetch('/api/auth/me')
			return res
		},
	})
}

export function usePostsQuery(filters?: MaybeRef<IView['filters']>) {
	const requestFetch = useRequestFetch()
	return useQuery({
		queryKey: ['posts', filters],
		async queryFn() {
			// TODO: Implement filters
			const res = await requestFetch('/api/posts')
			return res
		},
		placeholderData: [],
	})
}

export function usePostQuery(id: MaybeRef<string>) {
	const requestFetch = useRequestFetch()
	return useQuery({
		queryKey: ['post', id],
		async queryFn() {
			const res = await requestFetch(`/api/posts/${unref(id)}`)
			return res
		},
	})
}

export function useViewsQuery() {
	const requestFetch = useRequestFetch()
	return useQuery({
		queryKey: ['views'],
		async queryFn() {
			const res = await requestFetch('/api/views')
			return res
		},
		placeholderData: [],
	})
}

export function useTagsQuery() {
	const requestFetch = useRequestFetch()
	return useQuery({
		queryKey: ['tags'],
		async queryFn() {
			const res = await requestFetch('/api/tags')
			return res
		},
		placeholderData: [],
	})
}
