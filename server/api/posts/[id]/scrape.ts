import { and, eq } from 'drizzle-orm'
import { db } from '~~/server/db'
import { Post } from '~~/server/db/schema'
import { scrapingManager } from '~~/server/scraper/manager'

export default defineEventHandler(async event => {
	console.log(event.path)
	ensureRequestMethod(event, 'POST')
	const authData = await mustGetAuthData(event)
	const [post] = await db
		.select()
		.from(Post)
		.where(
			and(eq(Post.userId, authData.user.id), eq(Post.id, getRouterParam(event, 'id') || ''))
		)
	if (!post) {
		throw createError({
			statusCode: 404,
		})
	}

	scrapingManager.addToQueue(post)

	return true
})
