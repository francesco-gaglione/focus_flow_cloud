<script lang="ts">
    import { CalendarDays } from "lucide-svelte";

    interface Props {
        value: string;
        class?: string;
        onkeydown?: (e: KeyboardEvent) => void;
    }

    let { value = $bindable(), class: cls = "", onkeydown }: Props = $props();

    function formatDisplay(v: string): string {
        if (!v) return "Select date & time";
        const [datePart, timePart] = v.split("T");
        if (!datePart) return "Select date & time";
        const [year, month, day] = datePart.split("-");
        const time = timePart ? timePart.slice(0, 5) : "";
        return `${day}/${month}/${year}${time ? " " + time : ""}`;
    }
</script>

<div class="relative {cls}">
    <div class="input flex items-center gap-2 w-full cursor-pointer pointer-events-none select-none">
        <CalendarDays size={15} class="text-surface-400 shrink-0" />
        <span class={value ? "text-surface-50" : "text-surface-500"}>
            {formatDisplay(value)}
        </span>
    </div>
    <input
        type="datetime-local"
        bind:value
        {onkeydown}
        class="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
        style="font-size: 16px"
    />
</div>
