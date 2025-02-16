import { z } from 'zod'
import { db } from '~~/server/db'
import { View } from '~~/server/db/schema'
import { wsPeerManager } from '../../ws'
import { and, eq } from 'drizzle-orm'

const schema = z.object({
	name: z.string().optional(),
	filters: z
		.object({
			tagIds: z.array(z.string()),
		})
		.nullable(),
})

export default defineEventHandler(async event => {
	assertRequestMethod(event, 'POST')
	const authData = await mustGetAuthData(event)
	const body = await readValidatedBodyEx(event, schema)

	const [view] = await db
		.update(View)
		.set({
			name: body.name,
			filters: body.filters,
		})
		.where(
			and(eq(View.id, getRouterParam(event, 'id') || ''), eq(View.userId, authData.user.id))
		)
		.returning()
	assertResource(view)

	wsPeerManager.sendDataChangedEvent(authData.user.id)

	return view
})
