<script lang="ts">
    import { goto } from "$app/navigation";
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import { authStore } from "$lib/stores/auth";
    import BottomNav from "@/components/BottomNav.svelte";
    import SideDrawer from "@/components/SideDrawer.svelte";

    const { children } = $props();

    let drawerOpen = $state(false);

    const ROUTE_TITLES: Record<string, string> = {
        "/": "Tasks",
        "/calendar": "Calendar",
        "/timer": "Pomodoro",
        "/stats": "Statistics",
        "/categories": "Categories",
        "/cards": "Flashcards",
        "/settings": "Settings",
    };

    let pathname = $derived($page.url.pathname);
    let title = $derived(ROUTE_TITLES[pathname] ?? "FocusFlow");
    let isTasksSection = $derived(
        !pathname.startsWith("/cards") && !pathname.startsWith("/settings"),
    );
    let onCategories = $derived(pathname === "/categories");

    onMount(() => {
        if (!$authStore.isAuthenticated) {
            goto("/login");
        }
    });
</script>

<div class="h-dvh flex flex-col relative overflow-hidden bg-surface-950">
    {#if drawerOpen}
        <div
            class="absolute inset-0 bg-black/55 backdrop-blur-sm z-30"
            onclick={() => (drawerOpen = false)}
            role="presentation"
        />
    {/if}

    <SideDrawer open={drawerOpen} onClose={() => (drawerOpen = false)} />

    <header
        class="shrink-0 flex items-center gap-3 px-4 pt-2 pb-4 bg-surface-950 relative z-10"
    >
        <button
            class="btn btn-icon preset-tonal-surface size-9"
            onclick={() => (drawerOpen = true)}
            aria-label="Open menu"
        >
            <svg
                viewBox="0 0 16 16"
                width="16"
                height="16"
                stroke="currentColor"
                fill="none"
                stroke-width="1.6"
            >
                <line x1="3" y1="5" x2="13" y2="5" />
                <line x1="3" y1="8" x2="13" y2="8" />
                <line x1="3" y1="11" x2="13" y2="11" />
            </svg>
        </button>

        <h1 class="flex-1 text-2xl font-bold tracking-tight text-surface-50">
            {title}
        </h1>

        {#if isTasksSection}
            <a
                href="/categories"
                class={`btn btn-icon size-9 ${onCategories ? "preset-filled-primary-500" : "preset-tonal-surface"}`}
                title="Categories"
                onclick={(e) => {
                    e.preventDefault();
                    goto("/categories");
                }}
            >
                <svg
                    viewBox="0 0 16 16"
                    width="16"
                    height="16"
                    stroke="currentColor"
                    fill="none"
                    stroke-width="1.6"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <path
                        d="M9 3H3a1 1 0 00-1 1v2a1 1 0 001 1h6l3 3 3-3V4a1 1 0 00-1-1h-2"
                    />
                    <path d="M9 9H3a1 1 0 00-1 1v2a1 1 0 001 1h6" />
                </svg>
            </a>
        {/if}
    </header>

    <div class="flex-1 min-h-0 flex flex-col overflow-hidden">
        {@render children()}
    </div>

    <BottomNav />
</div>
