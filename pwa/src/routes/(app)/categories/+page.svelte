<script lang="ts">
    import {
        createQuery,
        createMutation,
        useQueryClient,
    } from "@tanstack/svelte-query";
    import CategoryRow from "@/components/categories/CategoryRow.svelte";
    import ColorSwatch, {
        PRESET_COLORS,
    } from "@/components/categories/ColorSwatch.svelte";
    import { categories } from "@/lib/api";

    const qc = useQueryClient();
    let newName = $state("");
    let newColor = $state(PRESET_COLORS[0]);

    const catsQuery = createQuery({
        queryKey: ["categories"],
        queryFn: categories.getAll,
    });
    let cats = $derived($catsQuery.data?.categories ?? []);

    const create = createMutation({
        mutationFn: () =>
            categories.create({ name: newName.trim(), color: newColor }),
        onSuccess: () => {
            qc.invalidateQueries({ queryKey: ["categories"] });
            newName = "";
        },
    });

    const del = createMutation({
        mutationFn: (id: string) => categories.delete(id),
        onSuccess: () => qc.invalidateQueries({ queryKey: ["categories"] }),
    });
</script>

<div class="flex-1 min-h-0 flex flex-col overflow-hidden">
    <div class="flex-1 overflow-y-auto pb-32 px-4 pt-3">
        {#each cats as cat (cat.id)}
            <CategoryRow {cat} onDelete={(id) => $del.mutate(id)} />
        {/each}

        {#if cats.length === 0}
            <div
                class="flex flex-col items-center justify-center py-16 gap-3 text-surface-500"
            >
                <svg
                    viewBox="0 0 20 20"
                    width="28"
                    height="28"
                    stroke="currentColor"
                    fill="none"
                    stroke-width="1.5"
                >
                    <circle cx="10" cy="10" r="3" />
                    <circle cx="10" cy="10" r="8" />
                </svg>
                <p class="text-sm">No categories yet</p>
            </div>
        {/if}

        <div class="card bg-surface-900 border border-surface-700 p-4 mt-4">
            <p
                class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-3"
            >
                New Category
            </p>
            <div class="flex flex-col gap-3">
                <input
                    class="input text-sm"
                    placeholder="Category name"
                    bind:value={newName}
                    onkeydown={(e) => {
                        if (e.key === "Enter" && newName.trim())
                            $create.mutate();
                    }}
                />
                <ColorSwatch
                    colors={PRESET_COLORS}
                    selected={newColor}
                    onSelect={(c) => (newColor = c)}
                />
                <button
                    onclick={() => newName.trim() && $create.mutate()}
                    disabled={!newName.trim()}
                    class="btn preset-filled-primary-500 w-full disabled:opacity-50"
                >
                    Create Category
                </button>
            </div>
        </div>
    </div>
</div>
