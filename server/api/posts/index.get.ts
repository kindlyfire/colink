import { desc, eq } from 'drizzle-orm'
import { db } from '~~/server/db'
import { Post } from '~~/server/db/schema'
import { scrapingManager } from '~~/server/scraper/manager'

export default defineEventHandler(async event => {
	const authData = await mustGetAuthData(event)

	const posts = await db
		.select()
		.from(Post)
		.where(eq(Post.userId, authData.user.id))
		.orderBy(desc(Post.createdAt))

	return posts.map(p => scrapingManager.augmentPost(p))
})
