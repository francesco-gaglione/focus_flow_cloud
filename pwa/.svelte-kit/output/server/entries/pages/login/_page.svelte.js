import "../../../chunks/index-server.js";
import { J as attr, X as escape_html, g as unsubscribe_stores, m as store_get } from "../../../chunks/dev.js";
import { t as $format } from "../../../chunks/runtime.js";
import "../../../chunks/client.js";
import "../../../chunks/auth.js";
import "../../../chunks/api.js";
//#region src/routes/login/+page.svelte
function _page($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let username = "";
		let password = "";
		let loading = false;
		$$renderer.push(`<div class="h-dvh flex items-center justify-center bg-surface-950 p-5"><div class="card bg-surface-900 border border-surface-700 p-8 w-full max-w-sm space-y-6"><div><h1 class="h3 font-bold tracking-tight text-surface-50">Focus<em class="text-primary-400 not-italic">Flow</em></h1> <p class="text-sm text-surface-400 mt-1">${escape_html(store_get($$store_subs ??= {}, "$_", $format)("login.login_title"))}</p></div> <form class="space-y-4"><label class="label"><span class="label-text text-xs font-mono tracking-widest uppercase text-surface-400">${escape_html(store_get($$store_subs ??= {}, "$_", $format)("login.username"))}</span> <input class="input"${attr("value", username)} placeholder="username" autocomplete="username" required=""/></label> <label class="label"><span class="label-text text-xs font-mono tracking-widest uppercase text-surface-400">${escape_html(store_get($$store_subs ??= {}, "$_", $format)("login.password"))}</span> <input class="input" type="password"${attr("value", password)} placeholder="••••••••" autocomplete="current-password" required=""/></label> `);
		$$renderer.push("<!--[-1-->");
		$$renderer.push(`<!--]--> <button type="submit"${attr("disabled", loading, true)} class="btn preset-filled-primary-500 w-full">${escape_html(store_get($$store_subs ??= {}, "$_", $format)("login.sign_in"))}</button></form></div></div>`);
		if ($$store_subs) unsubscribe_stores($$store_subs);
	});
}
//#endregion
export { _page as default };
