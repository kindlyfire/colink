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
}
