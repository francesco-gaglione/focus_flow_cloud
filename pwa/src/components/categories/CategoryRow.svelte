<script lang="ts">
    import { createMutation, useQueryClient } from '@tanstack/svelte-query'
    import { categories } from '$lib/api'
    import type { CategoryDto } from '@/types'
    import ColorSwatch, { PRESET_COLORS } from './ColorSwatch.svelte'

    const {
        cat,
        onDelete,
    }: { cat: CategoryDto; onDelete: (id: string) => void } = $props()

    const qc = useQueryClient()
    let editing = $state(false)
    let name = $state(cat.name)
    let color = $state(cat.color)

    const update = createMutation({
        mutationFn: () => categories.update(cat.id, { name, color }),
        onSuccess: () => {
            qc.invalidateQueries({ queryKey: ['categories'] })
            editing = false
        },
    })
</script>

{#if editing}
    <div
        class="p-3 mb-2 rounded-r-md border border-surface-700 bg-surface-900 flex flex-col gap-3"
        style="border-left: 4px solid {color}"
    >
        <input
            value={name}
            oninput={(e) => (name = (e.target as HTMLInputElement).value)}
            class="input text-sm"
            placeholder="Category name"
        />
        <ColorSwatch colors={PRESET_COLORS} selected={color} onSelect={(c) => (color = c)} />
        <div class="flex gap-2">
            <button onclick={() => $update.mutate()} class="btn preset-filled-primary-500 text-xs h-8 px-4 flex-1">Save</button>
            <button onclick={() => (editing = false)} class="btn preset-tonal-surface text-xs h-8 px-4">Cancel</button>
        </div>
    </div>
{:else}
    <div
        class="flex items-center gap-3 p-3 mb-2 rounded-r-md border border-surface-700 bg-surface-900"
        style="border-left: 4px solid {cat.color}"
    >
        <div class="size-3 rounded-full shrink-0" style="background: {cat.color}"></div>
        <span class="flex-1 text-sm text-surface-100">{cat.name}</span>
        <button onclick={() => (editing = true)} class="btn btn-icon preset-tonal-surface size-7" aria-label="Edit">
            <svg viewBox="0 0 16 16" width="13" height="13" stroke="currentColor" fill="none" stroke-width="1.5">
                <path d="M11 2l3 3-9 9H2v-3l9-9z" />
            </svg>
        </button>
        <button onclick={() => onDelete(cat.id)} class="btn btn-icon preset-tonal-error size-7" aria-label="Delete">
            <svg viewBox="0 0 16 16" width="13" height="13" stroke="currentColor" fill="none" stroke-width="1.5">
                <polyline points="3 4 13 4" />
                <path d="M5 4V2h6v2" />
                <path d="M4 4l1 10h6l1-10" />
            </svg>
        </button>
    </div>
{/if}
