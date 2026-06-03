<script lang="ts">
    import { createQuery } from "@tanstack/svelte-query";
    import { writable } from "svelte/store";
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

    const PRIORITY_LEGEND = [
        { label: "Low", color: "#46a758" },
        { label: "Medium", color: "#d97706" },
        { label: "High", color: "#ef4444" },
        { label: "Urgent", color: "#7c3aed" },
    ] as const;

    let sheetOpen = $state(false);
    let filtersOpen = $state(false);
    let showCompleted = $state(false);

    let selectedCatIds = $state<Set<string> | null>(null);

    let openSections = $state({
        today: true,
        nextWeek: false,
        incoming: false,
        unscheduled: false,
    });

    const tasksQueryOpts = writable({
        queryKey: ["tasks", { ...openSections, completed: showCompleted }],
        queryFn: () =>
            tasks.getAll(
                showCompleted,
                openSections.today,
                openSections.nextWeek,
                openSections.unscheduled,
                openSections.incoming,
            ),
        placeholderData: (prev: any) => prev,
        staleTime: 30_000,
    });

    $effect(() => {
        const s = { ...openSections };
        const c = showCompleted;
        tasksQueryOpts.set({
            queryKey: ["tasks", { ...s, completed: c }],
            queryFn: () =>
                tasks.getAll(c, s.today, s.nextWeek, s.unscheduled, s.incoming),
            placeholderData: (prev: any) => prev,
            staleTime: 30_000,
        });
    });

    const tasksQuery = createQuery(tasksQueryOpts);

    // Fetch available categories
    const catsQuery = createQuery({
        queryKey: ["categories"],
        queryFn: categories.getAll,
    });

    let allCats = $derived($catsQuery.data?.categories ?? []);
    let allOptionIds = $derived([...allCats.map((c) => c.id), "none"]);

    function isCatSelected(id: string): boolean {
        return selectedCatIds === null || selectedCatIds.has(id);
    }

    function toggleCat(id: string) {
        console.log(
            "[cat] toggleCat called, id=",
            id,
            "selectedCatIds=",
            selectedCatIds,
        );
        if (id === "all") {
            // Toggle all: if currently filtered, reset to null (all selected)
            selectedCatIds = selectedCatIds === null ? new Set() : null;
            console.log(
                "[cat] after all toggle, selectedCatIds=",
                selectedCatIds,
            );
            return;
        }

        if (selectedCatIds === null) {
            // Deselect one: create a new set containing everything except the toggled id
            selectedCatIds = new Set(allOptionIds.filter((i) => i !== id));
        } else {
            const next = new Set(selectedCatIds);
            if (next.has(id)) {
                next.delete(id);
            } else {
                next.add(id);
            }
            // If the updated set includes all items, reset it to null
            selectedCatIds = next.size === allOptionIds.length ? null : next;
        }
    }

    // Map the API payload structure including the new unscheduled tasks group
    let taskGroups = $derived(
        $tasksQuery.data || {
            today: [],
            nextWeek: [],
            incoming: [],
            unscheduled: [],
            completed: [],
        },
    );

    // Filter a task list based on the selected categories
    function filterByCategory(taskList: any[] | undefined) {
        if (!taskList) return [];
        if (selectedCatIds === null) return taskList;
        return taskList.filter((t) =>
            t.categoryId != null
                ? selectedCatIds!.has(t.categoryId)
                : selectedCatIds!.has("none"),
        );
    }

    // Derived reactive filtered task groups
    let todayTasks = $derived(filterByCategory(taskGroups.today));
    let nextWeekTasks = $derived(filterByCategory(taskGroups.nextWeek));
    let incomingTasks = $derived(filterByCategory(taskGroups.incoming));
    let unscheduledTasks = $derived(filterByCategory(taskGroups.unscheduled));
    let completedTasks = $derived(filterByCategory(taskGroups.completed));

    // Dynamic task counter taking into account all visible groups
    let totalTasksCount = $derived(
        todayTasks.length +
            nextWeekTasks.length +
            incomingTasks.length +
            unscheduledTasks.length +
            (showCompleted ? completedTasks.length : 0),
    );

    let filterActive = $derived(selectedCatIds !== null);
</script>

{#snippet taskSection(
    title: string,
    tasksList: any[],
    isOpen: boolean,
    onToggle: (v: boolean) => void,
)}
    <details
        class="group"
        open={isOpen}
        ontoggle={(e) => onToggle((e.currentTarget as HTMLDetailsElement).open)}
    >
        <summary
            class="flex items-center justify-between px-4 py-3 cursor-pointer select-none list-none [&::-webkit-details-marker]:hidden"
        >
            <span
                class="text-xs font-mono text-surface-500 tracking-widest uppercase"
            >
                {title}
            </span>
            <div class="flex items-center gap-2 text-surface-500">
                {#if tasksList.length > 0}
                    <span class="badge preset-tonal-surface text-[10px]"
                        >{tasksList.length}</span
                    >
                {/if}
                <svg
                    class="size-4 transition-transform group-open:rotate-180"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <path d="m6 9 6 6 6-6" />
                </svg>
            </div>
        </summary>
        <div class="pb-2">
            {#if $tasksQuery.isFetching && tasksList.length === 0}
                <div class="px-4 py-2 text-xs text-surface-500 font-mono">
                    …
                </div>
            {:else}
                {#each tasksList as task (task.id)}
                    <TaskRow {task} categories={allCats} />
                {/each}
            {/if}
        </div>
    </details>
{/snippet}

<div class="flex-1 min-h-0 flex flex-col overflow-hidden">
    <div class="flex-1 overflow-y-auto pb-20">
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
                    aria-label="Close filters"
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
                    aria-label="Toggle completed tasks"
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
                    aria-label="Open filters"
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

        {@render taskSection(
            "Today",
            todayTasks,
            openSections.today,
            (v) => (openSections.today = v),
        )}
        {@render taskSection(
            "Next Week",
            nextWeekTasks,
            openSections.nextWeek,
            (v) => (openSections.nextWeek = v),
        )}
        {@render taskSection(
            "Incoming",
            incomingTasks,
            openSections.incoming,
            (v) => (openSections.incoming = v),
        )}
        {@render taskSection(
            "Unscheduled",
            unscheduledTasks,
            openSections.unscheduled,
            (v) => (openSections.unscheduled = v),
        )}

        {#if showCompleted}
            {@render taskSection(
                $_("todo.completed"),
                completedTasks,
                true,
                () => {},
            )}
        {/if}

        {#if !$tasksQuery.isPending && totalTasksCount === 0}
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

        {#if !$tasksQuery.isPending && totalTasksCount > 0}
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
        class="fixed right-4 z-30 size-12 rounded-full btn preset-filled-primary-500 shadow-lg"
        style="bottom: calc(env(safe-area-inset-bottom, 0px) + 68px)"
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
