import { and, eq } from 'drizzle-orm'
import { db } from '~~/server/db'
import { Post } from '~~/server/db/schema'
import { wsPeerManager } from '../../ws'

export default defineEventHandler(async event => {
	assertRequestMethod(event, 'POST')
	const authData = await mustGetAuthData(event)

	const [post] = await db
		.delete(Post)
		.where(
			and(eq(Post.id, getRouterParam(event, 'id') || ''), eq(Post.userId, authData.user.id))
		)
		.returning()
	assertResource(post)

	wsPeerManager.sendDataChangedEvent(authData.user.id)

	return post
})
