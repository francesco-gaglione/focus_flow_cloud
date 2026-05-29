import { I as writable } from "./dev.js";
import "./index-server2.js";
import "./client.js";
import { c as setTokens, o as clearTokens, s as getAccessToken } from "./api.js";
//#region src/lib/stores/auth.ts
function loadInitial() {
	if (typeof window === "undefined") return {
		username: null,
		isAuthenticated: false
	};
	try {
		const raw = localStorage.getItem("ff-auth");
		if (raw) {
			const p = JSON.parse(raw);
			return {
				username: p.username ?? null,
				isAuthenticated: p.isAuthenticated ?? false
			};
		}
	} catch {}
	return {
		username: null,
		isAuthenticated: !!getAccessToken()
	};
}
var _store = writable(loadInitial());
var authStore = {
	subscribe: _store.subscribe,
	login(username, token, refresh) {
		setTokens(token, refresh);
		const state = {
			username,
			isAuthenticated: true
		};
		_store.set(state);
		localStorage.setItem("ff-auth", JSON.stringify(state));
	},
	logout() {
		clearTokens();
		_store.set({
			username: null,
			isAuthenticated: false
		});
		localStorage.removeItem("ff-auth");
	}
};
//#endregion
export { authStore as t };
