import { and, eq } from 'drizzle-orm'
import { db } from '~~/server/db'
import { Post, PostTag, Tag } from '~~/server/db/schema'
import { wsPeerManager } from '../../ws'

export default defineEventHandler(async event => {
	const authData = await mustGetAuthData(event)

	const [tag] = await db
		.select()
		.from(Tag)
		.where(and(eq(Tag.userId, authData.user.id), eq(Tag.id, getRouterParam(event, 'id') || '')))
	assertResource(tag)

	const postsWithTag = await db
		.select({
			id: Post.id,
		})
		.from(Post)
		.innerJoin(PostTag, eq(PostTag.postId, Post.id))
		.where(eq(PostTag.tagId, tag.id))

	const [_tag] = await db
		.delete(Tag)
		.where(and(eq(Tag.id, getRouterParam(event, 'id') || ''), eq(Tag.userId, authData.user.id)))
		.returning()
	assertResource(_tag)

	wsPeerManager.sendDataChangedEvent(authData.user.id)
	await Promise.all(postsWithTag.map(post => indexingManager.index.post(post.id)))

	return _tag
})
