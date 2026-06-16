<script lang="ts">
    import { goto } from "$app/navigation";
    import { createQuery, createMutation, useQueryClient } from "@tanstack/svelte-query";
    import { writable, derived } from "svelte/store";
    import { flashcardsApi } from "@/lib/api";
    import type {
        FlashcardDto,
        FolderDto,
        FolderContentsResponseDto,
        RootFolderContentsResponseDto,
    } from "@/types";

    const qc = useQueryClient();

    // ── Navigation state ──────────────────────────────────────────────
    const breadcrumbStore = writable<{ id: string; name: string }[]>([]);
    const currentFolderIdStore = derived(breadcrumbStore, (bc) =>
        bc.length > 0 ? bc[bc.length - 1].id : null,
    );

    // ── Card modal ────────────────────────────────────────────────────
    let showCardModal = $state(false);
    let editingCard = $state<FlashcardDto | null>(null);
    let cardFront = $state("");
    let cardBack = $state("");

    // ── Folder modal ──────────────────────────────────────────────────
    let showFolderModal = $state(false);
    let folderName = $state("");

    // ── Due cards query (for banner count) ───────────────────────────
    const dueQuery = createQuery({
        queryKey: ["flashcards", "due"],
        queryFn: flashcardsApi.getDueFlashcards,
    });
    let dueCount = $derived($dueQuery.data?.flashcards.length ?? 0);

    // ── Stats for today progress ──────────────────────────────────────
    const statsQuery = createQuery({
        queryKey: ["flashcards", "stats"],
        queryFn: flashcardsApi.getGlobalStats,
    });

    // ── 7-day sparkline ───────────────────────────────────────────────
    const activityQuery = createQuery({
        queryKey: ["flashcards", "stats", "activity", 7],
        queryFn: () => flashcardsApi.getActivityHeatmap(7),
    });

    let sparklineMax = $derived(
        Math.max(...($activityQuery.data?.entries.map((e) => e.count) ?? [0]), 1)
    );

    // ── Folder contents query ─────────────────────────────────────────
    type FolderData = FolderContentsResponseDto | RootFolderContentsResponseDto;

    const folderQueryOptions = derived(currentFolderIdStore, ($fid) => ({
        queryKey: ["flashcards", "folder", $fid ?? "root"] as const,
        queryFn: (): Promise<FolderData> =>
            $fid
                ? flashcardsApi.getFolderContents($fid)
                : (flashcardsApi.getRootFolderContents() as Promise<FolderData>),
    }));

    const folderQuery = createQuery(folderQueryOptions);
    let folders = $derived($folderQuery.data?.folders ?? []);
    let cards = $derived($folderQuery.data?.flashcards ?? []);
    let isLoading = $derived($folderQuery.isPending);

    // Initialize breadcrumb from root folder ID
    $effect(() => {
        const data = $folderQuery.data;
        if (data && "folderId" in data && $breadcrumbStore.length === 0) {
            breadcrumbStore.set([
                { id: (data as RootFolderContentsResponseDto).folderId, name: "My Flashcards" },
            ]);
        }
    });

    // ── Mutations ─────────────────────────────────────────────────────
    const createCard = createMutation({
        mutationFn: () =>
            flashcardsApi.createFlashcard({
                front: cardFront.trim(),
                back: cardBack.trim(),
                folderId: $currentFolderIdStore!,
            }),
        onSuccess: () => {
            qc.invalidateQueries({ queryKey: ["flashcards", "folder"] });
            qc.invalidateQueries({ queryKey: ["flashcards", "due"] });
            closeCardModal();
        },
    });

    const updateCard = createMutation({
        mutationFn: () =>
            flashcardsApi.updateFlashcard(editingCard!.id, {
                front: cardFront.trim(),
                back: cardBack.trim(),
            }),
        onSuccess: () => {
            qc.invalidateQueries({ queryKey: ["flashcards", "folder"] });
            closeCardModal();
        },
    });

    const deleteCard = createMutation({
        mutationFn: (id: string) => flashcardsApi.deleteFlashcard(id),
        onSuccess: () => {
            pendingDeleteCardId = null;
            qc.invalidateQueries({ queryKey: ["flashcards", "folder"] });
            qc.invalidateQueries({ queryKey: ["flashcards", "due"] });
        },
    });

    const createFolder = createMutation({
        mutationFn: () =>
            flashcardsApi.createFolder({
                name: folderName.trim(),
                parentId: $currentFolderIdStore!,
            }),
        onSuccess: () => {
            qc.invalidateQueries({ queryKey: ["flashcards", "folder"] });
            closeFolderModal();
        },
    });

    const deleteFolder = createMutation({
        mutationFn: (id: string) => flashcardsApi.deleteFolder(id),
        onSuccess: () => {
            pendingDeleteFolderId = null;
            qc.invalidateQueries({ queryKey: ["flashcards", "folder"] });
        },
    });

    // ── Navigation ────────────────────────────────────────────────────
    function navigateInto(folder: FolderDto) {
        breadcrumbStore.update((bc) => [...bc, { id: folder.id, name: folder.name }]);
    }

    function navigateTo(index: number) {
        breadcrumbStore.update((bc) => bc.slice(0, index + 1));
    }

    // ── Card modal helpers ────────────────────────────────────────────
    function openCreateCard() {
        editingCard = null;
        cardFront = "";
        cardBack = "";
        showCardModal = true;
    }

    function openEditCard(card: FlashcardDto) {
        editingCard = card;
        cardFront = card.front;
        cardBack = card.back;
        showCardModal = true;
    }

    function closeCardModal() {
        showCardModal = false;
        editingCard = null;
        cardFront = "";
        cardBack = "";
    }

    function submitCardModal() {
        if (!cardFront.trim() || !cardBack.trim()) return;
        if (editingCard) $updateCard.mutate();
        else $createCard.mutate();
    }

    // ── Folder modal helpers ──────────────────────────────────────────
    function openFolderModal() {
        folderName = "";
        showFolderModal = true;
    }

    function closeFolderModal() {
        showFolderModal = false;
        folderName = "";
    }

    let pendingDeleteCardId = $state<string | null>(null);
    let pendingDeleteFolderId = $state<string | null>(null);

    function confirmDeleteCard(card: FlashcardDto) {
        pendingDeleteCardId = card.id;
    }

    function confirmDeleteFolder(folder: FolderDto) {
        pendingDeleteFolderId = folder.id;
    }

    let canSubmitCard = $derived(cardFront.trim().length > 0 && cardBack.trim().length > 0);
</script>

<div class="flex-1 min-h-0 flex flex-col overflow-hidden">
    <div class="flex-1 overflow-y-auto pb-24">

        <!-- Due cards banner -->
        {#if dueCount > 0}
            <div class="mx-4 mt-3 mb-1 px-4 py-3 rounded-xl bg-yellow-900/30 border border-yellow-700/40 flex items-center gap-3">
                <div class="size-8 rounded-lg bg-yellow-800/50 grid place-items-center shrink-0">
                    <svg viewBox="0 0 16 16" width="14" height="14" fill="none" stroke="currentColor" stroke-width="1.6" class="text-yellow-400">
                        <path d="M8 3v5l3 3" stroke-linecap="round" />
                        <circle cx="8" cy="8" r="6" />
                    </svg>
                </div>
                <div class="flex-1 min-w-0">
                    <p class="text-sm font-medium text-yellow-300">
                        {dueCount} card{dueCount !== 1 ? "s" : ""} due for review
                    </p>
                    <p class="text-xs text-yellow-500/80">Review now to keep your memory fresh</p>
                </div>
                <button
                    onclick={() => goto("/cards/review")}
                    class="btn preset-filled-primary-500 text-sm shrink-0"
                >
                    Start Review
                </button>
            </div>
        {/if}

        <!-- Today's progress + 7-day sparkline -->
        {#if $statsQuery.data}
            {@const s = $statsQuery.data}
            <div class="mx-4 mt-2 mb-1 grid grid-cols-2 gap-2">
                <!-- Today's progress -->
                <div class="px-3 py-2.5 rounded-xl bg-surface-900 border border-surface-800">
                    <p class="text-[10px] font-mono text-surface-500 uppercase tracking-widest mb-1.5">Today</p>
                    <p class="text-sm font-semibold text-surface-100">
                        {s.reviewedToday} / {s.dueToday > 0 ? s.dueToday : dueCount}
                    </p>
                    <div class="h-1 bg-surface-800 rounded-full mt-1.5 overflow-hidden">
                        {#if s.dueToday > 0}
                            <div
                                class="h-full bg-primary-500 rounded-full"
                                style="width: {Math.min(100, (s.reviewedToday / s.dueToday) * 100).toFixed(0)}%"
                            ></div>
                        {:else}
                            <div class="h-full bg-green-600 rounded-full w-full"></div>
                        {/if}
                    </div>
                </div>

                <!-- 7-day sparkline -->
                <div class="px-3 py-2.5 rounded-xl bg-surface-900 border border-surface-800">
                    <p class="text-[10px] font-mono text-surface-500 uppercase tracking-widest mb-1.5">7-day activity</p>
                    {#if $activityQuery.data}
                        <div class="flex items-end gap-0.5 h-6">
                            {#each $activityQuery.data.entries as entry (entry.date)}
                                <div
                                    class="flex-1 rounded-sm bg-primary-500 opacity-80 min-h-[2px]"
                                    style="height: {Math.max(2, (entry.count / sparklineMax) * 24)}px"
                                    title="{entry.date}: {entry.count}"
                                ></div>
                            {/each}
                        </div>
                    {:else}
                        <div class="h-6 flex items-center">
                            <span class="text-[10px] text-surface-600 font-mono">loading…</span>
                        </div>
                    {/if}
                </div>
            </div>
        {/if}

        <!-- Breadcrumb -->
        <div class="px-4 pt-3 pb-1 flex items-center gap-1 flex-wrap">
            {#each $breadcrumbStore as crumb, i (crumb.id)}
                {#if i < $breadcrumbStore.length - 1}
                    <button
                        onclick={() => navigateTo(i)}
                        class="text-sm text-primary-400 hover:text-primary-300"
                    >
                        {crumb.name}
                    </button>
                    <span class="text-surface-600 text-sm">/</span>
                {:else}
                    <span class="text-sm text-surface-200 font-medium">{crumb.name}</span>
                {/if}
            {/each}
        </div>

        <!-- Action buttons -->
        {#if $currentFolderIdStore}
            <div class="px-4 pt-2 pb-3 flex gap-2">
                <button
                    onclick={openFolderModal}
                    class="btn preset-tonal-surface text-sm flex items-center gap-1.5"
                >
                    <svg viewBox="0 0 16 16" width="14" height="14" stroke="currentColor" fill="none" stroke-width="1.6">
                        <path d="M2 4.5A1.5 1.5 0 013.5 3h3L8 5h4.5A1.5 1.5 0 0114 6.5v6A1.5 1.5 0 0112.5 14h-9A1.5 1.5 0 012 12.5V4.5z" />
                        <line x1="8" y1="8" x2="8" y2="12" stroke-linecap="round" />
                        <line x1="6" y1="10" x2="10" y2="10" stroke-linecap="round" />
                    </svg>
                    New Folder
                </button>
                <button
                    onclick={openCreateCard}
                    class="btn preset-filled-primary-500 text-sm flex items-center gap-1.5"
                >
                    <svg viewBox="0 0 16 16" width="14" height="14" stroke="currentColor" fill="none" stroke-width="1.6">
                        <rect x="2" y="4" width="12" height="9" rx="1.5" />
                        <line x1="8" y1="7" x2="8" y2="10" stroke-linecap="round" />
                        <line x1="6.5" y1="8.5" x2="9.5" y2="8.5" stroke-linecap="round" />
                    </svg>
                    New Card
                </button>
            </div>
        {/if}

        {#if isLoading}
            <div class="flex justify-center py-12">
                <div class="size-6 rounded-full border-2 border-surface-600 border-t-primary-500 animate-spin"></div>
            </div>
        {:else}
            <!-- Folders -->
            {#if folders.length > 0}
                <div class="px-4 mb-1">
                    <p class="text-xs font-mono text-surface-600 uppercase tracking-widest mb-2">Folders</p>
                    <div class="flex flex-col gap-1">
                        {#each folders as folder (folder.id)}
                            <div class="flex items-center gap-2 px-3 py-2.5 rounded-lg bg-surface-900 border border-surface-800 group">
                                <button
                                    onclick={() => navigateInto(folder)}
                                    class="flex items-center gap-2.5 flex-1 min-w-0 text-left"
                                >
                                    <svg viewBox="0 0 16 16" width="15" height="15" fill="currentColor" class="text-surface-500 shrink-0">
                                        <path d="M2 4.5A1.5 1.5 0 013.5 3h3L8 5h4.5A1.5 1.5 0 0114 6.5v6A1.5 1.5 0 0112.5 14h-9A1.5 1.5 0 012 12.5V4.5z" />
                                    </svg>
                                    <span class="text-sm text-surface-200 truncate">{folder.name}</span>
                                </button>
                                {#if pendingDeleteFolderId === folder.id}
                                    <div class="flex items-center gap-1 shrink-0">
                                        <button
                                            onclick={() => (pendingDeleteFolderId = null)}
                                            class="btn text-xs px-2 py-0.5 preset-tonal-surface text-surface-400"
                                        >Cancel</button>
                                        <button
                                            onclick={() => $deleteFolder.mutate(folder.id)}
                                            disabled={$deleteFolder.isPending}
                                            class="btn text-xs px-2 py-0.5 bg-red-900/60 border border-red-700/50 text-red-300 disabled:opacity-50"
                                        >Delete</button>
                                    </div>
                                {:else}
                                    <button
                                        onclick={() => confirmDeleteFolder(folder)}
                                        class="opacity-0 group-hover:opacity-100 btn btn-icon size-6 preset-tonal-surface text-surface-500 hover:text-red-400 transition-opacity"
                                        aria-label="Delete folder"
                                    >
                                        <svg viewBox="0 0 16 16" width="12" height="12" stroke="currentColor" fill="none" stroke-width="1.6">
                                            <path d="M3 4h10M6 4V3h4v1M5 4l.5 9h5l.5-9" stroke-linecap="round" stroke-linejoin="round" />
                                        </svg>
                                    </button>
                                {/if}
                            </div>
                        {/each}
                    </div>
                </div>
            {/if}

            <!-- Flashcards -->
            {#if cards.length > 0}
                <div class="px-4 mt-3">
                    <p class="text-xs font-mono text-surface-600 uppercase tracking-widest mb-2">
                        Cards ({cards.length})
                    </p>
                    <div class="flex flex-col gap-2">
                        {#each cards as card (card.id)}
                            <div class="card bg-surface-900 border border-surface-800 px-4 py-3 rounded-lg">
                                <div class="flex items-start gap-2">
                                    <div class="flex-1 min-w-0">
                                        <p class="text-sm text-surface-200 font-medium truncate">{card.front}</p>
                                        <p class="text-xs text-surface-500 truncate mt-0.5">{card.back}</p>
                                        {#if card.dueDate}
                                            {@const due = new Date(card.dueDate)}
                                            {@const isOverdue = due <= new Date()}
                                            <p class="text-xs mt-1 {isOverdue ? 'text-yellow-500' : 'text-surface-600'}">
                                                {isOverdue ? "Due now" : `Due ${due.toLocaleDateString()}`}
                                            </p>
                                        {:else}
                                            <p class="text-xs text-surface-700 mt-1">Not yet reviewed</p>
                                        {/if}
                                    </div>
                                    <div class="flex gap-1 shrink-0">
                                        {#if pendingDeleteCardId === card.id}
                                            <button
                                                onclick={() => (pendingDeleteCardId = null)}
                                                class="btn text-xs px-2 py-0.5 preset-tonal-surface text-surface-400"
                                            >Cancel</button>
                                            <button
                                                onclick={() => $deleteCard.mutate(card.id)}
                                                disabled={$deleteCard.isPending}
                                                class="btn text-xs px-2 py-0.5 bg-red-900/60 border border-red-700/50 text-red-300 disabled:opacity-50"
                                            >Delete</button>
                                        {:else}
                                            <button
                                                onclick={() => openEditCard(card)}
                                                class="btn btn-icon size-7 preset-tonal-surface text-surface-400 hover:text-surface-200"
                                                aria-label="Edit card"
                                            >
                                                <svg viewBox="0 0 16 16" width="12" height="12" stroke="currentColor" fill="none" stroke-width="1.6">
                                                    <path d="M11 2l3 3-8 8H3v-3L11 2z" stroke-linecap="round" stroke-linejoin="round" />
                                                </svg>
                                            </button>
                                            <button
                                                onclick={() => confirmDeleteCard(card)}
                                                class="btn btn-icon size-7 preset-tonal-surface text-surface-400 hover:text-red-400"
                                                aria-label="Delete card"
                                            >
                                                <svg viewBox="0 0 16 16" width="12" height="12" stroke="currentColor" fill="none" stroke-width="1.6">
                                                    <path d="M3 4h10M6 4V3h4v1M5 4l.5 9h5l.5-9" stroke-linecap="round" stroke-linejoin="round" />
                                                </svg>
                                            </button>
                                        {/if}
                                    </div>
                                </div>
                            </div>
                        {/each}
                    </div>
                </div>
            {:else if folders.length === 0 && $currentFolderIdStore}
                <div class="flex flex-col items-center justify-center py-12 gap-3 text-surface-600 px-4">
                    <svg viewBox="0 0 24 24" width="32" height="32" stroke="currentColor" fill="none" stroke-width="1.5">
                        <rect x="3" y="5" width="18" height="14" rx="2" />
                        <path d="M8 12h8" stroke-linecap="round" />
                    </svg>
                    <p class="text-sm text-center">This folder is empty.<br/>Add cards or subfolders above.</p>
                </div>
            {/if}
        {/if}
    </div>
</div>

<!-- ── Card Modal ──────────────────────────────────────────────────── -->
{#if showCardModal}
    <div class="absolute inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-end justify-center">
        <div class="w-full max-w-lg bg-surface-900 border border-surface-700 rounded-t-2xl p-5">
            <div class="flex items-center justify-between mb-4">
                <p class="text-sm font-semibold text-surface-100">
                    {editingCard ? "Edit Card" : "New Card"}
                </p>
                <button onclick={closeCardModal} class="btn btn-icon size-7 preset-tonal-surface text-surface-400">
                    <svg viewBox="0 0 16 16" width="12" height="12" stroke="currentColor" fill="none" stroke-width="1.8">
                        <line x1="3" y1="3" x2="13" y2="13" stroke-linecap="round" />
                        <line x1="13" y1="3" x2="3" y2="13" stroke-linecap="round" />
                    </svg>
                </button>
            </div>
            <div class="flex flex-col gap-3">
                <div>
                    <label class="text-xs text-surface-500 mb-1 block">Front (Question)</label>
                    <textarea
                        class="input text-sm resize-none w-full"
                        rows="3"
                        placeholder="Enter the question or term..."
                        bind:value={cardFront}
                    ></textarea>
                </div>
                <div>
                    <label class="text-xs text-surface-500 mb-1 block">Back (Answer)</label>
                    <textarea
                        class="input text-sm resize-none w-full"
                        rows="3"
                        placeholder="Enter the answer or definition..."
                        bind:value={cardBack}
                    ></textarea>
                </div>
                <button
                    onclick={submitCardModal}
                    disabled={!canSubmitCard || $createCard.isPending || $updateCard.isPending}
                    class="btn preset-filled-primary-500 w-full disabled:opacity-50"
                >
                    {editingCard ? "Save Changes" : "Create Card"}
                </button>
            </div>
        </div>
    </div>
{/if}

<!-- ── Folder Modal ────────────────────────────────────────────────── -->
{#if showFolderModal}
    <div class="absolute inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-end justify-center">
        <div class="w-full max-w-lg bg-surface-900 border border-surface-700 rounded-t-2xl p-5">
            <div class="flex items-center justify-between mb-4">
                <p class="text-sm font-semibold text-surface-100">New Folder</p>
                <button onclick={closeFolderModal} class="btn btn-icon size-7 preset-tonal-surface text-surface-400">
                    <svg viewBox="0 0 16 16" width="12" height="12" stroke="currentColor" fill="none" stroke-width="1.8">
                        <line x1="3" y1="3" x2="13" y2="13" stroke-linecap="round" />
                        <line x1="13" y1="3" x2="3" y2="13" stroke-linecap="round" />
                    </svg>
                </button>
            </div>
            <div class="flex flex-col gap-3">
                <input
                    class="input text-sm"
                    placeholder="Folder name"
                    bind:value={folderName}
                    onkeydown={(e) => {
                        if (e.key === "Enter" && folderName.trim()) $createFolder.mutate();
                    }}
                />
                <button
                    onclick={() => $createFolder.mutate()}
                    disabled={!folderName.trim() || $createFolder.isPending}
                    class="btn preset-filled-primary-500 w-full disabled:opacity-50"
                >
                    Create Folder
                </button>
            </div>
        </div>
    </div>
{/if}
