import { writable } from 'svelte/store'
import { setTokens, clearTokens, getAccessToken } from '$lib/api'

interface AuthState {
    username: string | null
    isAuthenticated: boolean
}

function loadInitial(): AuthState {
    if (typeof window === 'undefined') return { username: null, isAuthenticated: false }
    try {
        const raw = localStorage.getItem('ff-auth')
        if (raw) {
            const p = JSON.parse(raw) as { username?: string; isAuthenticated?: boolean }
            return { username: p.username ?? null, isAuthenticated: p.isAuthenticated ?? false }
        }
    } catch {
        /* ignore */
    }
    return { username: null, isAuthenticated: !!getAccessToken() }
}

const _store = writable<AuthState>(loadInitial())

export const authStore = {
    subscribe: _store.subscribe,
    login(username: string, token: string, refresh: string) {
        setTokens(token, refresh)
        const state: AuthState = { username, isAuthenticated: true }
        _store.set(state)
        localStorage.setItem('ff-auth', JSON.stringify(state))
    },
    logout() {
        clearTokens()
        const state: AuthState = { username: null, isAuthenticated: false }
        _store.set(state)
        localStorage.removeItem('ff-auth')
    },
}
