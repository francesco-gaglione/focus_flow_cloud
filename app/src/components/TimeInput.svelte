<script lang="ts">
    import { Clock } from "lucide-svelte";

    interface Props {
        value: string;
        class?: string;
    }

    let { value = $bindable(), class: cls = "" }: Props = $props();
</script>

<div class="relative {cls}">
    <!-- Visible styled trigger -->
    <div class="input flex items-center gap-2 w-full cursor-pointer pointer-events-none select-none">
        <Clock size={15} class="text-surface-400 shrink-0" />
        <span class={value ? "text-surface-50" : "text-surface-500"}>
            {value || "Select time"}
        </span>
    </div>
    <!-- Native input — invisible, covers full area, captures tap -->
    <input
        type="time"
        bind:value
        class="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
        style="font-size: 16px"
    />
</div>
