<template>
	<Teleport to="body">
		<dialog ref="modal" class="modal">
			<div class="modal-box">
				<form method="dialog">
					<button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">
						✕
					</button>
				</form>
				<h3 class="font-bold text-lg mb-4">{{ view ? 'Edit' : 'Create' }} View</h3>
				<form @submit="form.onSubmit">
					<AInput v-bind="form.getInputProps('name')" label="View Name" />
					<div class="modal-action">
						<button class="btn btn-ghost" @click="$emit('update:open', false)">
							Cancel
						</button>
						<button
							v-if="view"
							type="button"
							class="btn btn-error"
							@click="handleDelete"
							:disabled="deleteView.isPending.value"
						>
							Delete
						</button>
						<button
							class="btn btn-primary"
							type="submit"
							:disabled="form.mutation.isPending.value"
						>
							{{ view ? 'Save' : 'Create' }} View
						</button>
					</div>
				</form>
			</div>
			<form method="dialog" class="modal-backdrop">
				<button>close</button>
			</form>
		</dialog>
	</Teleport>
</template>

<script setup lang="ts">
import { z } from 'zod'
import type { IView } from '~~/server/db/schema'

const props = defineProps<{
	view?: SerializeObject<IView> | null
	open?: boolean
}>()

const emit = defineEmits<{
	'update:open': [boolean]
}>()

const modal = ref<HTMLDialogElement>()
const createView = useCreateViewMutation()
const updateView = useUpdateViewMutation()
const deleteView = useDeleteViewMutation()

const initialValues = {
	name: '',
	filters: {
		tagIds: [],
	},
}
const form = useForm({
	schema: z.object({
		name: z.string().min(1, 'Name is required'),
		filters: z.object({
			tagIds: z.array(z.string()),
		}),
	}),
	initialValues: structuredClone(initialValues),
	async onSubmit(values) {
		if (props.view) {
			await updateView.mutateAsync({
				id: props.view.id,
				...values,
			})
		} else {
			await createView.mutateAsync(values)
		}
		modal.value?.close()
	},
})

async function handleDelete() {
	if (!props.view) return
	await deleteView.mutateAsync({ id: props.view.id })
	modal.value?.close()
}

watch(
	() => props.open,
	newValue => {
		if (newValue && modal.value && !modal.value.open) {
			if (props.view) {
				form.setValues({
					name: props.view.name,
					filters: props.view.filters as any,
				})
			} else {
				form.setValues(structuredClone(initialValues))
			}
			modal.value.showModal()
		} else if (!newValue && modal.value?.open) {
			modal.value.close()
		}
	}
)

function onModalClose() {
	emit('update:open', false)
}

onMounted(() => {
	if (props.open && modal.value) {
		modal.value.showModal()
	}
	modal.value?.addEventListener('close', onModalClose)
})
</script>
