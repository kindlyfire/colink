import type { IView } from '~~/server/db/schema'

export function usePostsQuery(filters?: MaybeRef<IView['filters']>) {
	const requestFetch = useRequestFetch()
	return useQuery({
		queryKey: ['posts', filters],
		async queryFn() {
			// TODO: Implement filters
			const res = await requestFetch('/api/posts')
			return res
		},
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
