import { eq, sql } from 'drizzle-orm'
import { wsPeerManager } from '../api/ws'
import { db } from '../db'
import { IPost, IPostWithProgress, Post } from '../db/schema'
import { scrape } from './scraper'

/**
 * Manager for scraping posts. Scraping progress is stored in-memory and not
 * persisted.
 */
export class ScrapingManager {
	posts: IPost[] = []
	progress: Record<string, IPostWithProgress['scrapeProgress']> = {}

	addToQueue(post: IPost) {
		if (!(post.id in this.progress)) {
			this.posts.push(post)
		}

		this.#scrape(post).catch(console.error)
	}

	removeFromQueue(post: IPost) {
		this.posts = this.posts.filter(p => p.id !== post.id)
		delete this.progress[post.id]
	}

	augmentPost(post: IPost): IPostWithProgress {
		return {
			...post,
			scrapeProgress: this.progress[post.id],
		}
	}

	async loadPending() {
		const posts = await db
			.select()
			.from(Post)
			.where(sql`${Post.scrapeState}->>'pending' = 'true'`)

		for (const post of posts) {
			this.addToQueue(post)
		}
	}

	async #scrape(post: IPost) {
		let postUpdate = {} as Partial<IPost>

		try {
			const res = await scrape({
				post,
				setProgress: progress => this.#setProgress(post, progress),
			})
			postUpdate = {
				scrapeState: null,
				scrapedAt: new Date(),
				title: res.title,
			}
		} catch (e) {
			postUpdate = {
				scrapeState: { error: '' + e },
				scrapedAt: new Date(),
			}
		}

		await db.update(Post).set(postUpdate).where(eq(Post.id, post.id))
		this.removeFromQueue(post)
		wsPeerManager.sendDataChangedEvent(post.userId)
	}

	#setProgress(post: IPost, progress: IPostWithProgress['scrapeProgress']) {
		this.progress[post.id] = progress
		wsPeerManager.sendDataChangedEvent(post.userId)
	}
}

export const scrapingManager = new ScrapingManager()
