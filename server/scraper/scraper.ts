// - Actually scraping
// - On startup, scraping what's pending/was interrupted
// - A function to start a scrape

import puppeteer, { Page } from 'puppeteer'
import { PuppeteerBlocker } from '@ghostery/adblocker-puppeteer'
import { IPost, IPostWithProgress } from '../db/schema'

interface ScrapeOptions {
	post: IPost
	setProgress: (progress: NonNullable<IPostWithProgress['scrapeProgress']>) => void
}

export async function scrape({ post, setProgress }: ScrapeOptions) {
	if (!post.url) throw new Error('No URL to scrape')

	setProgress({ state: 'starting-browser' })

	return await runWithPage(async page => {
		setProgress({ state: 'navigating' })
		await page.goto(post.url!, {
			waitUntil: 'networkidle0',
		})

		setProgress({ state: 'scraping' })
		const res = await page.evaluate(() => {
			return (async () => {
				// TODO: Embed script in the codebase somewhere
				const importUrl = 'https://esm.sh/@mozilla/readability@0.5.0'
				const Readability = await import(importUrl)
				const parsed = new Readability.Readability(eval(`document`)).parse()
				return {
					title: parsed.title as string,
					html: parsed.content as string,
					text: parsed.textContent as string,
				}
			})()
		})
		return res
	})
}

async function runWithPage<T>(cb: (p: Page) => T) {
	const browser = await puppeteer.launch()
	const page = await browser.newPage()
	const blocker = await PuppeteerBlocker.fromPrebuiltAdsAndTracking(fetch)
	await blocker.enableBlockingInPage(page)
	await page.setBypassCSP(true)
	const res = await cb(page)
	await browser.close()
	return res
}
