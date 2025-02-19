<template>
	<div class="pr-[27rem]">
		<div v-if="post" class="max-w-[40rem] mx-auto">
			<h1 class="text-3xl font-bold mb-4">{{ post.title }}</h1>
			<div class="prose prose-neutral max-w-none" v-html="post.html"></div>

			<div v-if="post.url" class="mt-4">
				<a :href="post.url" target="_blank" rel="noopener noreferrer" class="link">
					View original source →
				</a>
			</div>
		</div>
		<div v-else-if="qPost.isLoading.value" class="text-center py-8">Loading...</div>
		<div v-else class="text-center py-8 text-error">Post not found</div>

		<ASidebar side="right" class="w-[27rem]"> <div class="p-2">My sidebar</div> </ASidebar>
	</div>
</template>

<script lang="ts" setup>
const route = useRoute()
const id = computed(() => route.params.id as string)

const qPost = usePostQuery(id)
const post = computed(() => qPost.data.value)

// Prefetch data on server
if (import.meta.server) {
	await qPost.suspense()
}
</script>
