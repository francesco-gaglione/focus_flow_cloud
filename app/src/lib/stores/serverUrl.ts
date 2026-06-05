const LS_KEY = 'ff_server_url'

function loadInitial(): string {
    if (typeof window === 'undefined') return ''
    try {
        return localStorage.getItem(LS_KEY) ?? ''
    } catch {
        return ''
    }
}

let _current = loadInitial()
const _listeners = new Set<(v: string) => void>()

export const serverUrlStore = {
    subscribe(fn: (v: string) => void): () => void {
        fn(_current)
        _listeners.add(fn)
        return () => _listeners.delete(fn)
    },
    set(url: string): void {
        _current = url.replace(/\/+$/, '')
        if (typeof window !== 'undefined') {
            if (_current) {
                localStorage.setItem(LS_KEY, _current)
            } else {
                localStorage.removeItem(LS_KEY)
            }
        }
        _listeners.forEach((fn) => fn(_current))
    },
    get(): string {
        return _current
    },
    isConfigured(): boolean {
        return !!_current
    },
}
