<script lang="ts">
    import { createQuery } from '@tanstack/svelte-query'
    import { tasks, categories } from '$lib/api'
    import { tsToDate, isToday } from '$lib/utils'
    import type { TaskDto } from '@/types'

    type ViewMode = 'month' | 'agenda'

    function getTaskDate(task: TaskDto): Date | null {
        if (task.schedule.type === 'allDay') return tsToDate(task.schedule.date)
        if (task.schedule.type === 'at') return tsToDate(task.schedule.startsAt)
        if (task.schedule.type === 'span') return tsToDate(task.schedule.startsAt)
        return null
    }

    function isSameDay(a: Date, b: Date) {
        return a.getFullYear() === b.getFullYear() && a.getMonth() === b.getMonth() && a.getDate() === b.getDate()
    }

    let view = $state<ViewMode>('month')
    let current = $state(new Date())
    let selected = $state(new Date())

    const tasksQuery = createQuery({ queryKey: ['tasks'], queryFn: tasks.getAll })
    const catsQuery = createQuery({ queryKey: ['categories'], queryFn: categories.getAll })

    let allTasks = $derived($tasksQuery.data?.tasks ?? [])
    let allCats = $derived($catsQuery.data?.categories ?? [])

    function tasksOnDay(d: Date) {
        return allTasks.filter((t) => { const td = getTaskDate(t); return td && isSameDay(td, d) })
    }

    let year = $derived(current.getFullYear())
    let month = $derived(current.getMonth())
    let startDow = $derived((new Date(year, month, 1).getDay() + 6) % 7)
    let daysInMonth = $derived(new Date(year, month + 1, 0).getDate())
    let monthName = $derived(current.toLocaleString('default', { month: 'long', year: 'numeric' }))
</script>

<div class="flex-1 min-h-0 flex flex-col overflow-hidden">
    <div class="flex items-center gap-2 px-4 pt-2 pb-3 border-b border-surface-800">
        <button class="btn btn-icon preset-tonal-surface size-8" aria-label="Previous month" onclick={() => (current = new Date(year, month - 1, 1))}>
            <svg viewBox="0 0 12 12" width="12" height="12" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"><polyline points="8 2 4 6 8 10" /></svg>
        </button>
        <span class="flex-1 text-center text-sm font-semibold text-surface-100">{monthName}</span>
        <button class="btn btn-icon preset-tonal-surface size-8" aria-label="Next month" onclick={() => (current = new Date(year, month + 1, 1))}>
            <svg viewBox="0 0 12 12" width="12" height="12" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"><polyline points="4 2 8 6 4 10" /></svg>
        </button>
        <button class="btn preset-tonal-surface text-xs h-8 px-3 ml-1" onclick={() => { current = new Date(); selected = new Date() }}>Today</button>
        <div class="flex rounded-md overflow-hidden border border-surface-700 ml-1">
            {#each (['month', 'agenda'] as ViewMode[]) as v (v)}
                <button
                    onclick={() => (view = v)}
                    class={`px-3 h-8 text-xs font-medium transition-colors ${view === v ? 'bg-primary-500 text-white' : 'bg-surface-900 text-surface-400 hover:text-surface-200'}`}
                >
                    {v === 'month' ? 'M' : 'A'}
                </button>
            {/each}
        </div>
    </div>

    <div class="flex-1 overflow-y-auto pb-32">
        {#if view === 'month'}
            <div class="grid grid-cols-7 px-2 pt-2">
                {#each ['Mo', 'Tu', 'We', 'Th', 'Fr', 'Sa', 'Su'] as d (d)}
                    <div class="text-center text-[10px] font-mono text-surface-500 uppercase tracking-wider py-1">{d}</div>
                {/each}
            </div>
            <div class="grid grid-cols-7 px-2 gap-px">
                {#each Array.from({ length: startDow }) as _, i (`pre-${i}`)}
                    <div></div>
                {/each}
                {#each Array.from({ length: daysInMonth }) as _, i (i)}
                    {@const d = new Date(year, month, i + 1)}
                    {@const dayTasks = tasksOnDay(d)}
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
                                {@const cat = allCats.find((c) => c.id === t.categoryId)}
                                <div class="text-[9px] px-1 rounded truncate text-white" style="background: {cat?.color ?? '#46a758'}">
                                    {t.title}
                                </div>
                            {/each}
                            {#if dayTasks.length > 2}
                                <div class="text-[9px] text-surface-500 text-center">+{dayTasks.length - 2}</div>
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
                    {#each tasksOnDay(selected) as t (t.id)}
                        {@const cat = allCats.find((c) => c.id === t.categoryId)}
                        <div
                            class={`flex items-center gap-3 p-3 mb-2 rounded-r-md border border-surface-700 bg-surface-900 ${t.completedAt ? 'opacity-50' : ''}`}
                            style="border-left: 4px solid {cat?.color ?? '#46a758'}"
                        >
                            <span class={`text-sm ${t.completedAt ? 'line-through text-surface-500' : 'text-surface-100'}`}>{t.title}</span>
                        </div>
                    {/each}
                {/if}
            </div>
        {/if}

        {#if view === 'agenda'}
            <div class="px-4 pt-3">
                {#each Array.from({ length: 30 }) as _, i (i)}
                    {@const d = (() => { const day = new Date(); day.setDate(day.getDate() + i); return day })()}
                    {@const dayTasks = tasksOnDay(d)}
                    {#if dayTasks.length > 0 || i === 0}
                        {@const todayDay = i === 0}
                        <div class="mb-4">
                            <div class="flex items-center gap-2 mb-1.5">
                                <span class={`text-sm font-semibold ${todayDay ? 'text-primary-400' : 'text-surface-300'}`}>
                                    {d.toLocaleString('default', { weekday: 'short', month: 'short', day: 'numeric' })}
                                </span>
                                {#if todayDay}
                                    <span class="badge preset-filled-primary-500 text-[10px]">Today</span>
                                {/if}
                            </div>
                            {#if dayTasks.length === 0}
                                <p class="text-xs text-surface-600 pl-2">No tasks</p>
                            {:else}
                                {#each dayTasks as t (t.id)}
                                    {@const cat = allCats.find((c) => c.id === t.categoryId)}
                                    <div class="p-3 mb-1.5 rounded-r-md border border-surface-700 bg-surface-900" style="border-left: 4px solid {cat?.color ?? '#46a758'}">
                                        <span class="text-sm text-surface-100">{t.title}</span>
                                    </div>
                                {/each}
                            {/if}
                        </div>
                    {/if}
                {/each}
            </div>
        {/if}
    </div>
</div>
