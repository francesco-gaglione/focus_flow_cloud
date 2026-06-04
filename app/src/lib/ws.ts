import { writable, get } from 'svelte/store'
import type { PomodoroWsState, WsSessionType } from '@/types'
import { getAccessToken } from '$lib/api'
import { showNotification } from '$lib/notifications'
import { serverUrlStore } from '$lib/stores/serverUrl'

export type WsCmd =
    | { type: 'start' }
    | { type: 'break' }
    | { type: 'terminate' }
    | { type: 'updateNote'; note: string }
    | { type: 'updateScore'; score: number }
    | { type: 'updateContext'; taskId: string | null }

interface WsState {
    state: PomodoroWsState | null
    connected: boolean
    error: string | null
}

const RECONNECT_BASE = 1000
const RECONNECT_MAX = 30000

const SESSION_NOTIFICATIONS: Record<WsSessionType, { title: string; body: string }> = {
    Work: { title: 'FocusFlow', body: "Break's over! Time to focus." },
    ShortBreak: { title: 'FocusFlow', body: 'Focus session complete! Take a short break.' },
    LongBreak: { title: 'FocusFlow', body: 'Focus session complete! Time for a long break.' },
}

function getWsUrl(): string {
    const base = serverUrlStore.get() || window.location.origin
    return base.replace(/^https?:\/\//, (m: string) =>
        m.startsWith('https') ? 'wss://' : 'ws://'
    )
}

function createWsStore() {
    const { subscribe, set, update } = writable<WsState>({
        state: null,
        connected: false,
        error: null,
    })

    let ws: WebSocket | null = null
    let retryCount = 0
    let retryTimer: ReturnType<typeof setTimeout> | undefined
    let active = false
    let prevSessionType: WsSessionType | null = null

    function handleStateUpdate(next: PomodoroWsState) {
        const nextType = next.currentSession?.sessionType ?? null

        if (prevSessionType !== nextType) {
            if (nextType !== null) {
                const n = SESSION_NOTIFICATIONS[nextType]
                showNotification(n.title, n.body).catch(() => {})
            } else if (prevSessionType !== null) {
                showNotification('FocusFlow', 'Session ended.').catch(() => {})
            }
            prevSessionType = nextType
        }

        update((s) => ({ ...s, state: next }))
    }

    function connect() {
        const token = getAccessToken()
        if (!token || !active) return

        const url = `${getWsUrl()}/ws/session?token=${encodeURIComponent(token)}`
        ws = new WebSocket(url)

        ws.onopen = () => {
            if (!active) { ws?.close(); return }
            retryCount = 0
            update((s) => ({ ...s, connected: true, error: null }))
            ws?.send(JSON.stringify({ requestSync: null }))
        }

        ws.onmessage = (ev) => {
            if (!active) return
            try {
                const val = JSON.parse(ev.data as string)
                const stateVal = val.syncData ?? val.pomodoroSessionUpdate
                if (stateVal) handleStateUpdate(stateVal as PomodoroWsState)
            } catch {
                /* ignore */
            }
        }

        ws.onerror = () => {
            if (!active) return
            update((s) => ({ ...s, error: 'WebSocket error' }))
        }

        ws.onclose = () => {
            if (!active) return
            update((s) => ({ ...s, connected: false }))
            ws = null
            const delay = Math.min(RECONNECT_BASE * 2 ** retryCount, RECONNECT_MAX)
            retryCount++
            retryTimer = setTimeout(connect, delay)
        }
    }

    return {
        subscribe,
        start() {
            active = true
            connect()
        },
        stop() {
            active = false
            prevSessionType = null
            clearTimeout(retryTimer)
            ws?.close()
            ws = null
            set({ state: null, connected: false, error: null })
        },
        send(cmd: WsCmd) {
            if (!ws || ws.readyState !== WebSocket.OPEN) return
            switch (cmd.type) {
                case 'start': ws.send(JSON.stringify({ startEvent: null })); break
                case 'break': ws.send(JSON.stringify({ breakEvent: null })); break
                case 'terminate': ws.send(JSON.stringify({ terminateEvent: null })); break
                case 'updateNote': ws.send(JSON.stringify({ updateNote: { newNote: cmd.note } })); break
                case 'updateScore': ws.send(JSON.stringify({ updateConcentrationScore: { concentrationScore: cmd.score } })); break
                case 'updateContext': ws.send(JSON.stringify({ updatePomodoroContext: { taskId: cmd.taskId } })); break
            }
        },
    }
}

export const wsStore = createWsStore()
