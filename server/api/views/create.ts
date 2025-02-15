import { z } from 'zod'
import { db } from '~~/server/db'
import { View } from '~~/server/db/schema'
import { wsPeerManager } from '../ws'

const schema = z.object({
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
		.insert(View)
		.values({
			userId: authData.user.id,
			filters: body.filters,
		})
		.returning()

	wsPeerManager.sendDataChangedEvent(authData.user.id)

	return view
})
