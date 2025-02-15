import { z } from 'zod'
import { db } from '~~/server/db'
import { Tag } from '~~/server/db/schema'
import { wsPeerManager } from '../ws'

const schema = z.object({
	name: z.string().nonempty(),
})

export default defineEventHandler(async event => {
	assertRequestMethod(event, 'POST')
	const authData = await mustGetAuthData(event)
	const body = await readValidatedBodyEx(event, schema)

	const [tag] = await db
		.insert(Tag)
		.values({
			userId: authData.user.id,
			name: body.name,
		})
		.returning()

	wsPeerManager.sendDataChangedEvent(authData.user.id)

	return tag
})
