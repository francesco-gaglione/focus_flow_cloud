import { I as writable, J as attr, X as escape_html, c as ensure_array_like, g as unsubscribe_stores, m as store_get } from "../../../../chunks/dev.js";
import "../../../../chunks/index-server2.js";
import { i as createQuery, r as createMutation } from "../../../../chunks/dist.js";
import "../../../../chunks/auth.js";
import { a as ApiError, t as users } from "../../../../chunks/api.js";
//#region src/lib/stores/theme.ts
var THEMES = [
	"catppuccin",
	"cerberus",
	"concord",
	"crimson",
	"fennec",
	"hamlindigo",
	"legacy",
	"mint",
	"modern",
	"mona",
	"nosh",
	"nouveau",
	"pine",
	"reign",
	"rocket",
	"rose",
	"sahara",
	"seafoam",
	"terminus",
	"vintage",
	"vox",
	"wintry"
];
function applyTheme(theme) {
	if (typeof document !== "undefined") document.documentElement.setAttribute("data-theme", theme);
}
function loadInitial() {
	if (typeof window === "undefined") return "nosh";
	try {
		const raw = localStorage.getItem("ff-theme");
		if (raw) {
			const p = JSON.parse(raw);
			if (p?.theme && THEMES.includes(p.theme)) return p.theme;
		}
	} catch {}
	return "nosh";
}
var initial = loadInitial();
applyTheme(initial);
var _store = writable(initial);
var themeStore = {
	subscribe: _store.subscribe,
	setTheme(theme) {
		applyTheme(theme);
		_store.set(theme);
		localStorage.setItem("ff-theme", JSON.stringify({ theme }));
	}
};
//#endregion
//#region src/components/settings/SettingsSection.svelte
function SettingsSection($$renderer, $$props) {
	const { title, children } = $$props;
	$$renderer.push(`<div class="card bg-surface-900 border border-surface-700 p-4 mb-3"><p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-4">${escape_html(title)}</p> `);
	children($$renderer);
	$$renderer.push(`<!----></div>`);
}
//#endregion
//#region src/routes/(app)/settings/+page.svelte
function _page($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const meQuery = createQuery({
			queryKey: ["me"],
			queryFn: users.me
		});
		let newUsername = "";
		let oldPassword = "";
		let newPassword = "";
		let usernameError = null;
		let passwordError = null;
		let usernameOk = false;
		let passwordOk = false;
		createMutation({
			mutationFn: () => users.updateUsername({ newUsername }),
			onSuccess: () => {
				usernameOk = true;
				usernameError = null;
				newUsername = "";
			},
			onError: (e) => {
				usernameError = e instanceof ApiError ? e.message : "Failed";
				usernameOk = false;
			}
		});
		createMutation({
			mutationFn: () => users.updatePassword({
				oldPassword,
				newPassword
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
			}
		});
		$$renderer.push(`<div class="flex-1 min-h-0 flex flex-col overflow-hidden"><div class="flex-1 overflow-y-auto pb-32 px-4 pt-3">`);
		SettingsSection($$renderer, {
			title: "Appearance",
			children: ($$renderer) => {
				$$renderer.push(`<label class="label"><span class="label-text text-xs font-mono tracking-widest uppercase text-surface-400">Theme</span> `);
				$$renderer.select({
					class: "select capitalize",
					value: store_get($$store_subs ??= {}, "$themeStore", themeStore),
					onchange: (e) => themeStore.setTheme(e.target.value)
				}, ($$renderer) => {
					$$renderer.push(`<!--[-->`);
					const each_array = ensure_array_like(THEMES);
					for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
						let t = each_array[$$index];
						$$renderer.option({
							value: t,
							class: "capitalize"
						}, ($$renderer) => {
							$$renderer.push(`${escape_html(t)}`);
						});
					}
					$$renderer.push(`<!--]-->`);
				});
				$$renderer.push(`</label>`);
			},
			$$slots: { default: true }
		});
		$$renderer.push(`<!----> `);
		SettingsSection($$renderer, {
			title: "Account",
			children: ($$renderer) => {
				$$renderer.push(`<p class="text-xs text-surface-400 mb-3">Signed in as <strong class="text-surface-100 font-medium">${escape_html(store_get($$store_subs ??= {}, "$meQuery", meQuery).data?.username ?? "…")}</strong></p> <label class="label mb-3"><span class="label-text text-xs font-mono tracking-widest uppercase text-surface-400">New username</span> <input class="input"${attr("value", newUsername)} placeholder="Enter new username"/></label> `);
				if (usernameError) {
					$$renderer.push("<!--[0-->");
					$$renderer.push(`<aside class="alert preset-tonal-error mb-2"><p class="alert-message text-xs">${escape_html(usernameError)}</p></aside>`);
				} else $$renderer.push("<!--[-1-->");
				$$renderer.push(`<!--]--> `);
				if (usernameOk) {
					$$renderer.push("<!--[0-->");
					$$renderer.push(`<aside class="alert preset-tonal-success mb-2"><p class="alert-message text-xs">Username updated!</p></aside>`);
				} else $$renderer.push("<!--[-1-->");
				$$renderer.push(`<!--]--> <button${attr("disabled", !newUsername.trim(), true)} class="btn preset-filled-primary-500 text-sm disabled:opacity-50">Update Username</button>`);
			},
			$$slots: { default: true }
		});
		$$renderer.push(`<!----> `);
		SettingsSection($$renderer, {
			title: "Change Password",
			children: ($$renderer) => {
				$$renderer.push(`<label class="label mb-3"><span class="label-text text-xs font-mono tracking-widest uppercase text-surface-400">Current password</span> <input class="input" type="password"${attr("value", oldPassword)} placeholder="••••••••"/></label> <label class="label mb-3"><span class="label-text text-xs font-mono tracking-widest uppercase text-surface-400">New password</span> <input class="input" type="password"${attr("value", newPassword)} placeholder="••••••••"/></label> `);
				if (passwordError) {
					$$renderer.push("<!--[0-->");
					$$renderer.push(`<aside class="alert preset-tonal-error mb-2"><p class="alert-message text-xs">${escape_html(passwordError)}</p></aside>`);
				} else $$renderer.push("<!--[-1-->");
				$$renderer.push(`<!--]--> `);
				if (passwordOk) {
					$$renderer.push("<!--[0-->");
					$$renderer.push(`<aside class="alert preset-tonal-success mb-2"><p class="alert-message text-xs">Password updated!</p></aside>`);
				} else $$renderer.push("<!--[-1-->");
				$$renderer.push(`<!--]--> <button${attr("disabled", !oldPassword || !newPassword, true)} class="btn preset-filled-primary-500 text-sm disabled:opacity-50">Change Password</button>`);
			},
			$$slots: { default: true }
		});
		$$renderer.push(`<!----> `);
		SettingsSection($$renderer, {
			title: "Session",
			children: ($$renderer) => {
				$$renderer.push(`<button class="btn preset-tonal-error w-full text-sm">Sign Out</button>`);
			},
			$$slots: { default: true }
		});
		$$renderer.push(`<!----> <p class="text-center text-[10px] font-mono text-surface-600 mt-2">FocusFlow PWA</p></div></div>`);
		if ($$store_subs) unsubscribe_stores($$store_subs);
	});
}
//#endregion
export { _page as default };
