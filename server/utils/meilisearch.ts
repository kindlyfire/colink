import { MeiliSearch } from 'meilisearch'
import { db } from '../db'
import { Post, PostTag } from '../db/schema'
import { eq } from 'drizzle-orm'

const runtimeConfig = useRuntimeConfig()

interface IndexedPost {
	id: string
	tags: string[]
	title: string
	text: string
	createdAt: number
}

class IndexingManager {
	client = new MeiliSearch(runtimeConfig.meilisearch)
	postsIndex = this.client.index('posts')

	index = {
		// TODO: Could turn this into postIds: string[] to avoid making a
		// request per post when deleting a tag (which reindexes all posts with
		// that tag)
		post: async (postId: string) => {
			const res = await db
				.select()
				.from(Post)
				.leftJoin(PostTag, eq(Post.id, PostTag.postId))
				.where(eq(Post.id, postId))

			if (!res.length) {
				throw new Error(`Post with ID ${postId} not found`)
			}

			const post = res[0].posts
			const tags = res.map(r => r.post_tags).filter(v => v != null)

			const indexedPost: IndexedPost = {
				id: post.id,
				tags: tags.map(t => t.tagId),
				title: post.titleOverride || post.title,
				text: post.text || '',
				createdAt: post.createdAt.getTime(),
			}
			const task = await this.postsIndex.addDocuments([indexedPost])
			await this.client.waitForTask(task.taskUid)
		},

		postDeleted: async (postId: string) => {
			await this.postsIndex.deleteDocument(postId)
		},
	}

	/**
	 * Set up or update the MeiliSearch indexes.
	 */
	async migrate() {
		await this.client.createIndex('posts')
		await this.postsIndex.updateSettings({
			filterableAttributes: ['tags'],
			searchableAttributes: ['text'],
		})
	}
}

export const indexingManager = new IndexingManager()
