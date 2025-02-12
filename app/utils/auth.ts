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
