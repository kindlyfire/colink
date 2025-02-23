/**
 * Utils for working with Puppeteer. Automatically manages creating the browser,
 * the pages, and closing them after a while.
 */
import puppeteer, { Page, Browser } from 'puppeteer'
import { PuppeteerBlocker } from '@ghostery/adblocker-puppeteer'

const puppeteerBlockerPromise = PuppeteerBlocker.fromPrebuiltFull(fetch)
let browser: Promise<Browser> | null = null
let browserTimeout: any
const BROWSER_INACTIVITY_TIMEOUT = 1000 * 60 * 5

function createBrowser() {
	let _browser = browser || puppeteer.launch()
	browser = _browser

	// Close browser after inactivity
	clearTimeout(browserTimeout)
	browserTimeout = setTimeout(() => {
		browser?.then(b => b.close())
		browser = null
	}, BROWSER_INACTIVITY_TIMEOUT)

	return _browser
}

async function createPage() {
	const _browser = await createBrowser()
	const page = await _browser.newPage()
	await Promise.all([
		page.setBypassCSP(true),
		puppeteerBlockerPromise.then(blocker => blocker.enableBlockingInPage(page)),
	])
	return page
}

export async function runWithPage<T>(cb: (p: Page) => Promise<T>): Promise<T> {
	const page = await createPage()
	return cb(page).finally(() => page.close())
}
