<script lang="ts">
    import { createQuery } from "@tanstack/svelte-query";
    import { fmtDuration } from "$lib/utils";
    import { statsApi } from "@/lib/api";

    const DAYS = ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];
    const PRIORITY_COLORS: Record<string, string> = {
        urgent: "#7c3aed",
        high: "#ef4444",
        medium: "#d97706",
        low: "#6b7280",
    };

    const statsQuery = createQuery({
        queryKey: ["stats"],
        queryFn: statsApi.get,
    });

    let today = new Date().toISOString().slice(0, 10);
</script>

{#if $statsQuery.isPending || !$statsQuery.data}
    <div
        class="flex-1 flex items-center justify-center text-surface-500 text-sm font-mono"
    >
        Loading…
    </div>
{:else}
    {@const data = $statsQuery.data}
    {@const cts = data.completedTasksCounts}
    {@const last14d = data.last14d}
    {@const completedFocusSessions = data.completedFocusSessions}
    {@const overdueTrend = data.overdueTrend}
    {@const peakWindow = data.peakWindow}
    {@const completedByPriority = data.completedByPriority}
    {@const max14d = Math.max(
        ...last14d.map((d: { count: number }) => d.count),
        1,
    )}
    {@const totalByPriority =
        (Object.values(completedByPriority) as number[]).reduce(
            (a, b) => a + b,
            0,
        ) || 1}

    <div class="flex-1 min-h-0 flex flex-col overflow-hidden">
        <div class="flex-1 overflow-y-auto pb-32 px-4 pt-3 flex flex-col gap-3">
            <div class="card bg-surface-900 border border-surface-700 p-4">
                <div class="flex items-start justify-between mb-1">
                    <span
                        class="text-xs font-mono text-surface-500 uppercase tracking-widest"
                        >Tasks today</span
                    >
                    {#if cts.weekDelta !== 0}
                        <span
                            class={`text-[10px] font-mono ${cts.weekDelta > 0 ? "text-green-400" : "text-red-400"}`}
                        >
                            {cts.weekDelta > 0 ? "+" : ""}{cts.weekDelta}% vs
                            last wk
                        </span>
                    {/if}
                </div>
                <div class="flex items-baseline gap-1.5 mt-1">
                    <span class="text-4xl font-bold text-surface-50"
                        >{cts.completedToday}</span
                    >
                    <span class="text-sm text-surface-500">done</span>
                </div>
                <p class="text-xs text-surface-500 mt-1">
                    Daily avg: {cts.dayAvg.toFixed(1)} · Month: {cts.completedThisMonth}
                </p>
            </div>

            <div class="card bg-surface-900 border border-surface-700 p-4">
                <p
                    class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-3"
                >
                    Last 14 days
                </p>
                <div class="flex items-end gap-1 h-[80px]">
                    {#each last14d as d (d.day)}
                        {@const isTodayBar = d.day === today}
                        {@const h = Math.max(2, (d.count / max14d) * 72)}
                        <div
                            class="flex-1 flex flex-col items-center justify-end gap-1"
                        >
                            <div
                                class={`w-full rounded-sm transition-all ${isTodayBar ? "bg-primary-500" : d.count === 0 ? "bg-surface-800" : "bg-primary-500/40"}`}
                                style="height: {h}px"
                            ></div>
                        </div>
                    {/each}
                </div>
                <div class="flex gap-1 mt-1">
                    {#each last14d as d (d.day)}
                        {@const isTodayBar = d.day === today}
                        {@const dow =
                            (new Date(d.day + "T00:00:00").getDay() + 6) % 7}
                        <div class="flex-1 text-center">
                            <span
                                class={`text-[8px] font-mono ${isTodayBar ? "text-primary-400" : "text-surface-600"}`}
                            >
                                {DAYS[dow]}
                            </span>
                        </div>
                    {/each}
                </div>
            </div>

            <div class="grid grid-cols-2 gap-3">
                <div class="card bg-surface-900 border border-surface-700 p-4">
                    <p
                        class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-2"
                    >
                        Sessions
                    </p>
                    <span class="text-3xl font-bold text-surface-50"
                        >{completedFocusSessions.count}</span
                    >
                    <p class="text-xs text-surface-500 mt-1">focus sessions</p>
                </div>
                <div class="card bg-surface-900 border border-surface-700 p-4">
                    <p
                        class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-2"
                    >
                        Avg duration
                    </p>
                    <span class="text-3xl font-bold text-surface-50"
                        >{fmtDuration(
                            completedFocusSessions.avgDurationSecs,
                        )}</span
                    >
                    <p class="text-xs text-surface-500 mt-1">per session</p>
                </div>
            </div>

            <div class="card bg-surface-900 border border-surface-700 p-4">
                <p
                    class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-3"
                >
                    By priority
                </p>
                <div class="flex flex-col gap-2">
                    {#each ["urgent", "high", "medium", "low"] as const as p (p)}
                        {@const count = completedByPriority[p] as number}
                        {@const pct = Math.round(
                            (count / totalByPriority) * 100,
                        )}
                        <div class="flex items-center gap-2">
                            <span
                                class="text-xs text-surface-400 w-14 capitalize"
                                >{p}</span
                            >
                            <div
                                class="flex-1 h-1.5 bg-surface-800 rounded-full overflow-hidden"
                            >
                                <div
                                    class="h-full rounded-full"
                                    style="width: {pct}%; background: {PRIORITY_COLORS[
                                        p
                                    ]}"
                                ></div>
                            </div>
                            <span
                                class="text-xs font-mono text-surface-500 w-8 text-right"
                                >{pct}%</span
                            >
                        </div>
                    {/each}
                </div>
            </div>

            {#if peakWindow.length > 0}
                <div class="card bg-surface-900 border border-surface-700 p-4">
                    <p
                        class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-3"
                    >
                        Peak hours
                    </p>
                    <div class="flex flex-col gap-2">
                        {#each peakWindow.slice(0, 5) as w, i (i)}
                            {@const maxCount = Math.max(
                                ...peakWindow.map(
                                    (x: { count: number }) => x.count,
                                ),
                                1,
                            )}
                            {@const pct = (w.count / maxCount) * 100}
                            <div class="flex items-center gap-2">
                                <span
                                    class="text-xs font-mono text-surface-400 w-12"
                                    >{w.start.slice(0, 5)}</span
                                >
                                <div
                                    class="flex-1 h-1.5 bg-surface-800 rounded-full overflow-hidden"
                                >
                                    <div
                                        class={`h-full rounded-full ${i === 0 ? "bg-primary-500" : "bg-primary-500/40"}`}
                                        style="width: {pct}%"
                                    ></div>
                                </div>
                                <span
                                    class="text-xs font-mono text-surface-500 w-4 text-right"
                                    >{w.count}</span
                                >
                            </div>
                        {/each}
                    </div>
                </div>
            {/if}

            <div class="card bg-surface-900 border border-surface-700 p-4">
                <p
                    class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-3"
                >
                    Overdue trend
                </p>
                <div class="flex items-center gap-3">
                    <span
                        class={`text-2xl font-bold ${overdueTrend.trendType === "DECREASING" ? "text-green-400" : overdueTrend.trendType === "INCREASING" ? "text-red-400" : "text-surface-400"}`}
                    >
                        {overdueTrend.trendType === "DECREASING"
                            ? "↓"
                            : overdueTrend.trendType === "INCREASING"
                              ? "↑"
                              : "→"}
                    </span>
                    <div>
                        <p class="text-sm text-surface-200 capitalize">
                            {overdueTrend.trendType.toLowerCase()}
                        </p>
                        <p class="text-xs text-surface-500">
                            {Math.abs(overdueTrend.trendValue).toFixed(1)}%
                            change
                        </p>
                    </div>
                </div>
            </div>
        </div>
    </div>
{/if}
