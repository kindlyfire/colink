export function usePostsQuery() {
	const requestFetch = useRequestFetch()
	return useQuery({
		queryKey: ['posts'],
		async queryFn() {
			const res = await requestFetch('/api/posts')
			return res
		},
	})
}
