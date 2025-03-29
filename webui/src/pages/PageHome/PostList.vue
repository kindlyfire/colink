<template>
	<div class="[&>div:last-child]:border-b-0">
		<div v-for="post in posts?.data" class="border-b-2 border-gray-200 py-2">
			<div class="text-xs text-gray-500">{{ formatDate(post.created_at) }}</div>
			<div>{{ post.text }}</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { api } from '../../utils/vue-query/api'
import { formatDate } from '../../utils/date'

const qPosts = api.posts.useList()
const posts = qPosts.data
</script>
