import { remindersApi } from "$lib/api/reminders";
import { showNotification } from "$lib/notifications";

const POLL_INTERVAL_MS = 30_000;
const SHOWN_KEY = "ff_shown_reminders";

function getShown(): Set<string> {
    try {
        const raw = localStorage.getItem(SHOWN_KEY);
        return raw ? new Set(JSON.parse(raw) as string[]) : new Set();
    } catch {
        return new Set();
    }
}

function markShown(ids: string[]): void {
    const shown = getShown();
    ids.forEach((id) => shown.add(id));
    // Keep only last 500 to avoid unbounded growth
    const trimmed = [...shown].slice(-500);
    localStorage.setItem(SHOWN_KEY, JSON.stringify(trimmed));
}

async function poll(): Promise<void> {
    try {
        const pending = await remindersApi.getPending();
        const shown = getShown();
        const newReminders = pending.filter((r) => !shown.has(r.id));

        if (newReminders.length === 0) return;

        for (const r of newReminders) {
            await showNotification(r.title, r.description || r.title);
        }

        markShown(newReminders.map((r) => r.id));
    } catch {
        // Silent — network may be unavailable
    }
}

let timer: ReturnType<typeof setInterval> | null = null;

export function startReminderPoller(): void {
    if (timer !== null) return;
    poll();
    timer = setInterval(poll, POLL_INTERVAL_MS);
}

export function stopReminderPoller(): void {
    if (timer !== null) {
        clearInterval(timer);
        timer = null;
    }
}
