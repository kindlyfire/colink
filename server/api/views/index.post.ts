import { z } from 'zod'
import { db } from '~~/server/db'
import { View } from '~~/server/db/schema'
import { wsPeerManager } from '../ws'
import { sql } from 'drizzle-orm'

const schema = z.object({
	name: z.string(),
	filters: z
		.object({
			tagIds: z.array(z.string()),
		})
		.nullable(),
})

export default defineEventHandler(async event => {
	const authData = await mustGetAuthData(event)
	const body = await readValidatedBodyEx(event, schema)

	const [view] = await db
		.insert(View)
		.values({
			userId: authData.user.id,
			name: body.name,
			filters: body.filters,
			order: sql`COALESCE((SELECT MAX("order") FROM "views" WHERE "userId" = ${authData.user.id}), 0) + 1`,
		})
		.returning()

	wsPeerManager.sendDataChangedEvent(authData.user.id)

	return view
})
