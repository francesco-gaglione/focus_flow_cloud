<script lang="ts">
    import { createQuery, createMutation } from "@tanstack/svelte-query";
    import { goto } from "$app/navigation";
    import { users as usersApi, auth as authApi, ApiError } from "$lib/api";
    import { authStore } from "$lib/stores/auth";
    import { themeStore, THEMES, type Theme } from "$lib/stores/theme";
    import { serverUrlStore } from "$lib/stores/serverUrl";
    import SettingsSection from "@/components/settings/SettingsSection.svelte";

    const meQuery = createQuery({ queryKey: ["me"], queryFn: usersApi.me });

    let serverUrl = $state(serverUrlStore.get())
    let serverUrlOk = $state(false)

    function saveServerUrl() {
        if (!serverUrl.trim()) return
        serverUrlStore.set(serverUrl.trim())
        serverUrlOk = true
        setTimeout(() => (serverUrlOk = false), 2000)
    }

    let newUsername = $state("");
    let oldPassword = $state("");
    let newPassword = $state("");
    let usernameError = $state<string | null>(null);
    let passwordError = $state<string | null>(null);
    let usernameOk = $state(false);
    let passwordOk = $state(false);

    const updateUsername = createMutation({
        mutationFn: () =>
            usersApi.updateUsername({ new_username: newUsername }),
        onSuccess: () => {
            usernameOk = true;
            usernameError = null;
            newUsername = "";
        },
        onError: (e) => {
            usernameError = e instanceof ApiError ? e.message : "Failed";
            usernameOk = false;
        },
    });

    const updatePassword = createMutation({
        mutationFn: () =>
            usersApi.updatePassword({
                old_password: oldPassword,
                new_password: newPassword,
            }),
        onSuccess: () => {
            passwordOk = true;
            passwordError = null;
            oldPassword = "";
            newPassword = "";
        },
        onError: (e) => {
            passwordError = e instanceof ApiError ? e.message : "Failed";
            passwordOk = false;
        },
    });

    async function handleLogout() {
        await authApi.logout().catch(() => {});
        authStore.logout();
        goto("/login");
    }
</script>

<div class="flex-1 min-h-0 flex flex-col overflow-hidden">
    <div class="flex-1 overflow-y-auto pb-24 px-4 pt-3">
        <SettingsSection title="Server">
            <label class="label mb-3">
                <span class="label-text text-xs font-mono tracking-widest uppercase text-surface-400">
                    Server URL
                </span>
                <input
                    class="input"
                    bind:value={serverUrl}
                    placeholder="https://api.example.com"
                    type="url"
                />
            </label>
            {#if serverUrlOk}
                <aside class="alert preset-tonal-success mb-2">
                    <p class="alert-message text-xs">Server URL updated!</p>
                </aside>
            {/if}
            <button
                onclick={saveServerUrl}
                disabled={!serverUrl.trim()}
                class="btn preset-filled-primary-500 text-sm disabled:opacity-50"
            >
                Save
            </button>
        </SettingsSection>

        <SettingsSection title="Appearance">
            <label class="label">
                <span
                    class="label-text text-xs font-mono tracking-widest uppercase text-surface-400"
                    >Theme</span
                >
                <select
                    class="select capitalize"
                    value={$themeStore}
                    onchange={(e) =>
                        themeStore.setTheme(
                            (e.target as HTMLSelectElement).value as Theme,
                        )}
                >
                    {#each THEMES as t (t)}
                        <option value={t} class="capitalize">{t}</option>
                    {/each}
                </select>
            </label>
        </SettingsSection>

        <SettingsSection title="Account">
            <p class="text-xs text-surface-400 mb-3">
                Signed in as <strong class="text-surface-100 font-medium"
                    >{$meQuery.data?.username ?? "…"}</strong
                >
            </p>
            <label class="label mb-3">
                <span
                    class="label-text text-xs font-mono tracking-widest uppercase text-surface-400"
                    >New username</span
                >
                <input
                    class="input"
                    bind:value={newUsername}
                    placeholder="Enter new username"
                />
            </label>
            {#if usernameError}
                <aside class="alert preset-tonal-error mb-2">
                    <p class="alert-message text-xs">{usernameError}</p>
                </aside>
            {/if}
            {#if usernameOk}
                <aside class="alert preset-tonal-success mb-2">
                    <p class="alert-message text-xs">Username updated!</p>
                </aside>
            {/if}
            <button
                onclick={() => newUsername.trim() && $updateUsername.mutate()}
                disabled={!newUsername.trim()}
                class="btn preset-filled-primary-500 text-sm disabled:opacity-50"
            >
                Update Username
            </button>
        </SettingsSection>

        <SettingsSection title="Change Password">
            <label class="label mb-3">
                <span
                    class="label-text text-xs font-mono tracking-widest uppercase text-surface-400"
                    >Current password</span
                >
                <input
                    class="input"
                    type="password"
                    bind:value={oldPassword}
                    placeholder="••••••••"
                />
            </label>
            <label class="label mb-3">
                <span
                    class="label-text text-xs font-mono tracking-widest uppercase text-surface-400"
                    >New password</span
                >
                <input
                    class="input"
                    type="password"
                    bind:value={newPassword}
                    placeholder="••••••••"
                />
            </label>
            {#if passwordError}
                <aside class="alert preset-tonal-error mb-2">
                    <p class="alert-message text-xs">{passwordError}</p>
                </aside>
            {/if}
            {#if passwordOk}
                <aside class="alert preset-tonal-success mb-2">
                    <p class="alert-message text-xs">Password updated!</p>
                </aside>
            {/if}
            <button
                onclick={() =>
                    oldPassword && newPassword && $updatePassword.mutate()}
                disabled={!oldPassword || !newPassword}
                class="btn preset-filled-primary-500 text-sm disabled:opacity-50"
            >
                Change Password
            </button>
        </SettingsSection>

        <SettingsSection title="Session">
            <button
                onclick={handleLogout}
                class="btn preset-tonal-error w-full text-sm">Sign Out</button
            >
        </SettingsSection>

        <p class="text-center text-[10px] font-mono text-surface-600 mt-2">
            FocusFlow PWA
        </p>
    </div>
</div>
