import { asc, eq } from 'drizzle-orm'
import { db } from '~~/server/db'
import { View } from '~~/server/db/schema'

export default defineEventHandler(async event => {
	const authData = await mustGetAuthData(event)

	const views = await db
		.select()
		.from(View)
		.where(eq(View.userId, authData.user.id))
		.orderBy(asc(View.order))

	return views
})
