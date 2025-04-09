export interface User {
	id: string
	username: string
	created_at: string
	updated_at: string
}

export interface Post {
	id: string
	created_at: string
	updated_at: string
	user_id: string
	text: string
	links: Link[]
}
export interface Link {
	id: string
	created_at: string
	updated_at: string
	user_id: string
	url: string
	title: string | null
	html: string | null
	text: string | null
	scraped_at: string | null
	scrape_pending: number
	scrape_error: string | null
}
