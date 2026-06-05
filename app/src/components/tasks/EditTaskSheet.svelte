<script lang="ts">
    import { createMutation, useQueryClient } from "@tanstack/svelte-query";
    import type { TaskDto, TaskPriority, TaskScheduleDto } from "@/types";
    import { tasks } from "@/lib/api";
    import DateInput from "@/components/DateInput.svelte";
    import TimeInput from "@/components/TimeInput.svelte";

    const {
        task,
        open,
        onClose,
    }: { task: TaskDto; open: boolean; onClose: () => void } = $props();

    const PRIORITY_COLORS: Record<TaskPriority, string> = {
        low: "#46a758",
        medium: "#d97706",
        high: "#ef4444",
        urgent: "#7c3aed",
    };

    type ScheduleType = "Unscheduled" | "AllDay" | "At" | "Span";

    function tsToDateStr(ts: number): string {
        return new Date(ts * 1000).toISOString().slice(0, 10);
    }

    function tsToTimeStr(ts: number): string {
        const d = new Date(ts * 1000);
        return `${String(d.getHours()).padStart(2, "0")}:${String(d.getMinutes()).padStart(2, "0")}`;
    }

    function initScheduleType(s: TaskScheduleDto): ScheduleType {
        if (s.type === "allDay") return "AllDay";
        if (s.type === "at") return "At";
        if (s.type === "span") return "Span";
        return "Unscheduled";
    }

    const qc = useQueryClient();

    let title = $state(task.title);
    let description = $state(task.description ?? "");
    let priority = $state<TaskPriority | null>(task.priority ?? null);
    let scheduleType = $state<ScheduleType>(initScheduleType(task.schedule));
    let scheduleDate = $state(
        task.schedule.type === "allDay"
            ? tsToDateStr(task.schedule.date)
            : task.schedule.type === "at" || task.schedule.type === "span"
              ? tsToDateStr(task.schedule.startsAt)
              : "",
    );
    let scheduleTime = $state(
        task.schedule.type === "at" || task.schedule.type === "span"
            ? tsToTimeStr(task.schedule.startsAt)
            : "",
    );
    let duration = $state(
        task.schedule.type === "span"
            ? Math.round(task.schedule.duration / 60)
            : 25,
    );

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

    const update = createMutation({
        mutationFn: () =>
            tasks.update(task.id, {
                title: title.trim() || null,
                description: description.trim() || null,
                schedule: buildSchedule(),
                priority,
                completed: null,
            }),
        onSuccess: () => {
            qc.invalidateQueries({ queryKey: ["tasks"] });
            onClose();
        },
    });
</script>

{#if open}
    <div
        class="fixed inset-0 z-30 bg-black/60"
        onclick={onClose}
        role="presentation"
    ></div>
    <div
        class="fixed bottom-0 left-0 right-0 z-40 bg-surface-900 rounded-t-2xl border-t border-surface-700 flex flex-col max-h-[90vh]"
    >
        <div class="flex justify-center pt-3 pb-1 shrink-0">
            <div class="w-10 h-1 rounded-full bg-surface-600"></div>
        </div>

        <div class="flex items-center justify-between px-4 pb-3 shrink-0">
            <p class="text-sm font-semibold text-surface-100">Edit Task</p>
            <button
                onclick={onClose}
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
                    for="edit-title"
                    class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-1 block"
                    >Title *</label
                >
                <input
                    id="edit-title"
                    class="input w-full text-sm"
                    placeholder="Task title"
                    bind:value={title}
                />
            </div>

            <div>
                <label
                    for="edit-desc"
                    class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-1 block"
                    >Description</label
                >
                <textarea
                    id="edit-desc"
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
                            onclick={() => (priority = priority === p ? null : p)}
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
                    <DateInput bind:value={scheduleDate} class="w-full" />
                {/if}
                {#if scheduleType === "At" || scheduleType === "Span"}
                    <div class="flex gap-2">
                        <DateInput bind:value={scheduleDate} class="flex-1" />
                        <TimeInput bind:value={scheduleTime} class="flex-1" />
                    </div>
                {/if}
                {#if scheduleType === "Span"}
                    <div class="flex items-center gap-2 mt-2">
                        <label
                            for="edit-dur"
                            class="text-xs text-surface-500 shrink-0"
                            >Duration (min)</label
                        >
                        <input
                            id="edit-dur"
                            type="number"
                            class="input flex-1 text-sm"
                            min={1}
                            bind:value={duration}
                        />
                    </div>
                {/if}
            </div>
        </div>

        <div class="px-4 py-3 border-t border-surface-800 shrink-0">
            <button
                onclick={() => title.trim() && $update.mutate()}
                disabled={!title.trim() || $update.isPending}
                class="btn preset-filled-primary-500 w-full disabled:opacity-50"
            >
                {$update.isPending ? "Saving…" : "Save Changes"}
            </button>
        </div>
    </div>
{/if}
