<template>
	<div class="card bg-base-100 shadow-sm border-gray-200 border">
		<form @submit="form.onSubmit">
			<!-- TODO: Ctrl+enter submission -->
			<textarea
				ref="textarea"
				type="text"
				v-bind="form.getInputProps('text')"
				placeholder="What's up?"
				class="w-full rounded-t-box p-2"
				autofocus
			></textarea>
			<div class="p-2">
				<button type="submit" class="btn btn-sm">Submit</button>
			</div>
		</form>
	</div>
</template>

<script setup lang="ts">
import { useQueryClient } from '@tanstack/vue-query'
import { api } from '../../utils/vue-query/api'
import { useForm } from '../../utils/forms'
import { z } from 'zod'
import { ref } from 'vue'

const queryClient = useQueryClient()
const mCreate = api.posts.useCreate()
const textarea = ref<HTMLTextAreaElement | null>(null)

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
		form.reset()
		textarea.value?.focus()
	},
})
</script>
