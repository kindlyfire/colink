<template>
	<Pagination v-if="pageCount > 1" :state="pagination" />
	<div class="[&>div:last-child]:border-b-0">
		<div v-for="post in posts?.data" class="border-b-2 border-gray-200 py-2">
			<div class="text-xs text-gray-500">{{ formatDate(post.created_at) }}</div>
			<div>{{ post.text }}</div>
		</div>
	</div>
	<Pagination v-if="pageCount > 1" :state="pagination" />
</template>

<script setup lang="ts">
import { api } from '../../utils/vue-query/api'
import { formatDate } from '../../utils/date'
import { computed, reactive, watch } from 'vue'
import Pagination from './Pagination.vue'

const pagination = reactive({
	page: 0,
	limit: 5,
	postsCount: 0,
})
const qPosts = api.posts.useList(
	computed(() => ({
		limit: pagination.limit,
		offset: pagination.page * pagination.limit,
	}))
)
const posts = qPosts.data

const pageCount = computed(() => {
	return Math.ceil((posts.value?.total || 0) / pagination.limit)
})

watch(
	() => posts.value,
	newPosts => {
		if (newPosts) {
			pagination.postsCount = newPosts.total
		}
	}
)
</script>
