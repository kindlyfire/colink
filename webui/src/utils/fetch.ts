const baseUrl = import.meta.env.VITE_API_URL || ''
export function fetchApi<T = any>(path: string, init?: RequestInit) {
	return fetchEx<T>(`${baseUrl}${path}`, {
		credentials: 'include',
		...init,
		headers: {
			Accept: 'application/json',
			'Content-Type': 'application/json',
			...init?.headers,
		},
	})
}

/**
 * Fetch wrapper that returns a {@link FetchPromise} that resolves with a
 * {@link ResponseEx}, and throws a {@link RequestError} if the response is not
 * OK.
 */
export function fetchEx<T = any>(input: string, init?: RequestInit) {
	return FetchPromise.resolve(
		fetch(new URL(input), init).then(async res => {
			if (!res.ok) {
				throw new RequestError(res, await res.text().catch(() => null))
			}
			return new ResponseEx<T>(res)
		})
	) as FetchPromise<T>
}

/**
 * A `Promise` that resolves with a {@link ResponseEx}, and has a `.json()` method
 * for easy chaining without double-awaiting.
 *
 * @example
 * ```ts
 * const res = await fetchEx('/api/posts').json()
 * ```
 */
class FetchPromise<T> extends Promise<ResponseEx<T>> {
	json() {
		return this.then(res => res.json() as T)
	}
}

/**
 * Error thrown by {@link fetchEx} when the response is not OK. The error
 * contains the original response and the body of the response, if available, as
 * well as a parsed JSON object if the body is valid JSON.
 */
export class RequestError extends Error {
	response: Response
	body?: any
	json?: any

	constructor(data: Response, body?: any) {
		try {
			if (body) {
				var json = JSON.parse(body)
			}
		} catch (e) {}
		super(json?.error ?? json?.message ?? 'Request failed')
		this.response = data
		this.body = body
	}
}

/**
 * Wrapper around the native Responce class that adds the right return type to
 * `.json()`.
 */
class ResponseEx<T> extends Response {
	constructor(response: Response) {
		super(response.body, response)
	}

	json() {
		return super.json().then(data => data as T)
	}
}
