<template>
	<div>
		<form @submit="form.onSubmit">
			<input
				type="text"
				:value="form.getInputProps('text').modelValue"
				@input="
					form.getInputProps('text')['onUpdate:modelValue'](($event.target as any).value)
				"
				placeholder="What's on your mind?"
			/>
			<button type="submit">Submit</button>
		</form>
	</div>
</template>

<script setup lang="ts">
import { useQueryClient } from '@tanstack/vue-query'
import { api } from '../../utils/vue-query/api'
import { useForm } from '../../utils/forms'
import { z } from 'zod'

const queryClient = useQueryClient()
const mCreate = api.posts.useCreate()

const form = useForm({
	schema: z.object({
		text: z.string().min(1, 'Content is required'),
	}),
	initialValues: {
		text: '',
	},
	async onSubmit(values) {
		await mCreate.mutateAsync(values)
		queryClient.invalidateQueries()
	},
})
</script>
