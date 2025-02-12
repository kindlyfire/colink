<template>
	<div>
		<form @submit="onSubmit" class="flex gap-2">
			<input
				v-model="formUrl"
				class="input w-full"
				placeholder="Save something..."
				autofocus
				:disabled="mCreatePost.isPaused.value"
			/>
			<button class="btn btn-info btn-soft" :disabled="mCreatePost.isPaused.value">
				Save
			</button>
		</form>
	</div>
	<div>
		<div v-for="post in posts ?? []">
			{{ post.url }}
		</div>
	</div>
</template>

<script lang="ts" setup>
const formUrl = ref('')

const mCreatePost = useCreatePostMutation()
async function onSubmit(e: Event) {
	e.preventDefault()
	const url = formUrl.value.trim()
	if (url) {
		await mCreatePost.mutateAsync({ url })
		formUrl.value = ''
	}
}

const qPosts = usePostsQuery()
const posts = qPosts.data

if (import.meta.server) {
	await qPosts.suspense()
}
</script>
