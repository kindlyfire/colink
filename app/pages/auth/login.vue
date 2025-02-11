<template>
	<div class="card-body">
		<div class="text-2xl font-bold text-center">Login</div>
		<form @submit="form.onSubmit">
			<AInput v-bind="form.getInputProps('username')" label="Username" />
			<AInput v-bind="form.getInputProps('password')" label="Password" type="password" />

			<div class="flex justify-center mt-4">
				<button class="btn btn-info" type="submit">Log in</button>
			</div>
		</form>
	</div>
</template>

<script setup lang="ts">
import { z } from 'zod'

definePageMeta({
	layout: 'auth',
})
useSeoMeta({
	title: 'Login',
})

const router = useRouter()

const form = useForm({
	schema: z.object({
		username: z.string().nonempty(),
		password: z.string().nonempty(),
	}),
	initialValues: {
		username: '',
		password: '',
	},
	async onSubmit(values) {
		const res = await $fetch('/api/auth/login', {
			method: 'POST',
			body: {
				username: values.username,
				password: values.password,
			},
		})
		router.push('/')
	},
})
</script>
