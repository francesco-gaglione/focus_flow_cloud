<script lang="ts">
    import { goto } from "$app/navigation";
    import { createQuery, createMutation, useQueryClient } from "@tanstack/svelte-query";
    import { flashcardsApi } from "@/lib/api";
    import type { FlashcardDto, CardRating } from "@/types";

    const qc = useQueryClient();

    // ── Load due cards ────────────────────────────────────────────────
    const dueQuery = createQuery({
        queryKey: ["flashcards", "due"],
        queryFn: flashcardsApi.getDueFlashcards,
    });

    // Build the review queue once data arrives
    let queue = $state<FlashcardDto[]>([]);

    // ── Review progress ───────────────────────────────────────────────
    let index = $state(0);
    let showBack = $state(false);
    let sessionStart = $state(Date.now());
    let done = $state(false);

    // Populate queue when fresh data arrives (handles stale-cache race)
    $effect(() => {
        const data = $dueQuery.data;
        if (data && data.flashcards.length > 0 && queue.length === 0 && !done) {
            queue = [...data.flashcards];
        }
    });

    let current = $derived(queue[index] ?? null);
    let progress = $derived(queue.length > 0 ? (index / queue.length) * 100 : 0);
    let remaining = $derived(queue.length - index);

    // ── Review mutation ───────────────────────────────────────────────
    const reviewMutation = createMutation({
        mutationFn: ({ id, rating }: { id: string; rating: CardRating }) => {
            const elapsedMs = Date.now() - sessionStart;
            const elapsedDays = Math.max(0, Math.floor(elapsedMs / 86_400_000));
            return flashcardsApi.reviewFlashcard(id, { rating, elapsedDays });
        },
        onSuccess: () => {
            qc.invalidateQueries({ queryKey: ["flashcards", "due"] });
            qc.invalidateQueries({ queryKey: ["flashcards", "folder"] });
            advance();
        },
    });

    // ── Helpers ───────────────────────────────────────────────────────
    function flip() {
        showBack = true;
    }

    function rate(rating: CardRating) {
        if (!current || $reviewMutation.isPending) return;
        $reviewMutation.mutate({ id: current.id, rating });
    }

    function advance() {
        if (index < queue.length - 1) {
            index += 1;
            showBack = false;
        } else {
            done = true;
        }
    }

    function exit() {
        goto("/cards");
    }
</script>

<div class="flex-1 flex flex-col overflow-hidden">

    {#if $dueQuery.isPending}
        <!-- Loading -->
        <div class="flex-1 flex items-center justify-center">
            <div class="size-6 rounded-full border-2 border-surface-600 border-t-primary-500 animate-spin"></div>
        </div>

    {:else if done || queue.length === 0}
        <!-- All done -->
        <div class="flex-1 flex flex-col items-center justify-center gap-5 px-6 text-center">
            <div class="size-16 rounded-2xl bg-green-900/30 border border-green-700/40 grid place-items-center">
                <svg viewBox="0 0 24 24" width="28" height="28" stroke="currentColor" fill="none" stroke-width="1.5" class="text-green-400">
                    <path d="M5 13l4 4L19 7" stroke-linecap="round" stroke-linejoin="round" />
                </svg>
            </div>
            <div>
                <h2 class="text-lg font-semibold text-surface-100">
                    {queue.length === 0 ? "No cards due!" : "Session complete!"}
                </h2>
                <p class="text-sm text-surface-500 mt-1">
                    {queue.length === 0
                        ? "All your flashcards are up to date."
                        : `You reviewed ${queue.length} card${queue.length !== 1 ? "s" : ""}.`}
                </p>
            </div>
            <button onclick={exit} class="btn preset-filled-primary-500">
                Back to Flashcards
            </button>
        </div>

    {:else}
        <!-- Header -->
        <div class="px-4 pt-3 pb-2 flex items-center gap-3 border-b border-surface-800 shrink-0">
            <div class="flex-1">
                <p class="text-sm font-medium text-surface-200">
                    {index + 1} <span class="text-surface-600">/ {queue.length}</span>
                </p>
                <p class="text-xs text-surface-500">{remaining} remaining</p>
            </div>
            <button
                onclick={exit}
                class="btn preset-tonal-surface text-sm flex items-center gap-1.5 text-surface-400"
            >
                <svg viewBox="0 0 16 16" width="13" height="13" stroke="currentColor" fill="none" stroke-width="1.6">
                    <line x1="3" y1="3" x2="13" y2="13" stroke-linecap="round" />
                    <line x1="13" y1="3" x2="3" y2="13" stroke-linecap="round" />
                </svg>
                Exit
            </button>
        </div>

        <!-- Progress bar -->
        <div class="h-1 bg-surface-800 shrink-0">
            <div
                class="h-full bg-primary-500 transition-all duration-500 ease-out"
                style="width: {progress}%"
            ></div>
        </div>

        <!-- Card -->
        <div class="flex-1 flex flex-col items-center justify-center px-4 py-6 gap-5 overflow-hidden">
            <div class="w-full max-w-md flex flex-col gap-4">

                <!-- Card face -->
                <div class="card bg-surface-900 border border-surface-700 rounded-xl overflow-hidden">
                    <!-- Front -->
                    <div class="p-6">
                        <p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-3">Question</p>
                        <p class="text-surface-100 text-lg leading-relaxed">{current.front}</p>
                    </div>

                    <!-- Back (revealed) -->
                    {#if showBack}
                        <div class="border-t border-surface-700 p-6 bg-surface-800/40">
                            <p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-3">Answer</p>
                            <p class="text-surface-200 leading-relaxed">{current.back}</p>
                        </div>
                    {/if}
                </div>

                <!-- Actions -->
                {#if !showBack}
                    <button
                        onclick={flip}
                        class="btn preset-tonal-surface w-full flex items-center justify-center gap-2"
                    >
                        <svg viewBox="0 0 16 16" width="14" height="14" stroke="currentColor" fill="none" stroke-width="1.6">
                            <path d="M8 3v6l3-3m-3 3L5 6" stroke-linecap="round" stroke-linejoin="round" />
                            <path d="M14 11a6 6 0 01-12 0" stroke-linecap="round" />
                        </svg>
                        Reveal Answer
                    </button>
                {:else}
                    <div class="flex flex-col gap-3">
                        <p class="text-center text-xs text-surface-500">How well did you remember?</p>
                        <div class="grid grid-cols-4 gap-2">
                            <button
                                onclick={() => rate("Again")}
                                disabled={$reviewMutation.isPending}
                                class="flex flex-col items-center gap-1 py-3 px-1 rounded-lg bg-red-950/60 border border-red-800/50 text-red-300 hover:bg-red-900/60 active:scale-95 transition-all disabled:opacity-50"
                            >
                                <span class="text-sm font-semibold">Again</span>
                                <span class="text-xs text-red-500">forgot</span>
                            </button>
                            <button
                                onclick={() => rate("Hard")}
                                disabled={$reviewMutation.isPending}
                                class="flex flex-col items-center gap-1 py-3 px-1 rounded-lg bg-orange-950/60 border border-orange-800/50 text-orange-300 hover:bg-orange-900/60 active:scale-95 transition-all disabled:opacity-50"
                            >
                                <span class="text-sm font-semibold">Hard</span>
                                <span class="text-xs text-orange-500">struggled</span>
                            </button>
                            <button
                                onclick={() => rate("Good")}
                                disabled={$reviewMutation.isPending}
                                class="flex flex-col items-center gap-1 py-3 px-1 rounded-lg bg-blue-950/60 border border-blue-800/50 text-blue-300 hover:bg-blue-900/60 active:scale-95 transition-all disabled:opacity-50"
                            >
                                <span class="text-sm font-semibold">Good</span>
                                <span class="text-xs text-blue-500">recalled</span>
                            </button>
                            <button
                                onclick={() => rate("Easy")}
                                disabled={$reviewMutation.isPending}
                                class="flex flex-col items-center gap-1 py-3 px-1 rounded-lg bg-green-950/60 border border-green-800/50 text-green-300 hover:bg-green-900/60 active:scale-95 transition-all disabled:opacity-50"
                            >
                                <span class="text-sm font-semibold">Easy</span>
                                <span class="text-xs text-green-500">perfect</span>
                            </button>
                        </div>
                    </div>
                {/if}
            </div>
        </div>
    {/if}

</div>
