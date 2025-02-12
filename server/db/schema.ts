import { pgTable, text, timestamp, unique } from 'drizzle-orm/pg-core'
import { createId } from '@paralleldrive/cuid2'

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
	password: text(),
})

export const Session = pgTable('sessions', {
	...defaultColumns,
	userId: text()
		.notNull()
		.references(() => User.id, { onDelete: 'cascade' }),
	token: text().unique().notNull(),
	lastSeen: timestamp().notNull().defaultNow(),
	label: text(),
})

export const Post = pgTable(
	'posts',
	{
		...defaultColumns,
		userId: text()
			.notNull()
			.references(() => User.id, { onDelete: 'cascade' }),
		type: text().notNull().$type<'link' | 'note'>(),
		html: text().notNull(),
		url: text(),
	},
	t => [unique().on(t.userId, t.url)]
)

export const Tag = pgTable('tags', {
	...defaultColumns,
	userId: text()
		.notNull()
		.references(() => User.id, { onDelete: 'cascade' }),
	name: text().notNull(),
})

export const PostTag = pgTable('post_tags', {
	...defaultColumns,
	postId: text()
		.notNull()
		.references(() => Post.id, { onDelete: 'cascade' }),
	tagId: text()
		.notNull()
		.references(() => Tag.id, { onDelete: 'cascade' }),
})
