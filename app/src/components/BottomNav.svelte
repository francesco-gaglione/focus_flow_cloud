<script lang="ts">
    import { goto } from '$app/navigation'
    import { page } from '$app/stores'
    import { ListTodo, Calendar, Timer, BarChart2, Layers, Settings, RotateCcw } from 'lucide-svelte'

    type Section = 'tasks' | 'flashcards' | 'settings'
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    type Tab = { to: string; label: string; Icon: any }

    const TASKS_TABS: Tab[] = [
        { to: '/', label: 'Tasks', Icon: ListTodo },
        { to: '/calendar', label: 'Calendar', Icon: Calendar },
        { to: '/timer', label: 'Timer', Icon: Timer },
        { to: '/stats', label: 'Stats', Icon: BarChart2 },
    ]
    const FLASHCARDS_TABS: Tab[] = [
        { to: '/cards', label: 'Cards', Icon: Layers },
        { to: '/cards/review', label: 'Review', Icon: RotateCcw },
    ]
    const SETTINGS_TABS: Tab[] = [{ to: '/settings', label: 'Settings', Icon: Settings }]

    function getSection(pathname: string): Section {
        if (pathname.startsWith('/cards')) return 'flashcards'
        if (pathname.startsWith('/settings')) return 'settings'
        return 'tasks'
    }

    let section = $derived(getSection($page.url.pathname))
    let tabs = $derived(
        section === 'tasks' ? TASKS_TABS :
        section === 'flashcards' ? FLASHCARDS_TABS :
        SETTINGS_TABS
    )

    function isActive(to: string) {
        if (to === '/' || to === '/cards') return $page.url.pathname === to
        return $page.url.pathname.startsWith(to)
    }
</script>

<nav class="fixed bottom-0 left-0 right-0 z-20 border-t border-surface-700 bg-surface-950"
    style="padding-bottom: env(safe-area-inset-bottom)">
    <div
        class="grid"
        style="grid-template-columns: repeat({tabs.length}, minmax(0, 1fr))"
    >
        {#each tabs as tab (tab.to)}
            {@const Icon = tab.Icon}
            <button
                onclick={() => goto(tab.to)}
                class={[
                    'flex flex-col items-center justify-center gap-1 py-1.5 text-[10px] font-medium transition-colors',
                    isActive(tab.to)
                        ? 'text-primary-400 bg-primary-500/10'
                        : 'text-surface-500 hover:text-surface-300',
                ].join(' ')}
            >
                <Icon size={20} />
                <span>{tab.label}</span>
            </button>
        {/each}
    </div>
</nav>
