export function isTauri(): boolean {
    return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window
}

export async function requestNotificationPermission(): Promise<boolean> {
    if (isTauri()) {
        const { isPermissionGranted, requestPermission } = await import('@tauri-apps/plugin-notification')
        const granted = await isPermissionGranted()
        if (granted) return true
        const result = await requestPermission()
        return result === 'granted'
    }
    if (!('Notification' in window)) return false
    if (Notification.permission === 'granted') return true
    return (await Notification.requestPermission()) === 'granted'
}

export async function showNotification(title: string, body: string): Promise<void> {
    if (isTauri()) {
        const { isPermissionGranted, sendNotification } = await import('@tauri-apps/plugin-notification')
        if (!(await isPermissionGranted())) return
        sendNotification({ title, body })
        return
    }
    if (Notification.permission === 'granted') {
        new Notification(title, { body, icon: '/icon-192.png' })
    }
}
