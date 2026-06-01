<script lang="ts">
    import { createQuery } from "@tanstack/svelte-query";
    import { _ } from "svelte-i18n";
    import TaskRow from "@/components/tasks/TaskRow.svelte";
    import CreateTaskSheet from "@/components/tasks/CreateTaskSheet.svelte";
    import { Menu, Portal } from "@skeletonlabs/skeleton-svelte";
    import {
        FunnelIcon,
        CheckIcon,
        EyeIcon,
        EyeClosedIcon,
    } from "lucide-svelte";
    import { categories, tasks } from "@/lib/api";
    import { isToday } from "@/lib/utils";

    const PRIORITY_LEGEND = [
        { label: "Low", color: "#46a758" },
        { label: "Medium", color: "#d97706" },
        { label: "High", color: "#ef4444" },
        { label: "Urgent", color: "#7c3aed" },
    ] as const;

    type Period = "all" | "today" | "week";

    let sheetOpen = $state(false);
    let filtersOpen = $state(false);
    let showCompleted = $state(false);

    // null = all selected (default). Explicit Set = user has filtered.
    let selectedCatIds = $state<Set<string> | null>(null);
    let selectedPeriod = $state<Period>("all");

    const tasksQuery = createQuery({
        queryKey: ["tasks"],
        queryFn: tasks.getAll,
    });
    const catsQuery = createQuery({
        queryKey: ["categories"],
        queryFn: categories.getAll,
    });

    let allTasks = $derived($tasksQuery.data?.tasks ?? []);
    let allCats = $derived($catsQuery.data?.categories ?? []);

    let allOptionIds = $derived([...allCats.map((c) => c.id), "none"]);

    function isCatSelected(id: string): boolean {
        return selectedCatIds === null || selectedCatIds.has(id);
    }

    function toggleCat(id: string) {
        console.log("[cat] toggleCat called, id=", id, "selectedCatIds=", selectedCatIds);
        if (id === "all") {
            // toggle: if all selected → deselect all; otherwise → select all
            selectedCatIds = selectedCatIds === null ? new Set() : null;
            console.log("[cat] after all toggle, selectedCatIds=", selectedCatIds);
            return;
        }

        if (selectedCatIds === null) {
            // deselect one: create set with all except this
            selectedCatIds = new Set(allOptionIds.filter((i) => i !== id));
        } else {
            const next = new Set(selectedCatIds);
            if (next.has(id)) {
                next.delete(id);
            } else {
                next.add(id);
            }
            // if all selected again, reset to null
            selectedCatIds = next.size === allOptionIds.length ? null : next;
        }
    }

    function getTaskTs(task: (typeof allTasks)[number]): number | null {
        const s = task.schedule;
        if (s.type === "allDay") return s.date;
        if (s.type === "at" || s.type === "span") return s.startsAt;
        return null;
    }

    function isThisWeek(ts: number): boolean {
        const d = new Date(ts * 1000);
        const now = new Date();
        const day = now.getDay();
        const startOfWeek = new Date(now);
        startOfWeek.setDate(now.getDate() - (day === 0 ? 6 : day - 1));
        startOfWeek.setHours(0, 0, 0, 0);
        const endOfWeek = new Date(startOfWeek);
        endOfWeek.setDate(startOfWeek.getDate() + 7);
        return d >= startOfWeek && d < endOfWeek;
    }

    let periodFilteredTasks = $derived(
        (() => {
            console.log("[period] selectedPeriod=", selectedPeriod, "allTasks.length=", allTasks.length);
            if (selectedPeriod === "all") return allTasks;
            const result = allTasks.filter((t) => {
                const ts = getTaskTs(t);
                console.log("[period] task", t.id, "schedule.type=", t.schedule.type, "ts=", ts);
                if (ts === null) return false;
                const matches = selectedPeriod === "today" ? isToday(ts) : isThisWeek(ts);
                console.log("[period] task", t.id, "matches=", matches);
                return matches;
            });
            console.log("[period] result.length=", result.length);
            return result;
        })()
    );

    let filteredTasks = $derived(
        selectedCatIds === null
            ? periodFilteredTasks
            : periodFilteredTasks.filter((t) =>
                  t.categoryId != null
                      ? selectedCatIds!.has(t.categoryId)
                      : selectedCatIds!.has("none"),
              ),
    );

    let active = $derived(filteredTasks.filter((t) => !t.completedAt));
    let done = $derived(filteredTasks.filter((t) => !!t.completedAt));

    let filterActive = $derived(
        selectedCatIds !== null || selectedPeriod !== "all",
    );

    let periodLabel = $derived(
        selectedPeriod === "today"
            ? "Oggi"
            : selectedPeriod === "week"
              ? "Questa settimana"
              : "Periodo",
    );
</script>

<div class="flex-1 min-h-0 flex flex-col overflow-hidden">
    <div class="flex-1 overflow-y-auto pb-24">
        {#if $tasksQuery.isPending}
            <div
                class="flex items-center justify-center py-16 text-surface-500 text-sm font-mono"
            >
                <span>{$_("common.loading")}</span>
            </div>
        {/if}

        {#if filtersOpen}
            <div class="flex flex-row gap-2 mb-3 px-2 pt-2">
                <div class="flex-1">
                    <Menu
                        onSelect={(e) => {
                            console.log("[period] onSelect fired, e=", e, "e.value=", e.value);
                            selectedPeriod = e.value as Period;
                            console.log("[period] selectedPeriod set to", selectedPeriod);
                        }}
                    >
                        <Menu.Trigger
                            class="btn preset-outlined w-full text-sm relative"
                        >
                            {periodLabel}
                            {#if selectedPeriod !== "all"}
                                <span
                                    class="absolute -top-1 -right-1 size-2 rounded-full bg-primary-500"
                                ></span>
                            {/if}
                        </Menu.Trigger>
                        <Portal>
                            <Menu.Positioner>
                                <Menu.Content
                                    class="bg-surface-900 border border-surface-700 rounded-md shadow-xl min-w-[160px]"
                                >
                                    <Menu.Item
                                        value="all"
                                        class="flex items-center gap-2 px-3 py-2 text-sm text-surface-100 hover:bg-surface-700 rounded cursor-pointer"
                                    >
                                        <Menu.ItemText class="flex-1"
                                            >Tutti</Menu.ItemText
                                        >
                                        {#if selectedPeriod === "all"}
                                            <CheckIcon class="size-4" />
                                        {/if}
                                    </Menu.Item>
                                    <Menu.Item
                                        value="today"
                                        class="flex items-center gap-2 px-3 py-2 text-sm text-surface-100 hover:bg-surface-700 rounded cursor-pointer"
                                    >
                                        <Menu.ItemText class="flex-1"
                                            >Oggi</Menu.ItemText
                                        >
                                        {#if selectedPeriod === "today"}
                                            <CheckIcon class="size-4" />
                                        {/if}
                                    </Menu.Item>
                                    <Menu.Item
                                        value="week"
                                        class="flex items-center gap-2 px-3 py-2 text-sm text-surface-100 hover:bg-surface-700 rounded cursor-pointer"
                                    >
                                        <Menu.ItemText class="flex-1"
                                            >Questa settimana</Menu.ItemText
                                        >
                                        {#if selectedPeriod === "week"}
                                            <CheckIcon class="size-4" />
                                        {/if}
                                    </Menu.Item>
                                </Menu.Content>
                            </Menu.Positioner>
                        </Portal>
                    </Menu>
                </div>
                <div class="flex-1">
                    <Menu onSelect={(e) => toggleCat(e.value)}>
                        <Menu.Trigger
                            class="btn preset-outlined w-full text-sm relative"
                        >
                            {$_("todo.category_filter")}
                            {#if filterActive}
                                <span
                                    class="absolute -top-1 -right-1 size-2 rounded-full bg-primary-500"
                                ></span>
                            {/if}
                        </Menu.Trigger>
                        <Portal>
                            <Menu.Positioner>
                                <Menu.Content
                                    class="bg-surface-900 border border-surface-700 rounded-md shadow-xl min-w-[160px]"
                                >
                                    {#each allCats as cat (cat.id)}
                                        {@const selected = isCatSelected(
                                            cat.id,
                                        )}
                                        <Menu.Item
                                            value={cat.id}
                                            class="flex items-center gap-2 px-3 py-2 text-sm text-surface-100 hover:bg-surface-700 rounded cursor-pointer"
                                        >
                                            <div
                                                class="size-2.5 rounded-full shrink-0"
                                                style="background:{cat.color}"
                                            ></div>
                                            <Menu.ItemText class="flex-1"
                                                >{cat.name}</Menu.ItemText
                                            >
                                            {#if selected}
                                                <CheckIcon class="size-4" />
                                            {/if}
                                        </Menu.Item>
                                    {/each}
                                    <Menu.Item
                                        value="none"
                                        class="flex items-center gap-2 px-3 py-2 text-sm text-surface-100 hover:bg-surface-700 rounded cursor-pointer"
                                    >
                                        <div
                                            class="size-2.5 rounded-full shrink-0 border border-surface-500 border-dashed"
                                        ></div>
                                        <Menu.ItemText class="flex-1"
                                            >{$_(
                                                "todo.no_category",
                                            )}</Menu.ItemText
                                        >
                                        {#if isCatSelected("none")}
                                            <CheckIcon class="size-4" />
                                        {/if}
                                    </Menu.Item>
                                    <Menu.Item
                                        value="all"
                                        class="flex items-center gap-2 px-3 py-2 text-sm text-surface-100 hover:bg-surface-700 rounded cursor-pointer"
                                    >
                                        <div
                                            class="size-2.5 rounded-full shrink-0"
                                        ></div>
                                        <Menu.ItemText class="flex-1"
                                            >{$_(
                                                "todo.all_categories",
                                            )}</Menu.ItemText
                                        >
                                        {#if selectedCatIds === null}
                                            <CheckIcon class="size-4" />
                                        {/if}
                                    </Menu.Item>
                                </Menu.Content>
                            </Menu.Positioner>
                        </Portal>
                    </Menu>
                </div>
                <button
                    type="button"
                    onclick={() => (filtersOpen = false)}
                    class="btn-icon preset-outlined"
                    aria-label="Chiudi filtri"
                >
                    <FunnelIcon size={18} />
                </button>
            </div>
        {:else}
            <div class="flex flex-row justify-end px-2 pt-2 mb-1">
                <button
                    type="button"
                    onclick={() => (showCompleted = !showCompleted)}
                    class="btn-icon preset-outlined relative mr-3"
                    aria-label="Apri filtri"
                >
                    {#if showCompleted}
                        <EyeIcon size={18} />
                    {:else}
                        <EyeClosedIcon size={18} />
                    {/if}
                </button>
                <button
                    type="button"
                    onclick={() => (filtersOpen = true)}
                    class="btn-icon preset-outlined relative"
                    aria-label="Apri filtri"
                >
                    <FunnelIcon size={18} />
                    {#if filterActive}
                        <span
                            class="absolute -top-0.5 -right-0.5 size-2 rounded-full bg-primary-500"
                        ></span>
                    {/if}
                </button>
            </div>
        {/if}

        {#each active as task (task.id)}
            <TaskRow {task} categories={allCats} />
        {/each}

        {#if done.length > 0 && showCompleted}
            <div class="px-4 pt-5 pb-1.5 flex items-center justify-between">
                <span
                    class="text-xs font-mono text-surface-500 tracking-widest uppercase"
                >
                    {$_("todo.completed")}
                </span>
                <span class="badge preset-tonal-surface text-[10px]"
                    >{done.length}</span
                >
            </div>
            {#each done as task (task.id)}
                <TaskRow {task} categories={allCats} />
            {/each}
        {/if}

        {#if !$tasksQuery.isPending && allTasks.length === 0}
            <div
                class="flex flex-col items-center justify-center py-20 gap-3 text-surface-500"
            >
                <svg
                    viewBox="0 0 20 20"
                    width="28"
                    height="28"
                    stroke="currentColor"
                    fill="none"
                    stroke-width="1.5"
                >
                    <path d="M3 6h14M3 10h10M3 14h7" stroke-linecap="round" />
                </svg>
                <p class="text-sm">{$_("todo.no_tasks")}</p>
            </div>
        {/if}

        {#if !$tasksQuery.isPending && allTasks.length > 0}
            <div class="mt-4 px-4 pb-3">
                <p class="text-[10px] font-mono text-surface-600 mb-2">
                    Priority legend
                </p>
                <div class="flex gap-3">
                    {#each PRIORITY_LEGEND as { label, color } (label)}
                        <div class="flex items-center gap-1">
                            <div
                                class="rounded-sm shrink-0"
                                style="background-color: {color}; width: 3px; height: 12px"
                            ></div>
                            <span class="text-[10px] text-surface-500"
                                >{label}</span
                            >
                        </div>
                    {/each}
                </div>
            </div>
        {/if}
    </div>

    <button
        onclick={() => (sheetOpen = true)}
        class="fixed bottom-[84px] right-4 z-20 size-12 rounded-full btn preset-filled-primary-500 shadow-lg"
        aria-label="New task"
    >
        <svg
            viewBox="0 0 16 16"
            width="20"
            height="20"
            stroke="currentColor"
            fill="none"
            stroke-width="2"
            stroke-linecap="round"
        >
            <line x1="8" y1="2" x2="8" y2="14" />
            <line x1="2" y1="8" x2="14" y2="8" />
        </svg>
    </button>

    <CreateTaskSheet open={sheetOpen} onClose={() => (sheetOpen = false)} />
</div>
