import tailwindcss from '@tailwindcss/vite'

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
	compatibilityDate: '2024-11-01',
	devtools: { enabled: true },
	srcDir: 'app',
	serverDir: 'server',
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
})
