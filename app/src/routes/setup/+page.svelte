<script lang="ts">
    import { goto } from '$app/navigation'
    import { serverUrlStore } from '$lib/stores/serverUrl'

    let url = $state(serverUrlStore.get())
    let error = $state<string | null>(null)
    let testing = $state(false)

    function isValidUrl(s: string): boolean {
        try {
            const u = new URL(s)
            return u.protocol === 'http:' || u.protocol === 'https:'
        } catch {
            return false
        }
    }

    async function handleSubmit(e: SubmitEvent) {
        e.preventDefault()
        error = null
        const trimmed = url.trim().replace(/\/+$/, '')

        if (!isValidUrl(trimmed)) {
            error = 'Enter a valid URL (e.g. https://api.example.com)'
            return
        }

        testing = true
        try {
            const res = await fetch(`${trimmed}/api/version`, { signal: AbortSignal.timeout(5000) })
            if (!res.ok && res.status !== 404) throw new Error(`Server responded with ${res.status}`)
        } catch (err) {
            if (err instanceof Error && err.name === 'TimeoutError') {
                error = 'Server unreachable — check the URL and try again'
                testing = false
                return
            }
        } finally {
            testing = false
        }

        serverUrlStore.set(trimmed)
        goto('/login')
    }
</script>

<div class="h-dvh flex items-center justify-center bg-surface-950 p-5">
    <div class="card bg-surface-900 border border-surface-700 p-8 w-full max-w-sm space-y-6">
        <div>
            <h1 class="h3 font-bold tracking-tight text-surface-50">
                Focus<em class="text-primary-400 not-italic">Flow</em>
            </h1>
            <p class="text-sm text-surface-400 mt-1">Connect to your server</p>
        </div>

        <form onsubmit={handleSubmit} class="space-y-4">
            <label class="label">
                <span class="label-text text-xs font-mono tracking-widest uppercase text-surface-400">
                    Server URL
                </span>
                <input
                    class="input"
                    bind:value={url}
                    placeholder="https://api.example.com"
                    type="url"
                    autocomplete="url"
                    required
                />
            </label>

            {#if error}
                <aside class="alert preset-tonal-error">
                    <p class="alert-message text-sm">{error}</p>
                </aside>
            {/if}

            <button
                type="submit"
                disabled={testing || !url.trim()}
                class="btn preset-filled-primary-500 w-full"
            >
                {testing ? 'Connecting…' : 'Connect'}
            </button>
        </form>
    </div>
</div>
