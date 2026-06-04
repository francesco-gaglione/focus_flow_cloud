import { pushApi } from "$lib/api";

function urlBase64ToUint8Array(base64String: string): Uint8Array {
    const padding = "=".repeat((4 - (base64String.length % 4)) % 4);
    const base64 = (base64String + padding).replace(/-/g, "+").replace(/_/g, "/");
    const raw = atob(base64);
    return Uint8Array.from([...raw].map((c) => c.charCodeAt(0)));
}

export async function subscribeToPush(): Promise<void> {
    console.log("[push] subscribeToPush called");

    if (!("Notification" in window) || !("serviceWorker" in navigator)) {
        console.warn("[push] Notification or serviceWorker not supported");
        return;
    }

    const permission = await Notification.requestPermission();
    console.log("[push] permission:", permission);
    if (permission !== "granted") return;

    const reg = await navigator.serviceWorker.ready;
    console.log("[push] SW ready:", reg);

    const vapidKey = import.meta.env.VITE_VAPID_PUBLIC_KEY as string;
    console.log("[push] VAPID key present:", !!vapidKey);

    let sub = await reg.pushManager.getSubscription();
    console.log("[push] existing subscription:", sub);

    if (!sub) {
        console.log("[push] subscribing...");
        sub = await reg.pushManager.subscribe({
            userVisibleOnly: true,
            applicationServerKey: urlBase64ToUint8Array(vapidKey),
        });
        console.log("[push] new subscription created:", sub);
    }

    console.log("[push] sending subscription to backend...");
    await pushApi.subscribe(sub.toJSON());
    console.log("[push] subscription saved to backend");
}
