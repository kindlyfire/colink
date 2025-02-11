import { drizzle } from 'drizzle-orm/bun-sql'

export const db = drizzle(useRuntimeConfig().databaseUrl)
