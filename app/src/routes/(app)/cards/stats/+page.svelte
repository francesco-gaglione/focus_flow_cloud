<script lang="ts">
    import { createQuery } from "@tanstack/svelte-query";
    import { flashcardsApi } from "@/lib/api";
    import type { ActivityEntryDto } from "@/types";

    const statsQuery = createQuery({
        queryKey: ["flashcards", "stats"],
        queryFn: flashcardsApi.getGlobalStats,
    });

    const heatmapQuery = createQuery({
        queryKey: ["flashcards", "stats", "activity"],
        queryFn: () => flashcardsApi.getActivityHeatmap(90),
    });

    function retentionColor(rate: number): string {
        if (rate >= 0.85) return "text-green-400";
        if (rate >= 0.65) return "text-yellow-400";
        return "text-red-400";
    }

    // Build a 90-day grid for the heatmap
    function buildHeatmapGrid(entries: ActivityEntryDto[]) {
        const today = new Date();
        const map = new Map<string, number>();
        for (const e of entries) {
            map.set(e.date, e.count);
        }
        const cells: { date: string; count: number }[] = [];
        for (let i = 89; i >= 0; i--) {
            const d = new Date(today);
            d.setDate(today.getDate() - i);
            const key = d.toISOString().slice(0, 10);
            cells.push({ date: key, count: map.get(key) ?? 0 });
        }
        return cells;
    }

    function heatmapCellColor(count: number): string {
        if (count === 0) return "bg-surface-800";
        if (count <= 3) return "bg-primary-900";
        if (count <= 8) return "bg-primary-700";
        if (count <= 15) return "bg-primary-500";
        return "bg-primary-300";
    }

    let heatmapCells = $derived(
        $heatmapQuery.data ? buildHeatmapGrid($heatmapQuery.data.entries) : []
    );
</script>

{#if $statsQuery.isPending || !$statsQuery.data}
    <div class="flex-1 flex items-center justify-center text-surface-500 text-sm font-mono">
        Loading…
    </div>
{:else}
    {@const s = $statsQuery.data}

    <div class="flex-1 min-h-0 flex flex-col overflow-hidden">
        <div class="flex-1 overflow-y-auto pb-24 px-4 pt-3 flex flex-col gap-3">

            <!-- Summary cards -->
            <div class="grid grid-cols-2 gap-3">
                <div class="card bg-surface-900 border border-surface-700 p-4">
                    <p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-2">Total cards</p>
                    <span class="text-3xl font-bold text-surface-50">{s.totalCards}</span>
                </div>
                <div class="card bg-surface-900 border border-surface-700 p-4">
                    <p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-2">Due today</p>
                    <span class="text-3xl font-bold {s.dueToday > 0 ? 'text-warning-400' : 'text-surface-50'}">{s.dueToday}</span>
                </div>
            </div>

            <div class="grid grid-cols-2 gap-3">
                <div class="card bg-surface-900 border border-surface-700 p-4">
                    <p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-2">Reviewed today</p>
                    <span class="text-3xl font-bold text-surface-50">{s.reviewedToday}</span>
                </div>
                <div class="card bg-surface-900 border border-surface-700 p-4">
                    <p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-2">Retention (30d)</p>
                    <span class="text-3xl font-bold {retentionColor(s.retentionRate30d)}">
                        {(s.retentionRate30d * 100).toFixed(0)}%
                    </span>
                </div>
            </div>

            <!-- Progress bar: reviewed today / due today -->
            {#if s.dueToday > 0}
                <div class="card bg-surface-900 border border-surface-700 p-4">
                    <div class="flex justify-between items-center mb-2">
                        <span class="text-xs font-mono text-surface-500 uppercase tracking-widest">Today's progress</span>
                        <span class="text-xs font-mono text-surface-400">
                            {s.reviewedToday} / {s.dueToday}
                        </span>
                    </div>
                    <div class="h-2 bg-surface-800 rounded-full overflow-hidden">
                        <div
                            class="h-full bg-primary-500 rounded-full transition-all"
                            style="width: {Math.min(100, (s.reviewedToday / s.dueToday) * 100).toFixed(1)}%"
                        ></div>
                    </div>
                </div>
            {/if}

            <!-- Activity heatmap -->
            <div class="card bg-surface-900 border border-surface-700 p-4">
                <p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-3">Activity (90 days)</p>

                {#if $heatmapQuery.isPending}
                    <div class="text-xs text-surface-500 font-mono">Loading…</div>
                {:else if heatmapCells.length > 0}
                    <div class="flex flex-wrap gap-0.5">
                        {#each heatmapCells as cell (cell.date)}
                            <div
                                class="size-3 rounded-sm {heatmapCellColor(cell.count)}"
                                title="{cell.date}: {cell.count} reviews"
                            ></div>
                        {/each}
                    </div>
                    <div class="flex items-center gap-1 mt-2 justify-end">
                        <span class="text-[9px] text-surface-600 font-mono">less</span>
                        <div class="size-2.5 rounded-sm bg-surface-800"></div>
                        <div class="size-2.5 rounded-sm bg-primary-900"></div>
                        <div class="size-2.5 rounded-sm bg-primary-700"></div>
                        <div class="size-2.5 rounded-sm bg-primary-500"></div>
                        <div class="size-2.5 rounded-sm bg-primary-300"></div>
                        <span class="text-[9px] text-surface-600 font-mono">more</span>
                    </div>
                {/if}
            </div>

        </div>
    </div>
{/if}
