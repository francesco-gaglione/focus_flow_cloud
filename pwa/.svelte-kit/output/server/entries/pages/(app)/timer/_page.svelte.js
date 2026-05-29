import "../../../../chunks/index-server.js";
import { I as writable, J as attr, X as escape_html, Y as clsx, c as ensure_array_like, g as unsubscribe_stores, m as store_get, n as attr_class, o as derived } from "../../../../chunks/dev.js";
import "../../../../chunks/index-server2.js";
import { s as getAccessToken } from "../../../../chunks/api.js";
import { n as fmtTime } from "../../../../chunks/utils2.js";
//#region src/lib/ws.ts
var RECONNECT_BASE = 1e3;
var RECONNECT_MAX = 3e4;
function getWsUrl() {
	return "http://192.168.1.135:8080".replace(/^https?:\/\//, (m) => m.startsWith("https") ? "wss://" : "ws://");
}
function createWsStore() {
	const { subscribe, set, update } = writable({
		state: null,
		connected: false,
		error: null
	});
	let ws = null;
	let retryCount = 0;
	let retryTimer;
	let active = false;
	function connect() {
		const token = getAccessToken();
		if (!token || !active) return;
		const url = `${getWsUrl()}/ws/session?token=${encodeURIComponent(token)}`;
		ws = new WebSocket(url);
		ws.onopen = () => {
			if (!active) {
				ws?.close();
				return;
			}
			retryCount = 0;
			update((s) => ({
				...s,
				connected: true,
				error: null
			}));
			ws?.send(JSON.stringify({ requestSync: null }));
		};
		ws.onmessage = (ev) => {
			if (!active) return;
			try {
				const val = JSON.parse(ev.data);
				const stateVal = val.syncData ?? val.pomodoroSessionUpdate;
				if (stateVal) update((s) => ({
					...s,
					state: stateVal
				}));
			} catch {}
		};
		ws.onerror = () => {
			if (!active) return;
			update((s) => ({
				...s,
				error: "WebSocket error"
			}));
		};
		ws.onclose = () => {
			if (!active) return;
			update((s) => ({
				...s,
				connected: false
			}));
			ws = null;
			const delay = Math.min(RECONNECT_BASE * 2 ** retryCount, RECONNECT_MAX);
			retryCount++;
			retryTimer = setTimeout(connect, delay);
		};
	}
	return {
		subscribe,
		start() {
			active = true;
			connect();
		},
		stop() {
			active = false;
			clearTimeout(retryTimer);
			ws?.close();
			ws = null;
			set({
				state: null,
				connected: false,
				error: null
			});
		},
		send(cmd) {
			if (!ws || ws.readyState !== WebSocket.OPEN) return;
			switch (cmd.type) {
				case "start":
					ws.send(JSON.stringify({ startEvent: null }));
					break;
				case "break":
					ws.send(JSON.stringify({ breakEvent: null }));
					break;
				case "terminate":
					ws.send(JSON.stringify({ terminateEvent: null }));
					break;
				case "updateNote":
					ws.send(JSON.stringify({ updateNote: { newNote: cmd.note } }));
					break;
				case "updateScore":
					ws.send(JSON.stringify({ updateConcentrationScore: { concentrationScore: cmd.score } }));
					break;
				case "updateContext":
					ws.send(JSON.stringify({ updatePomodoroContext: { taskId: cmd.taskId } }));
					break;
			}
		}
	};
}
var wsStore = createWsStore();
//#endregion
//#region src/lib/stores/timer.ts
var _store = writable({ selectedTask: null });
var timerStore = {
	subscribe: _store.subscribe,
	setSelectedTask(task) {
		_store.update((s) => ({
			...s,
			selectedTask: task
		}));
	}
};
//#endregion
//#region src/routes/(app)/timer/+page.svelte
function _page($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const CIRCUMFERENCE = 552.92;
		const SESSION_LABELS = {
			Work: "Focus",
			ShortBreak: "Short Break",
			LongBreak: "Long Break"
		};
		const SESSION_TARGETS = {
			Work: 1500,
			ShortBreak: 300,
			LongBreak: 900
		};
		let noteInput = "";
		let session = derived(() => store_get($$store_subs ??= {}, "$wsStore", wsStore).state?.currentSession ?? null);
		let hasSession = derived(() => !!session());
		let isWork = derived(() => session()?.sessionType === "Work");
		let sessionLabel = derived(() => session() ? SESSION_LABELS[session().sessionType] ?? session().sessionType : "Focus");
		let remaining = derived(() => {
			if (!session()) return 1500;
			const now = Math.floor(Date.now() / 1e3);
			const elapsed = Math.max(0, now - session().sessionStartTime);
			const target = SESSION_TARGETS[session().sessionType] ?? 1500;
			return Math.max(0, target - elapsed);
		});
		let progress = derived(() => {
			if (!session()) return 0;
			const target = SESSION_TARGETS[session().sessionType] ?? 1500;
			return 1 - remaining() / target;
		});
		let dashOffset = derived(() => CIRCUMFERENCE * (1 - progress()));
		$$renderer.push(`<div class="flex-1 min-h-0 flex flex-col overflow-hidden"><div class="flex-1 overflow-y-auto pb-32 flex flex-col items-center px-4 pt-4 gap-4"><div class="w-full flex items-center justify-between">`);
		if (store_get($$store_subs ??= {}, "$timerStore", timerStore).selectedTask) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<div class="flex items-center gap-2 bg-primary-500/10 border border-primary-500/30 rounded-full px-3 py-1.5 max-w-[75%]"><div class="size-1.5 rounded-full bg-primary-400 shrink-0"></div> <span class="text-xs text-primary-300 truncate">${escape_html(store_get($$store_subs ??= {}, "$timerStore", timerStore).selectedTask.title)}</span></div>`);
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<div class="flex items-center gap-2 bg-surface-800 border border-surface-700 rounded-full px-3 py-1.5"><span class="text-xs text-surface-500">No task selected</span></div>`);
		}
		$$renderer.push(`<!--]--> <div${attr_class(`size-2 rounded-full ${store_get($$store_subs ??= {}, "$wsStore", wsStore).connected ? "bg-green-400" : "bg-surface-600"}`)}${attr("title", store_get($$store_subs ??= {}, "$wsStore", wsStore).connected ? "Connected" : "Disconnected")}></div></div> <p class="text-xs font-mono text-surface-500 uppercase tracking-widest">${escape_html(sessionLabel())}</p> <div class="relative"><svg viewBox="0 0 200 200" width="220" height="220"><circle cx="100" cy="100" r="88" fill="none" stroke="currentColor" stroke-width="6" class="text-surface-800"></circle><circle cx="100" cy="100" r="88" fill="none" stroke="currentColor" stroke-width="6" stroke-linecap="round"${attr_class(clsx(isWork() ? "text-primary-500" : "text-green-500"))}${attr("stroke-dasharray", CIRCUMFERENCE)}${attr("stroke-dashoffset", dashOffset())} transform="rotate(-90 100 100)" style="transition: stroke-dashoffset 0.5s ease"></circle></svg> <div class="absolute inset-0 flex flex-col items-center justify-center"><span class="text-4xl font-mono font-bold text-surface-50 tabular-nums">${escape_html(fmtTime(remaining()))}</span> <span class="text-xs text-surface-500 mt-1">${escape_html(hasSession() ? "Remaining" : "Ready")}</span></div></div> <div class="flex items-center gap-3">`);
		if (!isWork()) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<button class="btn preset-filled-primary-500 px-6">${escape_html(hasSession() ? "Resume" : "Start")}</button>`);
		} else $$renderer.push("<!--[-1-->");
		$$renderer.push(`<!--]--> `);
		if (isWork()) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<button class="btn preset-tonal-surface px-6">Break</button>`);
		} else $$renderer.push("<!--[-1-->");
		$$renderer.push(`<!--]--> `);
		if (hasSession()) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<button class="btn preset-tonal-error px-6">Stop</button>`);
		} else $$renderer.push("<!--[-1-->");
		$$renderer.push(`<!--]--></div> `);
		if (hasSession() && isWork()) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<div class="w-full card bg-surface-900 border border-surface-700 p-4 flex flex-col gap-4"><div><p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-2">Focus Score</p> <div class="flex gap-2"><!--[-->`);
			const each_array = ensure_array_like([
				1,
				2,
				3,
				4,
				5
			]);
			for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
				let i = each_array[$$index];
				$$renderer.push(`<button${attr_class(`text-xl transition-colors ${(session()?.concentrationScore ?? 0) >= i ? "text-primary-400" : "text-surface-700"}`)}>★</button>`);
			}
			$$renderer.push(`<!--]--></div></div> <div><p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-2">Note</p> <div class="flex items-center gap-2 input rounded-lg px-3 py-2"><input class="flex-1 bg-transparent outline-none text-sm text-surface-100 placeholder:text-surface-500"${attr("placeholder", session()?.note || "Add a note…")}${attr("value", noteInput)}/> <button aria-label="Send note" class="text-surface-500 hover:text-surface-200 transition-colors"><svg viewBox="0 0 16 16" width="14" height="14"><line x1="2" y1="8" x2="14" y2="8" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"></line><polyline points="9 3 14 8 9 13" stroke="currentColor" stroke-width="1.6" fill="none" stroke-linecap="round" stroke-linejoin="round"></polyline></svg></button></div></div></div>`);
		} else $$renderer.push("<!--[-1-->");
		$$renderer.push(`<!--]--> `);
		if (store_get($$store_subs ??= {}, "$wsStore", wsStore).error) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<aside class="alert preset-tonal-error w-full"><p class="alert-message text-sm">Connection error: ${escape_html(store_get($$store_subs ??= {}, "$wsStore", wsStore).error)}</p></aside>`);
		} else $$renderer.push("<!--[-1-->");
		$$renderer.push(`<!--]--></div></div>`);
		if ($$store_subs) unsubscribe_stores($$store_subs);
	});
}
//#endregion
export { _page as default };
