<template>
	<PostDisplayCards :posts="qPosts.data.value ?? []" />
</template>

<script lang="ts" setup>
import type { IView } from '~~/server/db/schema'
import PostDisplayCards from './PostDisplayCards.vue'

const props = defineProps<{
	filters: IView['filters']
}>()

const qPosts = usePostsQuery(toRef(props, 'filters'))

if (import.meta.server) {
	await qPosts.suspense()
}
</script>
