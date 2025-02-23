import { z } from 'zod'
import { db } from '~~/server/db'
import { Post } from '~~/server/db/schema'
import { wsPeerManager } from '../../ws'
import { and, eq } from 'drizzle-orm'

const schema = z.object({
	titleOverride: z.string().nullable(),
})

export default defineEventHandler(async event => {
	const authData = await mustGetAuthData(event)
	const body = await readValidatedBodyEx(event, schema)

	const [post] = await db
		.update(Post)
		.set({
			titleOverride: body.titleOverride,
		})
		.where(
			and(eq(Post.id, getRouterParam(event, 'id') || ''), eq(Post.userId, authData.user.id))
		)
		.returning()
	assertResource(post)

	wsPeerManager.sendDataChangedEvent(authData.user.id)
	await indexingManager.index.post(post.id)

	return post
})
