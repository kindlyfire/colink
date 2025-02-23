import { and, eq } from 'drizzle-orm'
import { db } from '~~/server/db'
import { View } from '~~/server/db/schema'
import { wsPeerManager } from '../../ws'

export default defineEventHandler(async event => {
	const authData = await mustGetAuthData(event)

	const [view] = await db
		.delete(View)
		.where(
			and(eq(View.id, getRouterParam(event, 'id') || ''), eq(View.userId, authData.user.id))
		)
		.returning()
	assertResource(view)

	wsPeerManager.sendDataChangedEvent(authData.user.id)

	return view
})
