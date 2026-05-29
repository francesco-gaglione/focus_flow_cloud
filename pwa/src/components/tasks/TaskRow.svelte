<script lang="ts">
    import { createMutation, useQueryClient } from "@tanstack/svelte-query";
    import { isToday, isOverdue, tsToDate } from "$lib/utils";
    import type {
        TaskDto,
        TaskPriority,
        CategoryDto,
        TaskScheduleDto,
    } from "@/types";
    import {
        ChevronDown,
        ChevronUp,
        EllipsisVerticalIcon,
    } from "lucide-svelte";
    import { tasks } from "@/lib/api";
    import { Menu, Portal } from "@skeletonlabs/skeleton-svelte";
    import { timerStore } from "@/lib/stores/timer";
    import { goto } from "$app/navigation";

    const {
        task,
        categories: cats,
    }: { task: TaskDto; categories: CategoryDto[] } = $props();

    const PRIORITY_BORDER: Record<TaskPriority, string> = {
        low: "#46a758",
        medium: "#d97706",
        high: "#ef4444",
        urgent: "#7c3aed",
    };

    const DATE_OPTS: Intl.DateTimeFormatOptions = {
        month: "short",
        day: "numeric",
    };
    const TIME_OPTS: Intl.DateTimeFormatOptions = {
        hour: "2-digit",
        minute: "2-digit",
    };

    function scheduleStartTs(schedule: TaskScheduleDto): number | undefined {
        if (schedule.type === "allDay") return schedule.date;
        if (schedule.type === "at") return schedule.startsAt;
        if (schedule.type === "span") return schedule.startsAt;
    }

    function scheduleLabel(
        schedule: TaskScheduleDto,
        overdue: boolean,
        today: boolean,
    ): string {
        if (schedule.type === "allDay") {
            if (overdue) return "Overdue";
            if (today) return "Today";
            return tsToDate(schedule.date).toLocaleDateString(
                undefined,
                DATE_OPTS,
            );
        }
        if (schedule.type === "at") {
            const t = tsToDate(schedule.startsAt).toLocaleTimeString(
                undefined,
                TIME_OPTS,
            );
            if (overdue) return `Overdue · ${t}`;
            if (today) return `Today · ${t}`;
            return `${tsToDate(schedule.startsAt).toLocaleDateString(undefined, DATE_OPTS)} · ${t}`;
        }
        if (schedule.type === "span") {
            const start = tsToDate(schedule.startsAt);
            const end = tsToDate(schedule.startsAt + schedule.duration);
            const range = `${start.toLocaleTimeString(undefined, TIME_OPTS)}–${end.toLocaleTimeString(undefined, TIME_OPTS)}`;
            if (overdue) return `Overdue · ${range}`;
            if (today) return `Today · ${range}`;
            return `${start.toLocaleDateString(undefined, DATE_OPTS)} · ${range}`;
        }
        return "";
    }

    const qc = useQueryClient();
    let expanded = $state(false);

    let cat = $derived(cats.find((c) => c.id === task.categoryId));
    let startTs = $derived(scheduleStartTs(task.schedule));
    let today = $derived(startTs ? isToday(startTs) : false);
    let overdue = $derived(
        startTs && !task.completedAt ? isOverdue(startTs) : false,
    );
    let completedSubtasks = $derived(
        task.subtasks.filter((s) => s.isCompleted).length,
    );
    let totalSubtasks = $derived(task.subtasks.length);

    const toggleDone = createMutation({
        mutationFn: () =>
            tasks.update(task.id, {
                completed: !task.completedAt,
                title: null,
                description: null,
                schedule: null,
                priority: null,
            }),
        onSuccess: () => {
            qc.invalidateQueries({ queryKey: ["tasks"] });
            qc.invalidateQueries({ queryKey: ["stats"] });
        },
    });

    const toggleSubtask = createMutation({
        mutationFn: ({
            subtaskId,
            isCompleted,
        }: {
            subtaskId: string;
            isCompleted: boolean;
        }) =>
            tasks.updateSubtask(task.id, subtaskId, {
                completed: isCompleted,
            }),
        onSuccess: () => qc.invalidateQueries({ queryKey: ["tasks"] }),
    });

    const deleteTask = createMutation({
        mutationFn: (id: string) => tasks.delete(id),
        onSuccess: () => qc.invalidateQueries({ queryKey: ["tasks"] }),
    });

    const menuSelection = (value: string) => {
        console.debug(value);
        if (value === "delete") {
            $deleteTask.mutate(task.id);
        } else if (value === "startTimer") {
            timerStore.setSelectedTask({ id: task.id, title: task.title });
            goto("/timer");
        }
    };
</script>

<div
    class={`border-b border-surface-800 ${task.completedAt ? "opacity-50" : ""}`}
    style="border-left: 3px solid {task.priority
        ? PRIORITY_BORDER[task.priority]
        : 'transparent'}"
>
    <div class="flex items-start gap-3 px-4 py-3">
        <input
            type="checkbox"
            class="checkbox mt-1 shrink-0 size-5"
            checked={!!task.completedAt}
            onchange={() => $toggleDone.mutate()}
            aria-label={task.completedAt ? "Mark incomplete" : "Mark complete"}
        />

        <div class="flex-1 min-w-0">
            <p
                class={`text-sm font-medium leading-snug ${task.completedAt ? "line-through text-surface-500" : "text-surface-50"}`}
            >
                {task.title}
            </p>
            {#if task.description}
                <p class="text-xs text-surface-500 mt-0.5 leading-snug">
                    {task.description}
                </p>
            {/if}
            <div class="flex flex-wrap items-center gap-1.5 mt-1.5">
                {#if cat}
                    <span
                        class="chip text-[10px]"
                        style="background-color: {cat.color}; color: #fff"
                    >
                        {cat.name}
                    </span>
                {/if}
                {#if task.schedule.type !== "unscheduled"}
                    <span
                        class={`chip text-[10px] ${overdue ? "preset-tonal-error" : today ? "preset-tonal-warning" : "preset-tonal-surface"}`}
                    >
                        {scheduleLabel(task.schedule, !!overdue, today)}
                    </span>
                {/if}
                {#if totalSubtasks > 0}
                    <span class="text-[10px] text-surface-500 font-mono">
                        {completedSubtasks}/{totalSubtasks}
                    </span>
                {/if}
            </div>
        </div>

        <div class="flex items-center gap-1 shrink-0 mt-2">
            {#if totalSubtasks > 0}
                <button
                    onclick={() => (expanded = !expanded)}
                    class="btn btn-icon preset-tonal-surface size-4"
                    aria-label="Expand subtasks"
                >
                    {#if expanded}
                        <ChevronUp class="size-4" />
                    {:else}
                        <ChevronDown class="size-4" />
                    {/if}
                </button>
            {/if}

            <Menu onSelect={(value) => menuSelection(value.value)}>
                <Menu.Trigger class="btn">
                    <EllipsisVerticalIcon class="size-4" />
                </Menu.Trigger>
                <Portal>
                    <Menu.Positioner>
                        <Menu.Content>
                            <Menu.Item value="startTimer">
                                <Menu.ItemText>Start timer</Menu.ItemText>
                            </Menu.Item>
                            <Menu.Item value="edit">
                                <Menu.ItemText>Edit task</Menu.ItemText>
                            </Menu.Item>
                            <Menu.Separator />
                            <Menu.Item value="delete">
                                <Menu.ItemText class="text-red-600"
                                    >Delete</Menu.ItemText
                                >
                            </Menu.Item>
                        </Menu.Content>
                    </Menu.Positioner>
                </Portal>
            </Menu>
        </div>
    </div>

    {#if expanded && totalSubtasks > 0}
        <div class="pl-12 pr-4 pb-2 flex flex-col gap-1.5">
            {#each task.subtasks as sub (sub.id)}
                <div class="flex items-center gap-2">
                    <div class={`grid place-items-center shrink-0 `}>
                        <label class="flex items-center space-x-2">
                            <input
                                class="checkbox"
                                type="checkbox"
                                checked={sub.isCompleted}
                                onchange={() =>
                                    $toggleSubtask.mutate({
                                        subtaskId: sub.id,
                                        isCompleted: !sub.isCompleted,
                                    })}
                            />
                            <span
                                class={`text-xs ${sub.isCompleted ? "line-through text-surface-500" : "text-surface-300"}`}
                            >
                                {sub.title}
                            </span>
                        </label>
                    </div>
                </div>
            {/each}
        </div>
    {/if}
</div>
