import { useMutation } from '@tanstack/vue-query'
import { z } from 'zod'
import { getByPath, setByPath, type Path } from './dot-path-value'
import { computed, reactive, toRef } from 'vue'

interface UseFormOptions<TSchema extends z.ZodTypeAny> {
	schema: TSchema
	validateAsync?: (values: z.output<TSchema>, ctx: z.RefinementCtx) => Promise<any>
	initialValues: z.input<TSchema>
	onSubmit?: (values: z.output<TSchema>) => any
	clone?: (values: z.input<TSchema>) => z.input<TSchema>
}

export function useForm<TSchema extends z.ZodTypeAny>(options: UseFormOptions<TSchema>) {
	type TInput = z.input<TSchema>
	const clone = options.clone ?? structuredClone

	const state = reactive({
		values: clone(options.initialValues) as TInput,
		errors: [] as z.ZodIssue[],
	})

	function validatePath(path: string) {
		const result = options.schema.safeParse(state.values)
		const newErrors = result.error?.errors.filter(error => error.path.join('.') === path)
		state.errors = state.errors
			.filter(error => error.path.join('.') !== path)
			.concat(newErrors ?? [])
	}

	function getInputProps<T extends Path<TInput>>(name: T) {
		const errors = computed(() => state.errors.filter(error => error.path.join('.') === name))
		return {
			modelValue: getByPath(state.values, name),
			'onUpdate:modelValue': (value: any) => {
				setByPath(state.values, name, value)
				if (errors.value.length) validatePath(name)
			},
			onBlur: () => {
				validatePath(name)
			},
			errors,
		}
	}

	function onSubmit(e?: Event) {
		if (e) {
			e.preventDefault()
		}

		const result = options.schema.safeParse(state.values)
		if (!result.success) {
			console.log('Form errors:', result.error.errors)
			state.errors = result.error.errors
			return
		}
		mutation.mutate(result.data)
	}

	function setValues(values: TInput) {
		state.values = values
	}

	function setValue<T extends Path<TInput>>(name: T, value: TInput[T]) {
		setByPath(state.values, name, value)
	}

	function setErrors(errors: z.ZodIssue[]) {
		state.errors = errors
	}

	const mutation = useMutation({
		async mutationFn(data: z.output<TSchema>) {
			if (options.validateAsync) {
				const result = await options.schema
					.superRefine(options.validateAsync!)
					.safeParseAsync(state.values)
				if (result.error) {
					console.log('Form errors:', result.error.errors)
					state.errors = result.error.errors
					return
				}
			}
			await options.onSubmit?.(data)
		},
	})

	return {
		values: toRef(state, 'values'),
		errors: toRef(state, 'errors'),
		setValues,
		setValue,
		getInputProps,
		onSubmit,
		setErrors,
		mutation,
	}
}
