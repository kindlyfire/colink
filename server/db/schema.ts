import { boolean, integer, jsonb, pgTable, text, timestamp, unique } from 'drizzle-orm/pg-core'
import { createId } from '@paralleldrive/cuid2'
import { relations } from 'drizzle-orm'

const defaultColumns = {
	id: text().primaryKey().$defaultFn(createId),
	createdAt: timestamp().notNull().defaultNow(),
	updatedAt: timestamp()
		.notNull()
		.defaultNow()
		.$onUpdateFn(() => new Date()),
}

export const User = pgTable('users', {
	...defaultColumns,
	username: text().unique().notNull(),
	password: text().notNull(),
})
export type IUser = typeof User.$inferSelect

export const Session = pgTable('sessions', {
	...defaultColumns,
	userId: text()
		.notNull()
		.references(() => User.id, { onDelete: 'cascade' }),
	token: text().unique().notNull(),
	lastSeen: timestamp().notNull().defaultNow(),
	label: text(),
})
export type ISession = typeof Session.$inferSelect

export const Post = pgTable(
	'posts',
	{
		...defaultColumns,
		userId: text()
			.notNull()
			.references(() => User.id, { onDelete: 'cascade' }),
		type: text().notNull().$type<'link' | 'note'>(),
		html: text(),
		text: text(),
		url: text(),
		title: text().notNull(),
		titleOverride: text(),
		scrapeState: jsonb().$type<null | {
			pending?: boolean
			error?: string
		}>(),
		scrapedAt: timestamp(),
	},
	t => [unique().on(t.userId, t.url)]
)
export type IPost = typeof Post.$inferSelect
export type IPostWithProgress = IPost & {
	scrapeProgress?: {
		state: 'waiting' | 'starting-browser' | 'navigating' | 'scraping' | 'done'
	}
}

export const Tag = pgTable('tags', {
	...defaultColumns,
	userId: text()
		.notNull()
		.references(() => User.id, { onDelete: 'cascade' }),
	name: text().notNull(),
})
export type ITag = typeof Tag.$inferSelect

export const PostTag = pgTable('post_tags', {
	...defaultColumns,
	postId: text()
		.notNull()
		.references(() => Post.id, { onDelete: 'cascade' }),
	tagId: text()
		.notNull()
		.references(() => Tag.id, { onDelete: 'cascade' }),
})
export type IPostTag = typeof PostTag.$inferSelect

export const View = pgTable('views', {
	...defaultColumns,
	userId: text()
		.notNull()
		.references(() => User.id, { onDelete: 'cascade' }),
	name: text().notNull(),
	filters: jsonb().$type<null | { tagIds?: string[] }>(),
	order: integer().default(0),
})
export type IView = typeof View.$inferSelect
