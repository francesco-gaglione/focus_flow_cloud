<script lang="ts">
    import {
        createMutation,
        createQuery,
        useQueryClient,
    } from "@tanstack/svelte-query";
    import type {
        CreateTaskDto,
        GetAllCategoryResponseDto,
        TaskPriority,
        TaskScheduleDto,
    } from "@/types";
    import { categories, tasks } from "@/lib/api";

    const { open, onClose }: { open: boolean; onClose: () => void } = $props();

    const PRIORITY_COLORS: Record<TaskPriority, string> = {
        low: "#46a758",
        medium: "#d97706",
        high: "#ef4444",
        urgent: "#7c3aed",
    };

    type ScheduleType = "Unscheduled" | "AllDay" | "At" | "Span";

    const qc = useQueryClient();

    let title = $state("");
    let description = $state("");
    let priority = $state<TaskPriority | null>(null);
    let categoryId = $state("");
    let scheduleType = $state<ScheduleType>("Unscheduled");
    let scheduleDate = $state("");
    let scheduleTime = $state("");
    let duration = $state(25);
    let subtasks = $state<string[]>([]);
    let subtaskInput = $state("");

    const catsQuery = createQuery<GetAllCategoryResponseDto>({
        queryKey: ["categories"],
        queryFn: categories.getAll,
    });

    let cats = $derived($catsQuery.data?.categories ?? []);

    function buildSchedule(): TaskScheduleDto {
        if (scheduleType === "Unscheduled") return { type: "unscheduled" };
        if (scheduleType === "AllDay") {
            const ts = scheduleDate
                ? Math.floor(new Date(scheduleDate).getTime() / 1000)
                : Math.floor(Date.now() / 1000);
            return { type: "allDay", date: ts };
        }
        const ts = new Date(
            `${scheduleDate}T${scheduleTime || "00:00"}`,
        ).getTime();
        if (scheduleType === "At")
            return { type: "at", startsAt: Math.floor(ts / 1000) };
        return {
            type: "span",
            startsAt: Math.floor(ts / 1000),
            duration: duration * 60,
        };
    }

    const create = createMutation({
        mutationFn: () => {
            const dto: CreateTaskDto = {
                title: title.trim(),
                description: description.trim() || null,
                priority: priority,
                categoryId: categoryId || null,
                schedule: buildSchedule(),
                subtasks: subtasks.map((t) => ({
                    title: t,
                    description: null,
                })),
            };
            return tasks.create(dto);
        },
        onSuccess: () => {
            qc.invalidateQueries({ queryKey: ["tasks"] });
            handleClose();
        },
    });

    function handleClose() {
        title = "";
        description = "";
        priority = null;
        categoryId = "";
        scheduleType = "Unscheduled";
        scheduleDate = "";
        scheduleTime = "";
        duration = 25;
        subtasks = [];
        subtaskInput = "";
        onClose();
    }

    function addSubtask() {
        if (subtaskInput.trim()) {
            subtasks = [...subtasks, subtaskInput.trim()];
            subtaskInput = "";
        }
    }

    function removeSubtask(i: number) {
        subtasks = subtasks.filter((_, j) => j !== i);
    }
</script>

{#if open}
    <div
        class="fixed inset-0 z-30 bg-black/60"
        onclick={handleClose}
        role="presentation"
    ></div>
    <div
        class="fixed bottom-0 left-0 right-0 z-40 bg-surface-900 rounded-t-2xl border-t border-surface-700 flex flex-col max-h-[90vh]"
    >
        <div class="flex justify-center pt-3 pb-1 shrink-0">
            <div class="w-10 h-1 rounded-full bg-surface-600"></div>
        </div>

        <div class="flex items-center justify-between px-4 pb-3 shrink-0">
            <p class="text-sm font-semibold text-surface-100">New Task</p>
            <button
                onclick={handleClose}
                class="btn btn-icon preset-tonal-surface size-7"
                aria-label="Close"
            >
                <svg
                    viewBox="0 0 14 14"
                    width="12"
                    height="12"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                >
                    <line x1="2" y1="2" x2="12" y2="12" />
                    <line x1="12" y1="2" x2="2" y2="12" />
                </svg>
            </button>
        </div>

        <div class="overflow-y-auto flex-1 px-4 pb-4 flex flex-col gap-4">
            <div>
                <label
                    for="task-title"
                    class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-1 block"
                    >Title *</label
                >
                <input
                    id="task-title"
                    class="input w-full text-sm"
                    placeholder="Task title"
                    bind:value={title}
                />
            </div>

            <div>
                <label
                    for="task-desc"
                    class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-1 block"
                    >Description</label
                >
                <textarea
                    id="task-desc"
                    class="textarea w-full text-sm resize-none"
                    placeholder="Optional description"
                    rows={2}
                    bind:value={description}
                ></textarea>
            </div>

            <div>
                <p
                    class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-2"
                >
                    Priority
                </p>
                <div class="flex gap-2">
                    {#each ["low", "medium", "high", "urgent"] as TaskPriority[] as p (p)}
                        <button
                            onclick={() =>
                                (priority = priority === p ? null : p)}
                            class="chip text-[11px] capitalize flex-1"
                            style="background-color: {priority === p
                                ? PRIORITY_COLORS[p]
                                : 'transparent'}; color: {priority === p
                                ? '#fff'
                                : PRIORITY_COLORS[
                                      p
                                  ]}; border: 1px solid {PRIORITY_COLORS[p]}"
                        >
                            {p}
                        </button>
                    {/each}
                </div>
            </div>

            {#if cats.length > 0}
                <div>
                    <label
                        for="task-cat"
                        class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-1 block"
                        >Category</label
                    >
                    <select
                        id="task-cat"
                        class="select w-full text-sm"
                        bind:value={categoryId}
                    >
                        <option value="">None</option>
                        {#each cats as c (c.id)}
                            <option value={c.id}>{c.name}</option>
                        {/each}
                    </select>
                </div>
            {/if}

            <div>
                <p
                    class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-2"
                >
                    Schedule
                </p>
                <div class="flex gap-1.5 mb-3 flex-wrap">
                    {#each ["Unscheduled", "AllDay", "At", "Span"] as ScheduleType[] as t (t)}
                        <button
                            onclick={() => (scheduleType = t)}
                            class={`chip text-[11px] ${scheduleType === t ? "preset-filled-primary-500" : "preset-tonal-surface"}`}
                        >
                            {t === "Unscheduled"
                                ? "None"
                                : t === "AllDay"
                                  ? "All Day"
                                  : t}
                        </button>
                    {/each}
                </div>
                {#if scheduleType === "AllDay"}
                    <input
                        type="date"
                        class="input w-full text-sm"
                        bind:value={scheduleDate}
                    />
                {/if}
                {#if scheduleType === "At" || scheduleType === "Span"}
                    <div class="flex gap-2">
                        <input
                            type="date"
                            class="input flex-1 text-sm"
                            bind:value={scheduleDate}
                        />
                        <input
                            type="time"
                            class="input flex-1 text-sm"
                            bind:value={scheduleTime}
                        />
                    </div>
                {/if}
                {#if scheduleType === "Span"}
                    <div class="flex items-center gap-2 mt-2">
                        <label
                            for="task-dur"
                            class="text-xs text-surface-500 shrink-0"
                            >Duration (min)</label
                        >
                        <input
                            id="task-dur"
                            type="number"
                            class="input flex-1 text-sm"
                            min={1}
                            bind:value={duration}
                        />
                    </div>
                {/if}
            </div>

            <div>
                <p
                    class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-2"
                >
                    Subtasks
                </p>
                {#each subtasks as s, i (i)}
                    <div class="flex items-center gap-2 mb-1.5">
                        <span class="text-xs text-surface-300 flex-1 truncate"
                            >{s}</span
                        >
                        <button
                            onclick={() => removeSubtask(i)}
                            class="btn btn-icon preset-tonal-error size-6 shrink-0"
                            aria-label="Remove subtask"
                        >
                            <svg
                                viewBox="0 0 12 12"
                                width="10"
                                height="10"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="1.8"
                                stroke-linecap="round"
                            >
                                <line x1="2" y1="2" x2="10" y2="10" />
                                <line x1="10" y1="2" x2="2" y2="10" />
                            </svg>
                        </button>
                    </div>
                {/each}
                <div class="flex gap-2">
                    <input
                        class="input flex-1 text-sm"
                        placeholder="Add subtask"
                        bind:value={subtaskInput}
                        onkeydown={(e) => {
                            if (e.key === "Enter") addSubtask();
                        }}
                    />
                    <button
                        onclick={addSubtask}
                        disabled={!subtaskInput.trim()}
                        class="btn preset-tonal-surface text-sm px-3 disabled:opacity-50"
                    >
                        Add
                    </button>
                </div>
            </div>
        </div>

        <div class="px-4 py-3 border-t border-surface-800 shrink-0">
            <button
                onclick={() => title.trim() && $create.mutate()}
                disabled={!title.trim() || $create.isPending}
                class="btn preset-filled-primary-500 w-full disabled:opacity-50"
            >
                {$create.isPending ? "Creating…" : "Create Task"}
            </button>
        </div>
    </div>
{/if}
