<script lang="ts">
    import { goto } from '$app/navigation'
    import { onMount } from 'svelte'
    import { _ } from 'svelte-i18n'
    import { authStore } from '$lib/stores/auth'
    import { auth, ApiError } from '$lib/api'
    import { serverUrlStore } from '$lib/stores/serverUrl'

    let username = $state('')
    let password = $state('')
    let error = $state<string | null>(null)
    let loading = $state(false)

    onMount(() => {
        if (!serverUrlStore.isConfigured()) goto('/setup')
        else if ($authStore.isAuthenticated) goto('/')
    })

    async function handleSubmit(e: SubmitEvent) {
        e.preventDefault()
        error = null
        loading = true
        try {
            const res = await auth.login({ username, password })
            authStore.login(username, res.token, res.refreshToken)
            goto('/')
        } catch (err) {
            error = err instanceof ApiError ? err.message || 'Login failed' : 'Network error'
        } finally {
            loading = false
        }
    }
</script>

<div class="h-dvh flex items-center justify-center bg-surface-950 p-5">
    <div class="card bg-surface-900 border border-surface-700 p-8 w-full max-w-sm space-y-6">
        <div>
            <h1 class="h3 font-bold tracking-tight text-surface-50">
                Focus<em class="text-primary-400 not-italic">Flow</em>
            </h1>
            <p class="text-sm text-surface-400 mt-1">{$_('login.login_title')}</p>
        </div>

        <form onsubmit={handleSubmit} class="space-y-4">
            <label class="label">
                <span class="label-text text-xs font-mono tracking-widest uppercase text-surface-400">
                    {$_('login.username')}
                </span>
                <input
                    class="input"
                    bind:value={username}
                    placeholder="username"
                    autocomplete="username"
                    required
                />
            </label>

            <label class="label">
                <span class="label-text text-xs font-mono tracking-widest uppercase text-surface-400">
                    {$_('login.password')}
                </span>
                <input
                    class="input"
                    type="password"
                    bind:value={password}
                    placeholder="••••••••"
                    autocomplete="current-password"
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
                disabled={loading}
                class="btn preset-filled-primary-500 w-full"
            >
                {loading ? $_('common.loading') : $_('login.sign_in')}
            </button>
        </form>
    </div>
</div>
