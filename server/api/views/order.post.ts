import { z } from 'zod'
import { and, eq, inArray, sql } from 'drizzle-orm'
import { db } from '~~/server/db'
import { View } from '~~/server/db/schema'
import { wsPeerManager } from '../ws'

const schema = z.object({
	viewIds: z.array(z.string()),
})

export default defineEventHandler(async event => {
	const authData = await mustGetAuthData(event)
	const body = await readValidatedBodyEx(event, schema)

	// Verify all views exist and belong to the user
	const existingViews = await db
		.select({ id: View.id })
		.from(View)
		.where(and(eq(View.userId, authData.user.id), inArray(View.id, body.viewIds)))

	if (existingViews.length !== body.viewIds.length) {
		throw createError({
			statusCode: 400,
			message: 'Invalid view IDs',
		})
	}

	await db.execute(sql`
		WITH update_values (id, new_order) AS (
			(VALUES ${sql.join(
				// At this point body.viewIds is not an injection risk because
				// we've already verified that all IDs are valid
				body.viewIds.map((id, index) => sql`(${id}, ${index}::integer)`),
				sql.raw(', ')
			)})
		)
		UPDATE views
		SET "order" = update_values.new_order
		FROM update_values
		WHERE views.id = update_values.id
	`)

	wsPeerManager.sendDataChangedEvent(authData.user.id)

	return true
})
