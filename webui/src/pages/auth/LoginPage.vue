<template>
	<form @submit="onSubmit">
		<div>
			<label for="username">Username:</label>
			<input type="text" id="username" v-model="formData.username" required />
		</div>
		<div>
			<label for="password">Password:</label>
			<input type="password" id="password" v-model="formData.password" required />
		</div>
		<button type="submit">Login</button>
	</form>
</template>

<script setup lang="ts">
import { reactive } from 'vue'
import { api } from '../../utils/vue-query/api'
import { useRouter } from 'vue-router'

const formData = reactive({
	username: '',
	password: '',
})

const mLogin = api.auth.useLogin()
const router = useRouter()

async function onSubmit(e: Event) {
	e.preventDefault()
	await mLogin.mutateAsync({
		username: formData.username,
		password: formData.password,
	})
	router.push('/')
}
</script>
