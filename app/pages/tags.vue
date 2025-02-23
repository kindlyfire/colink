<template>
	<div class="flex flex-col gap-4">
		<h1 class="text-2xl font-bold">Manage tags</h1>

		<div class="flex flex-wrap gap-1">
			<div v-for="tag in qTags.data.value" class="badge badge-soft badge-primary badge-lg">
				{{ tag.name }}
				<button class="ml-1 cursor-pointer" @click="deleteTag(tag.id)">×</button>
			</div>
		</div>

		<div>
			<form @submit="form.onSubmit">
				<div class="flex flex-col gap-2">
					<div>
						<AInput v-bind="form.getInputProps('name')" label="Tag name" />
					</div>
					<div>
						<button class="btn btn-primary" type="submit">Create</button>
					</div>
				</div>
			</form>
		</div>
	</div>
</template>

<script lang="ts" setup>
import { z } from 'zod'

const qTags = useTagsQuery()
const createTagMutation = useCreateTagMutation()
const deleteTagMutation = useDeleteTagMutation()

const form = useForm({
	schema: z.object({
		name: z.string().nonempty(),
	}),
	initialValues: {
		name: '',
	},
	async onSubmit(values) {
		await createTagMutation.mutateAsync({
			name: values.name,
		})
		form.setValues({ name: '' })
	},
})

function deleteTag(id: string) {
	deleteTagMutation.mutate({ id })
}
</script>
