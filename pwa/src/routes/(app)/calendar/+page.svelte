<script lang="ts">
    import { createQuery } from '@tanstack/svelte-query'
    import { tasks, categories } from '$lib/api'
    import { tsToDate, isToday } from '$lib/utils'
    import type { TaskDto, TaskPriority, CategoryDto } from '@/types'

    type ViewMode = 'month' | 'week'

    const PRIORITY_COLORS: Record<TaskPriority, string> = {
        low: '#46a758',
        medium: '#d97706',
        high: '#ef4444',
        urgent: '#7c3aed',
    }
    const FALLBACK_COLOR = '#6b7280'
    const PRIORITY_ORDER: Record<string, number> = { urgent: 0, high: 1, medium: 2, low: 3 }

    function sortByUrgency(ts: TaskDto[]): TaskDto[] {
        return [...ts].sort((a, b) => {
            const pa = a.priority ? (PRIORITY_ORDER[a.priority] ?? 4) : 4
            const pb = b.priority ? (PRIORITY_ORDER[b.priority] ?? 4) : 4
            return pa - pb
        })
    }

    const HOUR_H = 56       // px per hour
    const DAY_START = 6     // 06:00
    const DAY_END = 23      // 23:00
    const TOTAL_HOURS = DAY_END - DAY_START
    const DOW = ['Mo', 'Tu', 'We', 'Th', 'Fr', 'Sa', 'Su']

    function taskColor(task: TaskDto, cats: CategoryDto[]): string {
        if (task.priority) return PRIORITY_COLORS[task.priority]
        const cat = cats.find(c => c.id === task.categoryId)
        return cat?.color ?? FALLBACK_COLOR
    }

    function getTaskDate(task: TaskDto): Date | null {
        if (task.schedule.type === 'allDay') return tsToDate(task.schedule.date)
        if (task.schedule.type === 'at') return tsToDate(task.schedule.startsAt)
        if (task.schedule.type === 'span') return tsToDate(task.schedule.startsAt)
        return null
    }

    function isSameDay(a: Date, b: Date) {
        return a.getFullYear() === b.getFullYear() &&
               a.getMonth() === b.getMonth() &&
               a.getDate() === b.getDate()
    }

    function weekStart(d: Date): Date {
        const day = d.getDay()
        const diff = day === 0 ? -6 : 1 - day
        const m = new Date(d)
        m.setDate(d.getDate() + diff)
        m.setHours(0, 0, 0, 0)
        return m
    }

    function addDays(d: Date, n: number): Date {
        const r = new Date(d)
        r.setDate(d.getDate() + n)
        return r
    }

    function taskTop(task: TaskDto): number {
        let ts: number | undefined
        if (task.schedule.type === 'at') ts = task.schedule.startsAt
        else if (task.schedule.type === 'span') ts = task.schedule.startsAt
        if (ts === undefined) return 0
        const d = tsToDate(ts)
        const hours = d.getHours() + d.getMinutes() / 60 - DAY_START
        return Math.max(0, Math.min(hours * HOUR_H, TOTAL_HOURS * HOUR_H - 28))
    }

    function taskHeight(task: TaskDto): number {
        if (task.schedule.type === 'span') {
            return Math.max(28, (task.schedule.duration / 3600) * HOUR_H)
        }
        return Math.max(28, 0.5 * HOUR_H)
    }

    let view = $state<ViewMode>('month')
    let current = $state(new Date())
    let selected = $state(new Date())

    const tasksQuery = createQuery({ queryKey: ['tasks'], queryFn: tasks.getAll })
    const catsQuery = createQuery({ queryKey: ['categories'], queryFn: categories.getAll })

    let allTasks = $derived($tasksQuery.data?.tasks ?? [])
    let allCats = $derived($catsQuery.data?.categories ?? [])

    // MONTH
    let year = $derived(current.getFullYear())
    let month = $derived(current.getMonth())
    let startDow = $derived((new Date(year, month, 1).getDay() + 6) % 7)
    let daysInMonth = $derived(new Date(year, month + 1, 0).getDate())
    let monthName = $derived(current.toLocaleString('default', { month: 'long', year: 'numeric' }))

    // WEEK
    let wStart = $derived(weekStart(current))
    let weekDays = $derived(Array.from({ length: 7 }, (_, i) => addDays(wStart, i)))
    let weekLabel = $derived(
        `${wStart.toLocaleDateString('default', { month: 'short', day: 'numeric' })} – ` +
        `${addDays(wStart, 6).toLocaleDateString('default', { month: 'short', day: 'numeric', year: 'numeric' })}`
    )
    let headerLabel = $derived(view === 'month' ? monthName : weekLabel)
    let hours = $derived(Array.from({ length: TOTAL_HOURS }, (_, i) => i))

    function tasksOnDay(d: Date) {
        return allTasks.filter(t => { const td = getTaskDate(t); return td && isSameDay(td, d) })
    }

    function allDayOnDay(d: Date) {
        return allTasks.filter(t => t.schedule.type === 'allDay' && isSameDay(tsToDate(t.schedule.date), d))
    }

    function timedOnDay(d: Date) {
        return allTasks.filter(t => {
            if (t.schedule.type !== 'at' && t.schedule.type !== 'span') return false
            const td = getTaskDate(t)
            return td && isSameDay(td, d)
        })
    }

    function navPrev() {
        if (view === 'month') current = new Date(year, month - 1, 1)
        else current = addDays(current, -7)
    }

    function navNext() {
        if (view === 'month') current = new Date(year, month + 1, 1)
        else current = addDays(current, 7)
    }

    let hasAllDay = $derived(weekDays.some(d => allDayOnDay(d).length > 0))
</script>

<div class="flex-1 min-h-0 flex flex-col overflow-hidden">
    <!-- header -->
    <div class="flex items-center gap-2 px-4 pt-2 pb-3 border-b border-surface-800 shrink-0">
        <button class="btn btn-icon preset-tonal-surface size-8" aria-label="Previous" onclick={navPrev}>
            <svg viewBox="0 0 12 12" width="12" height="12" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"><polyline points="8 2 4 6 8 10" /></svg>
        </button>
        <span class="flex-1 text-center text-sm font-semibold text-surface-100">{headerLabel}</span>
        <button class="btn btn-icon preset-tonal-surface size-8" aria-label="Next" onclick={navNext}>
            <svg viewBox="0 0 12 12" width="12" height="12" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"><polyline points="4 2 8 6 4 10" /></svg>
        </button>
        <button class="btn preset-tonal-surface text-xs h-8 px-3 ml-1" onclick={() => { current = new Date(); selected = new Date() }}>Today</button>
        <div class="flex rounded-md overflow-hidden border border-surface-700 ml-1">
            {#each (['month', 'week'] as ViewMode[]) as v (v)}
                <button
                    onclick={() => (view = v)}
                    class={`px-3 h-8 text-xs font-medium transition-colors ${view === v ? 'bg-primary-500 text-white' : 'bg-surface-900 text-surface-400 hover:text-surface-200'}`}
                >
                    {v === 'month' ? 'M' : 'W'}
                </button>
            {/each}
        </div>
    </div>

    <!-- MONTH VIEW -->
    {#if view === 'month'}
        <div class="flex-1 overflow-y-auto pb-32">
            <div class="grid grid-cols-7 px-2 pt-2">
                {#each DOW as d (d)}
                    <div class="text-center text-[10px] font-mono text-surface-500 uppercase tracking-wider py-1">{d}</div>
                {/each}
            </div>
            <div class="grid grid-cols-7 px-2 gap-px">
                {#each Array.from({ length: startDow }) as _, i (`pre-${i}`)}
                    <div></div>
                {/each}
                {#each Array.from({ length: daysInMonth }) as _, i (i)}
                    {@const d = new Date(year, month, i + 1)}
                    {@const dayTasks = sortByUrgency(tasksOnDay(d))}
                    {@const todayCell = isToday(Math.floor(d.getTime() / 1000))}
                    {@const sel = isSameDay(d, selected)}
                    <div
                        onclick={() => (selected = d)}
                        class={`min-h-[52px] p-1 rounded-md cursor-pointer transition-colors ${sel ? 'bg-primary-500/20 ring-1 ring-primary-500' : 'hover:bg-surface-800'}`}
                        role="button"
                        tabindex="0"
                        onkeydown={(e) => e.key === 'Enter' && (selected = d)}
                    >
                        <span class={`text-xs font-medium block text-center w-5 h-5 rounded-full mx-auto leading-5 ${todayCell ? 'bg-primary-500 text-white' : 'text-surface-300'}`}>
                            {i + 1}
                        </span>
                        <div class="mt-0.5 flex flex-col gap-0.5">
                            {#each dayTasks.slice(0, 2) as t (t.id)}
                                {@const color = taskColor(t, allCats)}
                                <div class="text-[9px] px-1 rounded truncate text-white" style="background:{color}">
                                    {t.title}
                                </div>
                            {/each}
                            {#if dayTasks.length > 2}
                                <div class="flex gap-0.5 flex-wrap mt-0.5 px-0.5">
                                    {#each dayTasks.slice(2) as t (t.id)}
                                        <div class="size-1.5 rounded-full shrink-0" style="background:{taskColor(t, allCats)}"></div>
                                    {/each}
                                </div>
                            {/if}
                        </div>
                    </div>
                {/each}
            </div>

            <div class="px-4 pt-4">
                <p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-2">
                    {selected.toLocaleString('default', { weekday: 'long', day: 'numeric', month: 'long' })}
                </p>
                {#if tasksOnDay(selected).length === 0}
                    <p class="text-sm text-surface-500 py-4 text-center">No tasks</p>
                {:else}
                    {#each sortByUrgency(tasksOnDay(selected)) as t (t.id)}
                        {@const color = taskColor(t, allCats)}
                        <div
                            class={`flex items-center gap-3 p-3 mb-2 rounded-r-md border border-surface-700 bg-surface-900 ${t.completedAt ? 'opacity-50' : ''}`}
                            style="border-left: 4px solid {color}"
                        >
                            <span class={`text-sm ${t.completedAt ? 'line-through text-surface-500' : 'text-surface-100'}`}>{t.title}</span>
                        </div>
                    {/each}
                {/if}
            </div>
        </div>
    {/if}

    <!-- WEEK VIEW -->
    {#if view === 'week'}
        <!-- day header -->
        <div class="flex shrink-0 border-b border-surface-800">
            <div class="w-10 shrink-0"></div>
            {#each weekDays as d, i (i)}
                {@const todayCol = isToday(Math.floor(d.getTime() / 1000))}
                <div class="flex-1 text-center py-1.5 border-l border-surface-800/40">
                    <span class="text-[10px] font-mono text-surface-500 uppercase block">{DOW[i]}</span>
                    <span class={`text-sm font-semibold block leading-none mt-0.5 ${todayCol ? 'text-primary-400' : 'text-surface-200'}`}>
                        {d.getDate()}
                    </span>
                </div>
            {/each}
        </div>

        <!-- all-day strip -->
        {#if hasAllDay}
            <div class="flex shrink-0 border-b border-surface-800 min-h-[28px]">
                <div class="w-10 shrink-0 flex items-center justify-end pr-1">
                    <span class="text-[8px] text-surface-600 font-mono">all day</span>
                </div>
                {#each weekDays as d, i (i)}
                    <div class="flex-1 border-l border-surface-800/40 px-0.5 py-0.5 flex flex-col gap-0.5">
                        {#each allDayOnDay(d) as t (t.id)}
                            {@const color = taskColor(t, allCats)}
                            <div class="text-[9px] px-1 rounded truncate text-white leading-4" style="background:{color}; opacity:{t.completedAt ? 0.4 : 1}">
                                {t.title}
                            </div>
                        {/each}
                    </div>
                {/each}
            </div>
        {/if}

        <!-- time grid -->
        <div class="flex-1 overflow-y-auto">
            <div class="flex" style="height:{TOTAL_HOURS * HOUR_H}px">
                <!-- time gutter -->
                <div class="w-10 shrink-0 relative">
                    {#each hours as h (h)}
                        <div
                            class="absolute right-0 pr-1 text-[9px] font-mono text-surface-600 leading-none"
                            style="top:{h * HOUR_H - 5}px"
                        >
                            {String(DAY_START + h).padStart(2, '0')}
                        </div>
                    {/each}
                </div>

                <!-- day columns -->
                {#each weekDays as d, i (i)}
                    <div class="flex-1 relative border-l border-surface-800/40">
                        <!-- hour lines -->
                        {#each hours as h (h)}
                            <div
                                class="absolute left-0 right-0 border-t border-surface-800/40"
                                style="top:{h * HOUR_H}px"
                            ></div>
                        {/each}

                        <!-- task boxes -->
                        {#each timedOnDay(d) as t (t.id)}
                            {@const top = taskTop(t)}
                            {@const height = taskHeight(t)}
                            {@const color = taskColor(t, allCats)}
                            <div
                                class="absolute left-0.5 right-0.5 rounded px-1 overflow-hidden text-white flex flex-col justify-start"
                                style="top:{top}px; height:{height}px; background:{color}; opacity:{t.completedAt ? 0.4 : 1}"
                                title={t.title}
                            >
                                <span class="text-[9px] font-medium leading-tight truncate mt-0.5">{t.title}</span>
                                {#if height > 40 && t.description}
                                    <span class="text-[8px] opacity-80 leading-tight truncate">{t.description}</span>
                                {/if}
                            </div>
                        {/each}
                    </div>
                {/each}
            </div>
        </div>
    {/if}
</div>
