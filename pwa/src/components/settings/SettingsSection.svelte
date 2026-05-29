<script lang="ts">
    import type { Snippet } from 'svelte'
    const { title, children }: { title: string; children: Snippet } = $props()
</script>

<div class="card bg-surface-900 border border-surface-700 p-4 mb-3">
    <p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-4">{title}</p>
    {@render children()}
</div>
