import { H3Event } from 'h3'
import z from 'zod'
import { db } from '../db'
import { Session } from '../db/schema'

export async function readValidatedBodyEx<T extends z.ZodType>(
	event: H3Event,
	schema: T
): Promise<z.infer<T>> {
	const contentType = getRequestHeader(event, 'content-type')
	if (!contentType?.includes('application/json')) {
		throw createError({
			statusCode: 415,
			message: 'Content-Type must be application/json',
		})
	}

	const body = await readBody(event)
	const result = schema.safeParse(body)
	if (!result.success) {
		throw createError({
			statusCode: 400,
			message: 'Validation error',
			data: result.error.errors,
		})
	}
	return result.data
}

export function ensureRequestMethod(event: H3Event, method: string) {
	if (event.method !== method) {
		throw createError({ statusCode: 405, message: 'Method Not Allowed' })
	}
}

export async function assignSession(event: H3Event, userId: string) {
	const sessionToken = crypto.randomUUID()
	await db.insert(Session).values({
		userId,
		token: sessionToken,
	})
	const secure = getRequestProtocol(event) === 'https'
	setCookie(event, (secure ? '__Secure-' : '') + 'session', sessionToken, {
		httpOnly: true,
		secure,
		sameSite: 'strict',
		path: '/',
	})
	return sessionToken
}

export async function readSession(event: H3Event) {
	const secure = getRequestProtocol(event) === 'https'
	return getCookie(event, (secure ? '__Secure-' : '') + 'session')
}
