<template>
	<fieldset class="fieldset">
		<legend v-if="label" class="fieldset-legend">{{ label }}</legend>
		<label class="input">
			<input v-if="type !== 'textarea'" class="" :type="type" v-bind="inputPropsBind" />
			<textarea v-else class="" v-bind="inputPropsBind" />
		</label>
		<div v-if="errors?.length" class="text-sm text-error">
			<div v-for="err in errors">
				{{ err.message }}
			</div>
		</div>
	</fieldset>
</template>

<script setup lang="ts">
import type { z } from 'zod'

defineOptions({
	inheritAttrs: false,
})

const props = defineProps<{
	modelValue?: string | number
	type?: 'text' | 'password' | 'email' | 'number' | 'tel' | 'url' | 'textarea'
	placeholder?: string
	label?: string
	class?: string
	rows?: number
	maxlength?: number
	required?: boolean
	disabled?: boolean
	errors?: MaybeRef<z.ZodIssue[]>
}>()
const emit = defineEmits<{
	'update:modelValue': [string]
	blur: []
}>()

const errors = computed(() => unref(props.errors))

const inputPropsBind = computed((): any => {
	return {
		value: props.modelValue,
		placeholder: props.placeholder,
		rows: props.rows,
		maxlength: props.maxlength,
		required: props.required,
		disabled: props.disabled,
		class: props.class,
		onInput: (e: InputEvent) => emit('update:modelValue', (e.target as HTMLInputElement).value),
		onBlur: () => emit('blur'),
	}
})
</script>
