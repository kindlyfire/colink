import { scrapingManager } from '../scraper/manager'

export default defineNitroPlugin(nitroApp => {
	scrapingManager.loadPending().catch(console.error)
})
