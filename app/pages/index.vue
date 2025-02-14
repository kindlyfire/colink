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
	<div class="mt-4">
		<div class="card bg-base-200">
			<table class="table">
				<thead>
					<tr>
						<!-- <th>ID</th> -->
						<th>Title</th>
						<!-- <th>Type</th> -->
						<th>URL</th>
						<th>Created At</th>
						<th></th>
					</tr>
				</thead>
				<tbody>
					<tr v-for="post in posts ?? []" :key="post.id">
						<!-- <td>{{ post.id }}</td> -->
						<td>{{ post.title }}</td>
						<td>{{ post.url }}</td>
						<td>{{ post.createdAt }}</td>
						<td>{{ JSON.stringify(post.scrapeProgress || '"no progress"') }}</td>
						<td>
							<button class="btn" @click="mScrapePost.mutate({ id: post.id })">
								Scrape
							</button>
						</td>
					</tr>
				</tbody>
			</table>
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

const mScrapePost = useScrapePostMutation()
</script>
