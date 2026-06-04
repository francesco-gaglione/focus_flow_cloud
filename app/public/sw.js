// FocusFlow Service Worker
// Handles Web Push notifications and static asset caching

const CACHE_NAME = "focusflow-v1";
const STATIC_ASSETS = ["/", "/index.html"];

// Install: cache static assets (non-fatal — SW activates even if cache fails)
self.addEventListener("install", (event) => {
    event.waitUntil(
        caches
            .open(CACHE_NAME)
            .then((cache) => cache.addAll(STATIC_ASSETS).catch(() => {}))
            .then(() => self.skipWaiting()),
    );
});

// Activate: clean old caches
self.addEventListener("activate", (event) => {
    event.waitUntil(
        caches
            .keys()
            .then((keys) =>
                Promise.all(
                    keys
                        .filter((k) => k !== CACHE_NAME)
                        .map((k) => caches.delete(k)),
                ),
            ),
    );
    self.clients.claim();
});

// Fetch: cache-first for static assets, network-first for API
self.addEventListener("fetch", (event) => {
    const url = new URL(event.request.url);

    // Network-first for API and WS
    if (url.pathname.startsWith("/api") || url.pathname.startsWith("/ws")) {
        return;
    }

    // Cache-first for same-origin GET requests
    if (event.request.method === "GET" && url.origin === self.location.origin) {
        event.respondWith(
            caches.match(event.request).then((cached) => {
                if (cached) return cached;
                return fetch(event.request).then((res) => {
                    if (res.ok) {
                        const clone = res.clone();
                        caches
                            .open(CACHE_NAME)
                            .then((cache) => cache.put(event.request, clone));
                    }
                    return res;
                });
            }),
        );
    }
});

// Push: display notification
self.addEventListener("push", (event) => {
    console.debug("Push event", event);
    let data = { title: "FocusFlow", body: "TODO body" };
    try {
        data = event.data ? event.data.json() : data;
    } catch {
        /* ignore */
    }

    event.waitUntil(
        self.registration.showNotification(data.title, {
            body: data.body,
            icon: "/icon-192.png",
            badge: "/icon-192.png",
            tag: "focusflow-notification",
            renotify: true,
        }),
    );
});

// Notification click: focus or open the app
self.addEventListener("notificationclick", (event) => {
    event.notification.close();
    event.waitUntil(
        self.clients.matchAll({ type: "window" }).then((clients) => {
            const existing = clients.find((c) =>
                c.url.includes(self.location.origin),
            );
            if (existing) return existing.focus();
            return self.clients.openWindow("/");
        }),
    );
});
