<script lang="ts">
    import { goto } from '$app/navigation'
    import { page } from '$app/stores'
    import { authStore } from '$lib/stores/auth'
    import { auth as authApi } from '$lib/api'

    const { open, onClose }: { open: boolean; onClose: () => void } = $props()

    type Section = 'tasks' | 'flashcards' | 'settings'

    function getSection(pathname: string): Section {
        if (pathname.startsWith('/cards')) return 'flashcards'
        if (pathname.startsWith('/settings')) return 'settings'
        return 'tasks'
    }

    let active = $derived(getSection($page.url.pathname))

    const NAV = [
        {
            id: 'tasks' as Section,
            to: '/',
            label: 'Tasks',
        },
        {
            id: 'flashcards' as Section,
            to: '/cards',
            label: 'Flashcards',
            badge: 'Soon',
        },
        {
            id: 'settings' as Section,
            to: '/settings',
            label: 'Settings',
        },
    ]

    async function handleLogout() {
        await authApi.logout().catch(() => {})
        authStore.logout()
        goto('/login')
    }

    function navigate(to: string) {
        onClose()
        goto(to)
    }
</script>

<aside
    class={[
        'absolute top-0 left-0 w-[76%] h-full z-40 flex flex-col overflow-hidden',
        'bg-surface-900 border-r border-surface-700',
        'transition-transform duration-200 ease-out',
        open ? 'translate-x-0' : '-translate-x-full',
    ].join(' ')}
>
    <div class="pt-12 px-5 pb-4 flex items-center gap-3 border-b border-surface-700">
        <div class="size-[34px] bg-primary-500/15 border border-primary-500 rounded-md grid place-items-center shrink-0 relative after:content-[''] after:absolute after:size-3 after:bg-primary-500 after:rounded" />
        <span class="text-xl font-bold text-surface-50 tracking-tight">
            Focus<em class="text-primary-400 not-italic">Flow</em>
        </span>
        <button
            class="ml-auto btn btn-icon preset-tonal-surface size-[30px]"
            onclick={onClose}
            aria-label="Close"
        >
            <svg viewBox="0 0 16 16" width="14" height="14" stroke="currentColor" fill="none" stroke-width="1.6">
                <line x1="4" y1="4" x2="12" y2="12" />
                <line x1="12" y1="4" x2="4" y2="12" />
            </svg>
        </button>
    </div>

    <p class="px-5 pt-4 pb-1.5 font-mono text-[10px] text-surface-500 tracking-widest uppercase">
        Workspace
    </p>

    <nav class="px-3 flex flex-col gap-0.5">
        {#each NAV as item (item.to)}
            <button
                onclick={() => navigate(item.to)}
                class={[
                    'flex items-center gap-3 px-2.5 py-2.5 rounded-md text-sm font-medium transition-colors w-full text-left',
                    active === item.id
                        ? 'bg-primary-500/15 text-primary-400'
                        : 'text-surface-400 hover:bg-surface-800 hover:text-surface-200',
                ].join(' ')}
            >
                <span
                    class={[
                        'size-8 border rounded-sm grid place-items-center shrink-0 transition-colors',
                        active === item.id
                            ? 'bg-primary-500 border-primary-500 text-white'
                            : 'bg-surface-800 border-surface-600 text-surface-400',
                    ].join(' ')}
                >
                    {#if item.id === 'tasks'}
                        <svg viewBox="0 0 16 16" width="14" height="14" stroke="currentColor" fill="none" stroke-width="1.5" stroke-linecap="round">
                            <path d="M3 4h10M3 8h10M3 12h6" />
                        </svg>
                    {:else if item.id === 'flashcards'}
                        <svg viewBox="0 0 16 16" width="14" height="14" stroke="currentColor" fill="none" stroke-width="1.5">
                            <rect x="2" y="3" width="10" height="9" rx="1" />
                            <rect x="4" y="5" width="10" height="9" rx="1" />
                        </svg>
                    {:else}
                        <svg viewBox="0 0 16 16" width="14" height="14" stroke="currentColor" fill="none" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                            <circle cx="8" cy="8" r="2.5" />
                            <path d="M8 1v1.5M8 13.5V15M1 8h1.5M13.5 8H15M3.05 3.05l1.06 1.06M11.89 11.89l1.06 1.06M3.05 12.95l1.06-1.06M11.89 4.11l1.06-1.06" />
                        </svg>
                    {/if}
                </span>
                <span class="flex-1">{item.label}</span>
                {#if item.badge}
                    <span class="font-mono text-[10px] text-surface-500 tracking-widest uppercase">{item.badge}</span>
                {/if}
            </button>
        {/each}
    </nav>

    <div class="mt-auto px-5 pt-4 pb-8 flex items-center gap-3 border-t border-surface-700">
        <div class="size-[34px] bg-surface-700 border border-surface-600 rounded-full grid place-items-center font-mono text-xs font-semibold text-surface-300 shrink-0">
            FF
        </div>
        <div class="flex-1 min-w-0">
            <p class="text-sm font-medium text-surface-100 truncate">
                {$authStore.username ?? 'User'}
            </p>
        </div>
        <button
            class="btn btn-icon size-8 preset-tonal-surface hover:preset-tonal-error"
            onclick={handleLogout}
            aria-label="Logout"
        >
            <svg viewBox="0 0 16 16" width="14" height="14" stroke="currentColor" fill="none" stroke-width="1.5" stroke-linecap="round">
                <path d="M6 3H3a1 1 0 0 0-1 1v8a1 1 0 0 0 1 1h3" />
                <polyline points="10 11 13 8 10 5" stroke-linejoin="round" />
                <line x1="13" y1="8" x2="6" y2="8" />
            </svg>
        </button>
    </div>
</aside>
