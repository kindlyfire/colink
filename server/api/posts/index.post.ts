import { z } from 'zod'
import { db } from '~~/server/db'
import { Post } from '~~/server/db/schema'
import { wsPeerManager } from '../ws'
import { scrapingManager } from '~~/server/scraper/manager'

const schema = z.object({
	url: z.string().url(),
})

export default defineEventHandler(async event => {
	const authData = await mustGetAuthData(event)
	const body = await readValidatedBodyEx(event, schema)

	const [post] = await db
		.insert(Post)
		.values({
			userId: authData.user.id,
			type: 'link',
			html: '',
			url: body.url,
			title: '',
			scrapeState: {
				pending: true,
			},
		})
		.returning()

	wsPeerManager.sendDataChangedEvent(authData.user.id)
	scrapingManager.addToQueue(post)

	return post
})
