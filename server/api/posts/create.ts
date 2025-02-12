import { z } from 'zod'
import { db } from '~~/server/db'
import { Post } from '~~/server/db/schema'

const schema = z.object({
	url: z.string().url(),
})

export default defineEventHandler(async event => {
	ensureRequestMethod(event, 'POST')
	const authData = await mustGetAuthData(event)
	const body = await readValidatedBodyEx(event, schema)

	const [post] = await db
		.insert(Post)
		.values({
			userId: authData.user.id,
			type: 'link',
			html: '',
			url: body.url,
		})
		.returning()

	return post
})
