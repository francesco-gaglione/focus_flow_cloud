import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
}

export function fmtTime(secs: number): string {
    const s = Math.max(0, secs);
    return `${String(Math.floor(s / 60)).padStart(2, "0")}:${String(s % 60).padStart(2, "0")}`;
}

export function fmtDuration(secs: number): string {
    if (secs < 60) return `${secs}s`;
    const m = Math.floor(secs / 60);
    if (m < 60) return `${m}m`;
    return `${Math.floor(m / 60)}h ${m % 60}m`;
}

export function tsToDate(ts: number): Date {
    return new Date(ts * 1000);
}

export function isToday(ts: number): boolean {
    const d = tsToDate(ts);
    const n = new Date();
    return (
        d.getFullYear() === n.getFullYear() &&
        d.getMonth() === n.getMonth() &&
        d.getDate() === n.getDate()
    );
}

export function isOverdue(ts: number): boolean {
    return tsToDate(ts) < new Date() && !isToday(ts);
}
