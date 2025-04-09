<template>
	<Pagination v-if="pageCount > 1" :state="pagination" />
	<div class="[&>div:last-child]:border-b-0">
		<div v-for="post in posts?.data" class="border-b-2 border-gray-200 py-2">
			<div class="text-xs text-gray-500">{{ formatDate(post.created_at) }}</div>
			<div>{{ post.text }}</div>
			<div v-if="post.links.length" class="pt-1"></div>
			<a
				v-for="link in post.links"
				:key="link.id"
				:href="link.url"
				class="flex items-center gap-1 text-sm text-gray-600 hover:underline hover:text-gray-800"
				target="_blank"
			>
				<!-- TODO: Put icons in an AIcon component -->
				<svg xmlns="http://www.w3.org/2000/svg" width="1rem" viewBox="0 0 256 256">
					<path
						fill="currentColor"
						d="M240 88.23a54.43 54.43 0 0 1-16 37L189.25 160a54.27 54.27 0 0 1-38.63 16h-.05A54.63 54.63 0 0 1 96 119.84a8 8 0 0 1 16 .45A38.62 38.62 0 0 0 150.58 160a38.4 38.4 0 0 0 27.31-11.31l34.75-34.75a38.63 38.63 0 0 0-54.63-54.63l-11 11A8 8 0 0 1 135.7 59l11-11a54.65 54.65 0 0 1 77.3 0a54.86 54.86 0 0 1 16 40.23m-131 97.43l-11 11A38.4 38.4 0 0 1 70.6 208a38.63 38.63 0 0 1-27.29-65.94L78 107.31a38.63 38.63 0 0 1 66 28.4a8 8 0 0 0 16 .45A54.86 54.86 0 0 0 144 96a54.65 54.65 0 0 0-77.27 0L32 130.75A54.62 54.62 0 0 0 70.56 224a54.28 54.28 0 0 0 38.64-16l11-11a8 8 0 0 0-11.2-11.34"
					/>
				</svg>
				{{ link.url }}
			</a>
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
