<script lang="ts" module>
    export const PRESET_COLORS = [
        '#12a594', '#46a758', '#ffb224', '#f2056f', '#8e4ec6', '#e5484d',
        '#0070f3', '#d97706', '#7c3aed', '#6b7280',
    ]
</script>

<script lang="ts">
    const {
        colors,
        selected,
        onSelect,
    }: { colors: string[]; selected: string; onSelect: (c: string) => void } = $props()
</script>

<div class="flex gap-1.5 flex-wrap">
    {#each colors as c (c)}
        <button
            onclick={() => onSelect(c)}
            class="size-6 rounded-full transition-all"
            style="background: {c}; outline: {selected === c ? '2px solid white' : '2px solid transparent'}; outline-offset: 1px"
        />
    {/each}
</div>
