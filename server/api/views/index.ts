import { desc, eq } from 'drizzle-orm'
import { db } from '~~/server/db'
import { View } from '~~/server/db/schema'

export default defineEventHandler(async event => {
	assertRequestMethod(event, 'GET')
	const authData = await mustGetAuthData(event)

	const views = await db
		.select()
		.from(View)
		.where(eq(View.userId, authData.user.id))
		.orderBy(desc(View.createdAt))

	return views
})
