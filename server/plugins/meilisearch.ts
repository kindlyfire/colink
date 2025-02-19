export default defineNitroPlugin(async nitroApp => {
	await indexingManager.migrate()
})
