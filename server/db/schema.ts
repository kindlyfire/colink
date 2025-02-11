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

export const Link = pgTable(
	'links',
	{
		...defaultColumns,
		userId: text()
			.notNull()
			.references(() => User.id, { onDelete: 'cascade' }),
		html: text().notNull(),
		url: text().notNull(),
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

export const LinkTag = pgTable('link_tags', {
	...defaultColumns,
	linkId: text()
		.notNull()
		.references(() => Link.id, { onDelete: 'cascade' }),
	tagId: text()
		.notNull()
		.references(() => Tag.id, { onDelete: 'cascade' }),
})
