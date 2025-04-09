import { useMutation, useQuery } from '@tanstack/vue-query'
import { fetchApi } from '../fetch'
import type { Post, User } from './types'
import { computed, unref, type MaybeRef } from 'vue'

export const api = {
	auth: {
		useMe: () =>
			useQuery({
				queryKey: ['me'],
				async queryFn() {
					return fetchApi<User>('/auth/me', {
						method: 'GET',
					}).json()
				},
				retry: false,
			}),

		useLogin: () =>
			useMutation({
				async mutationFn(data: { username: string; password: string }) {
					return fetchApi<User>('/auth/login', {
						method: 'POST',
						body: JSON.stringify(data),
					}).json()
				},
			}),

		useLogout: () =>
			useMutation({
				async mutationFn() {
					return fetchApi('/auth/logout', {
						method: 'POST',
					})
				},
			}),
	},

	posts: {
		useList: (options?: MaybeRef<{ limit?: number; offset?: number }>) =>
			useQuery({
				queryKey: ['posts', options],
				async queryFn() {
					const options_ = unref(options)
					const params = new URLSearchParams()
					if (options_?.limit !== undefined) {
						params.append('limit', options_.limit.toString())
					}
					if (options_?.offset !== undefined) {
						params.append('offset', options_.offset.toString())
					}
					return fetchApi<{ data: Post[]; total: number }>(
						'/posts?' + params.toString(),
						{
							method: 'GET',
						}
					).json()
				},
			}),

		useGet: (id: MaybeRef<string>) =>
			useQuery({
				queryKey: ['posts', id],
				async queryFn() {
					return fetchApi<{ data: Post }>(`/posts/${unref(id)}`, {
						method: 'GET',
					}).json()
				},
				enabled: computed(() => !!unref(id)),
			}),

		useCreate: () =>
			useMutation({
				async mutationFn(data: { text: string }) {
					return fetchApi<{ data: Post }>('/posts', {
						method: 'POST',
						body: JSON.stringify(data),
					}).json()
				},
			}),

		useUpdate: () =>
			useMutation({
				async mutationFn({ id, text }: { id: string; text: string }) {
					return fetchApi<{ data: Post }>(`/posts/${id}`, {
						method: 'POST',
						body: JSON.stringify({ text }),
					}).json()
				},
			}),

		useDelete: () =>
			useMutation({
				async mutationFn(id: string) {
					return fetchApi<{ success: boolean }>(`/posts/${id}`, {
						method: 'DELETE',
					}).json()
				},
			}),
	},
}
