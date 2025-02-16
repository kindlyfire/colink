import { z } from 'zod'
import { db } from '~~/server/db'
import { View } from '~~/server/db/schema'
import { wsPeerManager } from '../ws'

const schema = z.object({
	name: z.string(),
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
			name: body.name,
			filters: body.filters,
		})
		.returning()

	wsPeerManager.sendDataChangedEvent(authData.user.id)

	return view
})
