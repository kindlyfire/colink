import { and, eq } from 'drizzle-orm'
import { db } from '~~/server/db'
import { Tag } from '~~/server/db/schema'
import { wsPeerManager } from '../../ws'

export default defineEventHandler(async event => {
	assertRequestMethod(event, 'POST')
	const authData = await mustGetAuthData(event)

	const [tag] = await db
		.delete(Tag)
		.where(and(eq(Tag.id, getRouterParam(event, 'id') || ''), eq(Tag.userId, authData.user.id)))
		.returning()
	assertResource(tag)

	wsPeerManager.sendDataChangedEvent(authData.user.id)

	return tag
})
