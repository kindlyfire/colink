import { getSessionCookieName } from '~~/server/utils/h3'

export default defineEventHandler(event => {
	ensureRequestMethod(event, 'POST')
	deleteCookie(event, getSessionCookieName(event))
	return true
})
