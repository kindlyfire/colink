import { Peer } from 'crossws'
import { parse } from 'cookie-es'
import { db } from '../db'
import { Session } from '../db/schema'
import { eq } from 'drizzle-orm'

export interface DataChangedEvent {
	type: 'dataChanged'
}

export type WebsocketEvent = DataChangedEvent

class WsPeerTracker {
	peers: Peer[] = []
	userMap: Map<string, Peer[]> = new Map()

	add(peer: Peer, user: string) {
		peer.context.user = user
		this.peers.push(peer)
		let userPeers = this.userMap.get(user) ?? []
		userPeers.push(peer)
		this.userMap.set(user, userPeers)
	}

	remove(peer: Peer) {
		this._removePeerFrom(peer, this.peers)
		const userPeers = this.userMap.get(peer.context.user as string)!
		this._removePeerFrom(peer, userPeers)
		if (userPeers.length === 0) {
			this.userMap.delete(peer.context.user as string)
		}
	}

	_removePeerFrom(peer: Peer, list: Peer[]) {
		const index = list.indexOf(peer)
		if (index !== -1) {
			list.splice(index, 1)
		}
	}

	sendToUser(user: string, data: WebsocketEvent) {
		const userPeers = this.userMap.get(user) ?? []
		for (const peer of userPeers) {
			peer.send(JSON.stringify(data))
		}
	}

	sendDataChangedEvent(user: string) {
		this.sendToUser(user, { type: 'dataChanged' })
	}
}

export const wsPeerTracker = new WsPeerTracker()

export default defineWebSocketHandler({
	async open(peer) {
		const cookies = parse(peer.request.headers.get('cookie') || '')
		const token = cookies['__Secure-session'] || cookies['session']
		if (!token) {
			peer.close(4001, 'Unauthorized')
			return
		}

		const [session] = await db.select().from(Session).where(eq(Session.token, token))
		if (!session) {
			peer.close(4001, 'Unauthorized')
			return
		}

		wsPeerTracker.add(peer, session.userId)
	},
	close(peer) {
		wsPeerTracker.remove(peer)
	},
})
