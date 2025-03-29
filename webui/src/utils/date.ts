export function formatDate(date: string) {
	if (!date) return ''

	const postDate = new Date(date)
	const now = new Date()

	// Format time component (HH:mm)
	const hours = postDate.getHours().toString().padStart(2, '0')
	const minutes = postDate.getMinutes().toString().padStart(2, '0')
	const timeStr = `${hours}:${minutes}`

	// Check if date is today
	if (postDate.setHours(0, 0, 0, 0) === now.setHours(0, 0, 0, 0)) {
		return `Today ${timeStr}`
	}

	// Reset the dates after comparison
	const postDateReset = new Date(date)
	const nowReset = new Date()

	// Check if date is yesterday
	const yesterday = new Date(nowReset)
	yesterday.setDate(yesterday.getDate() - 1)
	if (postDateReset.setHours(0, 0, 0, 0) === yesterday.setHours(0, 0, 0, 0)) {
		return `Yesterday ${timeStr}`
	}

	// Get day and month
	const day = postDateReset.getDate()
	const month = postDateReset.toLocaleString(undefined, { month: 'long' })

	// Check if date is within current year
	if (postDateReset.getFullYear() === nowReset.getFullYear()) {
		return `${day}${getDaySuffix(day)} of ${month}, ${timeStr}`
	}

	// Date is from previous year
	return `${day}${getDaySuffix(day)} of ${month} ${postDateReset.getFullYear()}, ${timeStr}`
}

// Helper function to get the correct suffix for the day (1st, 2nd, 3rd, etc.)
function getDaySuffix(day: number): string {
	if (day > 3 && day < 21) return 'th'
	switch (day % 10) {
		case 1:
			return 'st'
		case 2:
			return 'nd'
		case 3:
			return 'rd'
		default:
			return 'th'
	}
}
