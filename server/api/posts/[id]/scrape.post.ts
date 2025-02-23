import { and, eq } from 'drizzle-orm'
import { db } from '~~/server/db'
import { Post } from '~~/server/db/schema'
import { scrapingManager } from '~~/server/scraper/manager'

export default defineEventHandler(async event => {
	const authData = await mustGetAuthData(event)

	const [post] = await db
		.select()
		.from(Post)
		.where(
			and(eq(Post.userId, authData.user.id), eq(Post.id, getRouterParam(event, 'id') || ''))
		)
	assertResource(post)

	scrapingManager.addToQueue(post)

	return true
})
