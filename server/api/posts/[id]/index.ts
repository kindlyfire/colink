import { and, eq } from 'drizzle-orm'
import { db } from '~~/server/db'
import { Post } from '~~/server/db/schema'
import { scrapingManager } from '~~/server/scraper/manager'

export default defineEventHandler(async event => {
	assertRequestMethod(event, 'GET')
	const authData = await mustGetAuthData(event)

	const [post] = await db
		.select()
		.from(Post)
		.where(
			and(eq(Post.id, getRouterParam(event, 'id') || ''), eq(Post.userId, authData.user.id))
		)
	assertResource(post)

	return scrapingManager.augmentPost(post)
})
