import { createError } from 'h3'
import { z } from 'zod'
import { db } from '~~/server/db'
import { User } from '~~/server/db/schema'

const schema = z.object({
	username: z.string().min(3),
	password: z.string().min(6),
})

export default defineEventHandler(async event => {
	ensureRequestMethod(event, 'POST')

	const runtimeConfig = useRuntimeConfig()
	if (!runtimeConfig.registrationsEnabled) {
		throw createError({
			statusCode: 403,
			message: 'Registrations are disabled',
		})
	}

	const body = await readValidatedBodyEx(event, schema)
	const hashedPassword = await Bun.password.hash(body.password)

	try {
		var [newUser] = await db
			.insert(User)
			.values({
				username: body.username,
				password: hashedPassword,
			})
			.returning()
	} catch (error: any) {
		// TODO: Actually check for the unique constraint violation
		throw createError({
			statusCode: 409,
			message: 'Username already taken',
		})
	}

	await assignSession(event, newUser.id)

	return {
		id: newUser.id,
		username: newUser.username,
	}
})
