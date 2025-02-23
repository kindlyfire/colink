<template>
	<div class="grid gap-4 grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
		<div v-for="post in posts" :key="post.id" class="card bg-base-200">
			<div class="card-body">
				<h2 class="card-title">
					{{ post.titleOverride || post.title }}
				</h2>
				<div class="card-actions items-end grow">
					<div class="flex items-center w-full gap-2">
						<a
							v-if="post.url"
							:href="post.url"
							target="_blank"
							class="overflow-ellipsis overflow-x-hidden mr-auto"
						>
							{{ getDomainName(post.url) }}
						</a>
						<div
							class="flex items-center tooltip"
							data-tip=""
							v-if="post.scrapeProgress"
						>
							<div class="tooltip-content">
								<div>{{ post.scrapeProgress.state }}</div>
							</div>
							<span class="loading loading-spinner loading-xs"></span>
						</div>
						<NuxtLink :to="`/p/${post.id}`" class="btn btn-soft btn-sm">
							Open
						</NuxtLink>
						<div class="dropdown dropdown-end">
							<button tabindex="0" class="btn btn-soft btn-sm btn-square">
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="1em"
									height="1em"
									viewBox="0 0 256 256"
								>
									<path
										fill="currentColor"
										d="M224 128a8 8 0 0 1-8 8H40a8 8 0 0 1 0-16h176a8 8 0 0 1 8 8M40 72h176a8 8 0 0 0 0-16H40a8 8 0 0 0 0 16m176 112H40a8 8 0 0 0 0 16h176a8 8 0 0 0 0-16"
									/>
								</svg>
							</button>
							<ul
								tabindex="0"
								class="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-52"
							>
								<li>
									<button
										@click="mScrape.mutate({ id: post.id })"
										:disabled="!!post.scrapeProgress"
									>
										Refresh
									</button>
								</li>
								<li>
									<button
										@click="deletePost(post.id)"
										:disabled="!!post.scrapeProgress"
									>
										Delete
									</button>
								</li>
							</ul>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</template>

<script lang="ts" setup>
import type { IPostWithProgress } from '~~/server/db/schema'

const props = defineProps<{
	posts: SerializeObject<IPostWithProgress>[]
}>()

const getDomainName = (url: string) => new URL(url).hostname

const mScrape = useScrapePostMutation()
const mDeletePost = useDeletePostMutation()

function deletePost(id: string) {
	if (confirm('Are you sure you want to delete this post?')) {
		mDeletePost.mutate({ id })
	}
}
</script>
