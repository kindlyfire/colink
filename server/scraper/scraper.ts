import { IPost, IPostWithProgress } from '../db/schema'
import { runWithPage } from './browser'

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
