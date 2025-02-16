export default defineEventHandler(async event => {
	assertRequestMethod(event, 'GET')
	const authData = await getAuthData(event)
	if (!authData) {
		throw createError({ statusCode: 401, message: 'Unauthorized' })
	}
	return {
		user: {
			id: authData.user.id,
			username: authData.user.username,
		},
		session: {
			id: authData.session.id,
			lastSeen: authData.session.lastSeen,
			label: authData.session.label,
			createdAt: authData.session.createdAt,
		},
	}
})
