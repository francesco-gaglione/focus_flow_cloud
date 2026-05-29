<script lang="ts">
    import { createQuery } from "@tanstack/svelte-query";
    import { fmtDuration } from "$lib/utils";
    import { statsApi } from "@/lib/api";

    const DAYS = ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];
    const PRIORITY_COLORS: Record<string, string> = {
        urgent: "#7c3aed",
        high: "#ef4444",
        medium: "#d97706",
        low: "#46a758",
    };

    const statsQuery = createQuery({ queryKey: ["stats"], queryFn: statsApi.get });

    let today = new Date().toISOString().slice(0, 10);
</script>

{#if $statsQuery.isPending || !$statsQuery.data}
    <div class="flex-1 flex items-center justify-center text-surface-500 text-sm font-mono">
        Loading…
    </div>
{:else}
    {@const data = $statsQuery.data}
    {@const cts = data.completedTasksCounts}
    {@const last14d = data.last14d}
    {@const last8w = data.last8w}
    {@const completedFocusSessions = data.completedFocusSessions}
    {@const overdueTrend = data.overdueTrend}
    {@const peakWindow = data.peakWindow}
    {@const completedByPriority = data.completedByPriority}
    {@const countByCategory = data.countByCategory as { categoryId: string; categoryName: string; count: number }[]}
    {@const max14d = Math.max(...last14d.map((d: { count: number }) => d.count), 1)}
    {@const max8w = Math.max(...last8w.map((w: { count: number }) => w.count), 1)}
    {@const totalByPriority = (Object.values(completedByPriority) as number[]).reduce((a, b) => a + b, 0) || 1}
    {@const topCats = [...countByCategory].sort((a, b) => b.count - a.count).slice(0, 6)}
    {@const maxCatCount = Math.max(...topCats.map(c => c.count), 1)}

    <div class="flex-1 min-h-0 flex flex-col overflow-hidden">
        <div class="flex-1 overflow-y-auto pb-32 px-4 pt-3 flex flex-col gap-3">

            <!-- Tasks summary -->
            <div class="card bg-surface-900 border border-surface-700 p-4">
                <div class="flex items-start justify-between mb-1">
                    <span class="text-xs font-mono text-surface-500 uppercase tracking-widest">Tasks completed</span>
                    {#if cts.weekDelta !== 0}
                        <span class={`text-[10px] font-mono ${cts.weekDelta > 0 ? "text-green-400" : "text-red-400"}`}>
                            {cts.weekDelta > 0 ? "+" : ""}{cts.weekDelta}% vs last wk
                        </span>
                    {/if}
                </div>
                <div class="flex items-baseline gap-1.5 mt-1">
                    <span class="text-4xl font-bold text-surface-50">{cts.completedToday}</span>
                    <span class="text-sm text-surface-500">today</span>
                </div>
                <div class="grid grid-cols-3 gap-2 mt-3 pt-3 border-t border-surface-800">
                    <div class="text-center">
                        <p class="text-lg font-bold text-surface-100">{cts.completedThisWeek}</p>
                        <p class="text-[10px] text-surface-500 font-mono">this week</p>
                    </div>
                    <div class="text-center border-x border-surface-800">
                        <p class="text-lg font-bold text-surface-100">{cts.completedThisMonth}</p>
                        <p class="text-[10px] text-surface-500 font-mono">this month</p>
                    </div>
                    <div class="text-center">
                        <p class="text-lg font-bold text-surface-100">{cts.dayAvg.toFixed(1)}</p>
                        <p class="text-[10px] text-surface-500 font-mono">daily avg</p>
                    </div>
                </div>
            </div>

            <!-- Focus sessions -->
            <div class="grid grid-cols-2 gap-3">
                <div class="card bg-surface-900 border border-surface-700 p-4">
                    <p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-2">Sessions</p>
                    <span class="text-3xl font-bold text-surface-50">{completedFocusSessions.count}</span>
                    <p class="text-xs text-surface-500 mt-1">focus sessions</p>
                </div>
                <div class="card bg-surface-900 border border-surface-700 p-4">
                    <p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-2">Avg duration</p>
                    <span class="text-3xl font-bold text-surface-50">{fmtDuration(completedFocusSessions.avgDurationSecs)}</span>
                    <p class="text-xs text-surface-500 mt-1">per session</p>
                </div>
            </div>

            <!-- Last 14 days -->
            <div class="card bg-surface-900 border border-surface-700 p-4">
                <p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-3">Last 14 days</p>
                <div class="flex items-end gap-1 h-[72px]">
                    {#each last14d as d (d.day)}
                        {@const isTodayBar = d.day === today}
                        {@const h = Math.max(2, (d.count / max14d) * 64)}
                        <div class="flex-1 flex flex-col items-center justify-end">
                            <div
                                class={`w-full rounded-sm transition-all ${isTodayBar ? "bg-primary-500" : d.count === 0 ? "bg-surface-800" : "bg-primary-500/40"}`}
                                style="height:{h}px"
                            ></div>
                        </div>
                    {/each}
                </div>
                <div class="flex gap-1 mt-1">
                    {#each last14d as d (d.day)}
                        {@const isTodayBar = d.day === today}
                        {@const date = new Date(d.day + "T00:00:00")}
                        <div class="flex-1 text-center">
                            <span class={`text-[8px] font-mono ${isTodayBar ? "text-primary-400" : "text-surface-600"}`}>
                                {date.getDate()}/{date.getMonth() + 1}
                            </span>
                        </div>
                    {/each}
                </div>
            </div>

            <!-- Last 8 weeks -->
            <div class="card bg-surface-900 border border-surface-700 p-4">
                <p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-3">Last 8 weeks</p>
                <div class="flex items-end gap-1.5 h-[72px]">
                    {#each last8w as w, i (i)}
                        {@const h = Math.max(2, (w.count / max8w) * 64)}
                        {@const isCurrentWeek = i === last8w.length - 1}
                        <div class="flex-1 flex flex-col items-center justify-end">
                            <div
                                class={`w-full rounded-sm transition-all ${isCurrentWeek ? "bg-primary-500" : w.count === 0 ? "bg-surface-800" : "bg-primary-500/40"}`}
                                style="height:{h}px"
                            ></div>
                        </div>
                    {/each}
                </div>
                <div class="flex gap-1.5 mt-1">
                    {#each last8w as w, i (i)}
                        {@const isCurrentWeek = i === last8w.length - 1}
                        {@const date = new Date(w.weekStart + "T00:00:00")}
                        <div class="flex-1 text-center">
                            {#if i % 2 === 0 || isCurrentWeek}
                                <span class={`text-[8px] font-mono ${isCurrentWeek ? "text-primary-400" : "text-surface-600"}`}>
                                    {date.getDate()}/{date.getMonth() + 1}
                                </span>
                            {/if}
                        </div>
                    {/each}
                </div>
            </div>

            <!-- By priority -->
            <div class="card bg-surface-900 border border-surface-700 p-4">
                <p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-3">By priority</p>
                <div class="flex flex-col gap-2">
                    {#each ["urgent", "high", "medium", "low"] as const as p (p)}
                        {@const count = completedByPriority[p] as number}
                        {@const pct = Math.round((count / totalByPriority) * 100)}
                        <div class="flex items-center gap-2">
                            <span class="text-xs text-surface-400 w-14 capitalize">{p}</span>
                            <div class="flex-1 h-1.5 bg-surface-800 rounded-full overflow-hidden">
                                <div class="h-full rounded-full transition-all" style="width:{pct}%; background:{PRIORITY_COLORS[p]}"></div>
                            </div>
                            <span class="text-xs font-mono text-surface-500 w-12 text-right">{count} ({pct}%)</span>
                        </div>
                    {/each}
                </div>
            </div>

            <!-- By category -->
            {#if topCats.length > 0}
                <div class="card bg-surface-900 border border-surface-700 p-4">
                    <p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-3">By category</p>
                    <div class="flex flex-col gap-2">
                        {#each topCats as cat (cat.categoryId)}
                            {@const pct = (cat.count / maxCatCount) * 100}
                            <div class="flex items-center gap-2">
                                <span class="text-xs text-surface-400 flex-1 truncate">{cat.categoryName}</span>
                                <div class="w-24 h-1.5 bg-surface-800 rounded-full overflow-hidden">
                                    <div class="h-full rounded-full bg-primary-500/60 transition-all" style="width:{pct}%"></div>
                                </div>
                                <span class="text-xs font-mono text-surface-500 w-6 text-right">{cat.count}</span>
                            </div>
                        {/each}
                    </div>
                </div>
            {/if}

            <!-- Peak hours -->
            {#if peakWindow.length > 0}
                <div class="card bg-surface-900 border border-surface-700 p-4">
                    <p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-3">Peak hours</p>
                    <div class="flex flex-col gap-2">
                        {#each peakWindow.slice(0, 5) as w, i (i)}
                            {@const maxCount = Math.max(...peakWindow.map((x: { count: number }) => x.count), 1)}
                            {@const pct = (w.count / maxCount) * 100}
                            <div class="flex items-center gap-2">
                                <span class="text-xs font-mono text-surface-400 w-12">{w.start.slice(0, 5)}</span>
                                <div class="flex-1 h-1.5 bg-surface-800 rounded-full overflow-hidden">
                                    <div class={`h-full rounded-full ${i === 0 ? "bg-primary-500" : "bg-primary-500/40"}`} style="width:{pct}%"></div>
                                </div>
                                <span class="text-xs font-mono text-surface-500 w-4 text-right">{w.count}</span>
                            </div>
                        {/each}
                    </div>
                </div>
            {/if}

            <!-- Overdue trend -->
            <div class="card bg-surface-900 border border-surface-700 p-4">
                <p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-3">Overdue trend</p>
                <div class="flex items-center gap-3">
                    <span class={`text-2xl font-bold ${overdueTrend.trendType === "DECREASING" ? "text-green-400" : overdueTrend.trendType === "INCREASING" ? "text-red-400" : "text-surface-400"}`}>
                        {overdueTrend.trendType === "DECREASING" ? "↓" : overdueTrend.trendType === "INCREASING" ? "↑" : "→"}
                    </span>
                    <div>
                        <p class="text-sm text-surface-200 capitalize">{overdueTrend.trendType.toLowerCase()}</p>
                        <p class="text-xs text-surface-500">{Math.abs(overdueTrend.trendValue).toFixed(1)}% change</p>
                    </div>
                </div>
            </div>

        </div>
    </div>
{/if}
