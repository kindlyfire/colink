import tailwindcss from '@tailwindcss/vite'

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
	runtimeConfig: {
		databaseUrl: 'postgresql://postgres:postgres@127.0.0.1:5432/postgres',
		registrationsEnabled: true,
		meilisearch: {
			host: 'http://127.0.0.1:7700',
			apiKey: 'masterkey',
		},
	},

	compatibilityDate: '2024-11-01',
	devtools: { enabled: true },
	srcDir: 'app',
	serverDir: 'server',
	modules: ['@pinia/nuxt'],
	vite: {
		plugins: [tailwindcss()],
	},
	css: ['~/assets/app.css'],
	postcss: {
		plugins: {
			cssnano: {
				preset: ['cssnano-preset-default', { calc: false }],
			},
		},
	},
	nitro: {
		experimental: {
			websocket: true,
		},
	},
})
