<script lang="ts">
    import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query";
    import { setupI18n } from "$lib/i18n";
    import { onMount } from "svelte";
    import { authStore } from "$lib/stores/auth";
    import { subscribeToPush } from "$lib/push";
    import {
        requestNotificationPermission,
        isTauri as checkIsTauri,
    } from "$lib/notifications";
    import {
        startReminderPoller,
        stopReminderPoller,
    } from "$lib/reminderPoller";
    import "../app.css";

    const { children } = $props();

    setupI18n();

    const queryClient = new QueryClient({
        defaultOptions: {
            queries: {
                staleTime: 30_000,
                retry: (count: number, error: unknown) => {
                    if (error instanceof Error && error.message.includes("401"))
                        return false;
                    return count < 2;
                },
            },
        },
    });

    const isTauri = checkIsTauri();

    onMount(() => {
        if (!isTauri && "serviceWorker" in navigator && !import.meta.env.DEV) {
            navigator.serviceWorker.register("/sw.js").catch(() => {});
        }

        if (isTauri) {
            requestNotificationPermission().catch(() => {});
        }

        const unsubscribe = authStore.subscribe((state) => {
            if (state.isAuthenticated) {
                if (isTauri) {
                    startReminderPoller();
                } else {
                    subscribeToPush().catch((e) =>
                        console.error("[push] subscribe failed:", e),
                    );
                }
            } else {
                stopReminderPoller();
            }
        });

        return () => {
            unsubscribe();
            stopReminderPoller();
        };
    });
</script>

<QueryClientProvider client={queryClient}>
    {@render children()}
</QueryClientProvider>
