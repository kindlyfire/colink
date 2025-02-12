import { H3Event } from 'h3'
import z from 'zod'
import { db } from '../db'
import { Session, User } from '../db/schema'
import { eq } from 'drizzle-orm'
import dayjs from 'dayjs'

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
	setCookie(event, getSessionCookieName(event), sessionToken, {
		httpOnly: true,
		secure: getRequestProtocol(event) === 'https',
		sameSite: 'strict',
		path: '/',
	})
	return sessionToken
}

export function getSessionCookieName(event: H3Event) {
	const secure = getRequestProtocol(event) === 'https'
	return (secure ? '__Secure-' : '') + 'session'
}

export async function readSession(event: H3Event) {
	return getCookie(event, getSessionCookieName(event))
}

export async function getAuthData(event: H3Event) {
	const sessionToken = await readSession(event)
	if (!sessionToken) return null
	const res = await db
		.select()
		.from(Session)
		.innerJoin(User, eq(Session.userId, User.id))
		.where(eq(Session.token, sessionToken))
	if (res.length === 0) return null

	// Update lastSeen time
	const session = res[0].sessions
	if (session.lastSeen < dayjs().subtract(5, 'minutes').toDate()) {
		db.update(Session)
			.set({ lastSeen: new Date() })
			.where(eq(Session.id, session.id))
			.catch(() => {
				// TODO: Switch to structured logging
				console.error('Failed to update session lastSeen time', session.id)
			})
	}

	return {
		user: res[0].users,
		session,
	}
}

export async function mustGetAuthData(event: H3Event) {
	const authData = await getAuthData(event)
	if (!authData) {
		throw createError({ statusCode: 401, message: 'Unauthorized' })
	}
	return authData
}
