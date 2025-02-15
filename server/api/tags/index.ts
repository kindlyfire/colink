import { desc, eq } from 'drizzle-orm'
import { db } from '~~/server/db'
import { Tag } from '~~/server/db/schema'

export default defineEventHandler(async event => {
	assertRequestMethod(event, 'GET')
	const authData = await mustGetAuthData(event)

	const tags = await db
		.select()
		.from(Tag)
		.where(eq(Tag.userId, authData.user.id))
		.orderBy(desc(Tag.createdAt))

	return tags
})
