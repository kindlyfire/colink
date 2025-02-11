import { eq } from 'drizzle-orm'
import { createError } from 'h3'
import { z } from 'zod'
import { db } from '~~/server/db'
import { User } from '~~/server/db/schema'

const schema = z.object({
	username: z.string(),
	password: z.string(),
})

export default defineEventHandler(async event => {
	ensureRequestMethod(event, 'POST')
	const body = await readValidatedBodyEx(event, schema)

	const [user] = await db.select().from(User).where(eq(User.username, body.username))

	if (!user) {
		throw createError({ statusCode: 401, statusMessage: 'User not found' })
	}
	if (!user.password) {
		throw createError({ statusCode: 401, statusMessage: 'Invalid username or password' })
	}

	const passwordValid = await Bun.password.verify(body.password, user.password)
	if (!passwordValid) {
		throw createError({ statusCode: 401, statusMessage: 'Invalid username or password' })
	}

	await assignSession(event, user.id)

	return {
		id: user.id,
		username: user.username,
	}
})
