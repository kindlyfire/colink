import { drizzle } from 'drizzle-orm/node-postgres'

export const db = drizzle(useRuntimeConfig().databaseUrl)
