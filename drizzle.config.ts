import { defineConfig } from 'drizzle-kit'

export default defineConfig({
	out: './server/db/migrations',
	schema: './server/db/schema.ts',
	dialect: 'postgresql',
	dbCredentials: {
		url:
			process.env.NUXT_DATABASE_URL ||
			'postgresql://postgres:postgres@127.0.0.1:5432/postgres',
	},
})
