import { writable } from 'svelte/store'

interface TimerState {
    selectedTask: { id: string; title: string } | null
}

const _store = writable<TimerState>({ selectedTask: null })

export const timerStore = {
    subscribe: _store.subscribe,
    setSelectedTask(task: { id: string; title: string } | null) {
        _store.update((s) => ({ ...s, selectedTask: task }))
    },
}
