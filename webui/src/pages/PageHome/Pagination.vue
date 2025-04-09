<template>
	<div class="flex gap-1 items-center justify-center">
		<button class="btn btn-xs" @click="goToPrev" :disabled="props.state.page <= 0">Prev</button>

		<button
			v-for="pageNum in pagesBeforeCurrent"
			:key="`before-${pageNum}`"
			@click="goToPage(pageNum)"
			class="btn btn-xs"
		>
			{{ pageNum + 1 }}
		</button>

		<input
			v-model.number="currentPage"
			@keyup.enter="handlePageInput"
			class="input input-xs w-[3rem] text-center"
			:min="1"
			:max="totalPages"
		/>

		<button
			v-for="pageNum in pagesAfterCurrent"
			:key="`after-${pageNum}`"
			@click="goToPage(pageNum)"
			class="btn btn-xs"
		>
			{{ pageNum + 1 }}
		</button>

		<button class="btn btn-xs" @click="goToNext" :disabled="props.state.page >= totalPages - 1">
			Next
		</button>
	</div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'

const props = defineProps<{
	state: {
		page: number
		limit: number
		postsCount: number
	}
}>()

const totalPages = computed(() => {
	return Math.ceil(props.state.postsCount / props.state.limit)
})

const currentPage = ref(props.state.page + 1)
watch(
	() => props.state.page,
	newPage => {
		currentPage.value = newPage + 1
	}
)

// Navigation functions
const goToPrev = () => {
	if (props.state.page > 0) {
		props.state.page--
	}
}
const goToNext = () => {
	if (props.state.page < totalPages.value) {
		props.state.page++
	}
}
const goToPage = (page: number) => {
	props.state.page = page
}

const handlePageInput = () => {
	let page = currentPage.value - 1
	if (page < 0) page = 0
	if (page >= totalPages.value) page = totalPages.value - 1
	props.state.page = page
}

// Compute pages to show before current page (up to 3)
const pagesBeforeCurrent = computed(() => {
	const pages: number[] = []
	const start = Math.max(0, props.state.page - 3)

	for (let i = start; i < props.state.page; i++) {
		pages.push(i)
	}

	return pages
})

// Compute pages to show after current page (up to 3)
const pagesAfterCurrent = computed(() => {
	const pages: number[] = []
	const end = Math.max(Math.min(totalPages.value - 1, props.state.page + 3), 0)

	for (let i = props.state.page + 1; i <= end; i++) {
		pages.push(i)
	}

	return pages
})
</script>
