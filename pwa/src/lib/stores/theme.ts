import { writable } from 'svelte/store'

export const THEMES = [
    'catppuccin', 'cerberus', 'concord', 'crimson', 'fennec',
    'hamlindigo', 'legacy', 'mint', 'modern', 'mona', 'nosh',
    'nouveau', 'pine', 'reign', 'rocket', 'rose', 'sahara',
    'seafoam', 'terminus', 'vintage', 'vox', 'wintry',
] as const

export type Theme = (typeof THEMES)[number]

function applyTheme(theme: string) {
    if (typeof document !== 'undefined') {
        document.documentElement.setAttribute('data-theme', theme)
    }
}

function loadInitial(): Theme {
    if (typeof window === 'undefined') return 'nosh'
    try {
        const raw = localStorage.getItem('ff-theme')
        if (raw) {
            const p = JSON.parse(raw) as { theme?: string }
            if (p?.theme && THEMES.includes(p.theme as Theme)) return p.theme as Theme
        }
    } catch {
        /* ignore */
    }
    return 'nosh'
}

const initial = loadInitial()
applyTheme(initial)

const _store = writable<Theme>(initial)

export const themeStore = {
    subscribe: _store.subscribe,
    setTheme(theme: Theme) {
        applyTheme(theme)
        _store.set(theme)
        localStorage.setItem('ff-theme', JSON.stringify({ theme }))
    },
}
