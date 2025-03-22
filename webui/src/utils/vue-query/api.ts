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
			}),

		useLogin: () =>
			useMutation({
				async mutationFn(data: { username: string; password: string }) {
					return fetchApi<User>('/auth/login', {
						method: 'POST',
						body: JSON.stringify(data),
					})
				},
			}),
	},

	posts: {
		useList: () =>
			useQuery({
				queryKey: ['posts'],
				async queryFn() {
					return fetchApi<{ data: Post[] }>('/posts', {
						method: 'GET',
					}).json()
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
