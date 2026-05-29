<script lang="ts">
    import { onMount, onDestroy } from 'svelte'
    import { wsStore } from '$lib/ws'
    import { timerStore } from '$lib/stores/timer'
    import { fmtTime } from '$lib/utils'

    const CIRCUMFERENCE = 552.92

    const SESSION_LABELS: Record<string, string> = {
        Work: 'Focus',
        ShortBreak: 'Short Break',
        LongBreak: 'Long Break',
    }
    const SESSION_TARGETS: Record<string, number> = {
        Work: 25 * 60,
        ShortBreak: 5 * 60,
        LongBreak: 15 * 60,
    }

    let noteInput = $state('')
    let tick = $state(0)
    let tickInterval: ReturnType<typeof setInterval>

    onMount(() => {
        wsStore.start()
        tickInterval = setInterval(() => tick++, 1000)
        return () => {
            clearInterval(tickInterval)
            wsStore.stop()
        }
    })

    let session = $derived($wsStore.state?.currentSession ?? null)
    let hasSession = $derived(!!session)
    let isWork = $derived(session?.sessionType === 'Work')
    let sessionLabel = $derived(session ? (SESSION_LABELS[session.sessionType] ?? session.sessionType) : 'Focus')

    let remaining = $derived.by(() => {
        if (!session) return 25 * 60
        const now = Math.floor(Date.now() / 1000)
        const elapsed = Math.max(0, now - session.sessionStartTime)
        const target = SESSION_TARGETS[session.sessionType] ?? 25 * 60
        return Math.max(0, target - elapsed)
    })

    let progress = $derived.by(() => {
        if (!session) return 0
        const target = SESSION_TARGETS[session.sessionType] ?? 25 * 60
        return 1 - remaining / target
    })

    let dashOffset = $derived(CIRCUMFERENCE * (1 - progress))

    function sendNote() {
        const val = noteInput.trim()
        if (val) { wsStore.send({ type: 'updateNote', note: val }); noteInput = '' }
    }

    // tick is used to force re-evaluation of remaining/progress every second
    $effect(() => { void tick })
</script>

<div class="flex-1 min-h-0 flex flex-col overflow-hidden">
    <div class="flex-1 overflow-y-auto pb-32 flex flex-col items-center px-4 pt-4 gap-4">

        <div class="w-full flex items-center justify-between">
            {#if $timerStore.selectedTask}
                <div class="flex items-center gap-2 bg-primary-500/10 border border-primary-500/30 rounded-full px-3 py-1.5 max-w-[75%]">
                    <div class="size-1.5 rounded-full bg-primary-400 shrink-0" />
                    <span class="text-xs text-primary-300 truncate">{$timerStore.selectedTask.title}</span>
                </div>
            {:else}
                <div class="flex items-center gap-2 bg-surface-800 border border-surface-700 rounded-full px-3 py-1.5">
                    <span class="text-xs text-surface-500">No task selected</span>
                </div>
            {/if}
            <div class={`size-2 rounded-full ${$wsStore.connected ? 'bg-green-400' : 'bg-surface-600'}`} title={$wsStore.connected ? 'Connected' : 'Disconnected'} />
        </div>

        <p class="text-xs font-mono text-surface-500 uppercase tracking-widest">{sessionLabel}</p>

        <div class="relative">
            <svg viewBox="0 0 200 200" width="220" height="220">
                <circle cx="100" cy="100" r="88" fill="none" stroke="currentColor" stroke-width="6" class="text-surface-800" />
                <circle
                    cx="100" cy="100" r="88" fill="none"
                    stroke="currentColor" stroke-width="6" stroke-linecap="round"
                    class={isWork ? 'text-primary-500' : 'text-green-500'}
                    stroke-dasharray={CIRCUMFERENCE}
                    stroke-dashoffset={dashOffset}
                    transform="rotate(-90 100 100)"
                    style="transition: stroke-dashoffset 0.5s ease"
                />
            </svg>
            <div class="absolute inset-0 flex flex-col items-center justify-center">
                <span class="text-4xl font-mono font-bold text-surface-50 tabular-nums">{fmtTime(remaining)}</span>
                <span class="text-xs text-surface-500 mt-1">{hasSession ? 'Remaining' : 'Ready'}</span>
            </div>
        </div>

        <div class="flex items-center gap-3">
            {#if !isWork}
                <button class="btn preset-filled-primary-500 px-6" onclick={() => wsStore.send({ type: 'start' })}>
                    {hasSession ? 'Resume' : 'Start'}
                </button>
            {/if}
            {#if isWork}
                <button class="btn preset-tonal-surface px-6" onclick={() => wsStore.send({ type: 'break' })}>Break</button>
            {/if}
            {#if hasSession}
                <button class="btn preset-tonal-error px-6" onclick={() => wsStore.send({ type: 'terminate' })}>Stop</button>
            {/if}
        </div>

        {#if hasSession && isWork}
            <div class="w-full card bg-surface-900 border border-surface-700 p-4 flex flex-col gap-4">
                <div>
                    <p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-2">Focus Score</p>
                    <div class="flex gap-2">
                        {#each [1, 2, 3, 4, 5] as i (i)}
                            <button
                                onclick={() => wsStore.send({ type: 'updateScore', score: i })}
                                class={`text-xl transition-colors ${(session?.concentrationScore ?? 0) >= i ? 'text-primary-400' : 'text-surface-700'}`}
                            >★</button>
                        {/each}
                    </div>
                </div>

                <div>
                    <p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-2">Note</p>
                    <div class="flex items-center gap-2 input rounded-lg px-3 py-2">
                        <input
                            class="flex-1 bg-transparent outline-none text-sm text-surface-100 placeholder:text-surface-500"
                            placeholder={session?.note || 'Add a note…'}
                            bind:value={noteInput}
                            onkeydown={(e) => { if (e.key === 'Enter') { e.preventDefault(); sendNote() } }}
                        />
                        <button onclick={sendNote} aria-label="Send note" class="text-surface-500 hover:text-surface-200 transition-colors">
                            <svg viewBox="0 0 16 16" width="14" height="14">
                                <line x1="2" y1="8" x2="14" y2="8" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" />
                                <polyline points="9 3 14 8 9 13" stroke="currentColor" stroke-width="1.6" fill="none" stroke-linecap="round" stroke-linejoin="round" />
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        {/if}

        {#if $wsStore.error}
            <aside class="alert preset-tonal-error w-full">
                <p class="alert-message text-sm">Connection error: {$wsStore.error}</p>
            </aside>
        {/if}
    </div>
</div>
