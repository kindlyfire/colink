import { z } from 'zod'
import { db } from '~~/server/db'
import { Tag } from '~~/server/db/schema'
import { wsPeerManager } from '../../ws'
import { and, eq } from 'drizzle-orm'

const schema = z.object({
	name: z.string().nonempty(),
})

export default defineEventHandler(async event => {
	assertRequestMethod(event, 'POST')
	const authData = await mustGetAuthData(event)
	const body = await readValidatedBodyEx(event, schema)

	const [tag] = await db
		.update(Tag)
		.set({
			name: body.name,
		})
		.where(and(eq(Tag.id, getRouterParam(event, 'id') || ''), eq(Tag.userId, authData.user.id)))
		.returning()
	assertResource(tag)

	wsPeerManager.sendDataChangedEvent(authData.user.id)

	return tag
})
