import { getSessionCookieName } from '~~/server/utils/h3'

export default defineEventHandler(event => {
	assertRequestMethod(event, 'POST')
	deleteCookie(event, getSessionCookieName(event))
	return true
})
