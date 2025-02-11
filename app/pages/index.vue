<template>
	<div class="font-bold">Hello world</div>
	<div>{{ JSON.stringify(user) }}</div>
</template>

<script lang="ts" setup>
import { useQuery } from '@tanstack/vue-query'

const qUser = useQuery({
	queryKey: ['user'],
	async queryFn() {
		const v = await $fetch('/api/auth/me')
		return v
	},
})
const user = qUser.data
</script>

<style></style>
