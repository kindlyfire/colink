import { getSessionCookieName } from '~~/server/utils/h3'

export default defineEventHandler(event => {
	deleteCookie(event, getSessionCookieName(event))
	return true
})
