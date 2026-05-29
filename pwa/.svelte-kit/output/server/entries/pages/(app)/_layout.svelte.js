import "../../../chunks/index-server.js";
import { X as escape_html, Y as clsx, c as ensure_array_like, d as sanitize_props, f as slot, g as unsubscribe_stores, h as stringify, m as store_get, n as attr_class, o as derived, p as spread_props, r as attr_style, ut as getContext } from "../../../chunks/dev.js";
import "../../../chunks/client.js";
import { t as authStore } from "../../../chunks/auth.js";
import "../../../chunks/api.js";
import { t as Icon } from "../../../chunks/Icon.js";
//#region node_modules/@sveltejs/kit/src/runtime/app/stores.js
/**
* A function that returns all of the contextual stores. On the server, this must be called during component initialization.
* Only use this if you need to defer store subscription until after the component has mounted, for some reason.
*
* @deprecated Use `$app/state` instead (requires Svelte 5, [see docs for more info](https://svelte.dev/docs/kit/migrating-to-sveltekit-2#SvelteKit-2.12:-$app-stores-deprecated))
*/
var getStores = () => {
	const stores$1 = getContext("__svelte__");
	return {
		/** @type {typeof page} */
		page: { subscribe: stores$1.page.subscribe },
		/** @type {typeof navigating} */
		navigating: { subscribe: stores$1.navigating.subscribe },
		/** @type {typeof updated} */
		updated: stores$1.updated
	};
};
/**
* A readable store whose value contains page data.
*
* On the server, this store can only be subscribed to during component initialization. In the browser, it can be subscribed to at any time.
*
* @deprecated Use `page` from `$app/state` instead (requires Svelte 5, [see docs for more info](https://svelte.dev/docs/kit/migrating-to-sveltekit-2#SvelteKit-2.12:-$app-stores-deprecated))
* @type {import('svelte/store').Readable<import('@sveltejs/kit').Page>}
*/
var page = { subscribe(fn) {
	return getStores().page.subscribe(fn);
} };
//#endregion
//#region node_modules/lucide-svelte/dist/icons/calendar.svelte
function Calendar($$renderer, $$props) {
	/**
	* @license lucide-svelte v0.511.0 - ISC
	*
	* ISC License
	*
	* Copyright (c) for portions of Lucide are held by Cole Bemis 2013-2022 as part of Feather (MIT). All other copyright (c) for Lucide are held by Lucide Contributors 2022.
	*
	* Permission to use, copy, modify, and/or distribute this software for any
	* purpose with or without fee is hereby granted, provided that the above
	* copyright notice and this permission notice appear in all copies.
	*
	* THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
	* WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
	* MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
	* ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
	* WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
	* ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
	* OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
	*
	*/
	Icon($$renderer, spread_props([
		{ name: "calendar" },
		sanitize_props($$props),
		{
			/**
			* @component @name Calendar
			* @description Lucide SVG icon component, renders SVG Element with children.
			*
			* @preview ![img](data:image/svg+xml;base64,PHN2ZyAgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIgogIHdpZHRoPSIyNCIKICBoZWlnaHQ9IjI0IgogIHZpZXdCb3g9IjAgMCAyNCAyNCIKICBmaWxsPSJub25lIgogIHN0cm9rZT0iIzAwMCIgc3R5bGU9ImJhY2tncm91bmQtY29sb3I6ICNmZmY7IGJvcmRlci1yYWRpdXM6IDJweCIKICBzdHJva2Utd2lkdGg9IjIiCiAgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIgogIHN0cm9rZS1saW5lam9pbj0icm91bmQiCj4KICA8cGF0aCBkPSJNOCAydjQiIC8+CiAgPHBhdGggZD0iTTE2IDJ2NCIgLz4KICA8cmVjdCB3aWR0aD0iMTgiIGhlaWdodD0iMTgiIHg9IjMiIHk9IjQiIHJ4PSIyIiAvPgogIDxwYXRoIGQ9Ik0zIDEwaDE4IiAvPgo8L3N2Zz4K) - https://lucide.dev/icons/calendar
			* @see https://lucide.dev/guide/packages/lucide-svelte - Documentation
			*
			* @param {Object} props - Lucide icons props and any valid SVG attribute
			* @returns {FunctionalComponent} Svelte component
			*
			*/
			iconNode: [
				["path", { "d": "M8 2v4" }],
				["path", { "d": "M16 2v4" }],
				["rect", {
					"width": "18",
					"height": "18",
					"x": "3",
					"y": "4",
					"rx": "2"
				}],
				["path", { "d": "M3 10h18" }]
			],
			children: ($$renderer) => {
				$$renderer.push(`<!--[-->`);
				slot($$renderer, $$props, "default", {}, null);
				$$renderer.push(`<!--]-->`);
			},
			$$slots: { default: true }
		}
	]));
}
//#endregion
//#region node_modules/lucide-svelte/dist/icons/chart-no-axes-column.svelte
function Chart_no_axes_column($$renderer, $$props) {
	/**
	* @license lucide-svelte v0.511.0 - ISC
	*
	* ISC License
	*
	* Copyright (c) for portions of Lucide are held by Cole Bemis 2013-2022 as part of Feather (MIT). All other copyright (c) for Lucide are held by Lucide Contributors 2022.
	*
	* Permission to use, copy, modify, and/or distribute this software for any
	* purpose with or without fee is hereby granted, provided that the above
	* copyright notice and this permission notice appear in all copies.
	*
	* THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
	* WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
	* MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
	* ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
	* WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
	* ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
	* OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
	*
	*/
	Icon($$renderer, spread_props([
		{ name: "chart-no-axes-column" },
		sanitize_props($$props),
		{
			/**
			* @component @name ChartNoAxesColumn
			* @description Lucide SVG icon component, renders SVG Element with children.
			*
			* @preview ![img](data:image/svg+xml;base64,PHN2ZyAgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIgogIHdpZHRoPSIyNCIKICBoZWlnaHQ9IjI0IgogIHZpZXdCb3g9IjAgMCAyNCAyNCIKICBmaWxsPSJub25lIgogIHN0cm9rZT0iIzAwMCIgc3R5bGU9ImJhY2tncm91bmQtY29sb3I6ICNmZmY7IGJvcmRlci1yYWRpdXM6IDJweCIKICBzdHJva2Utd2lkdGg9IjIiCiAgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIgogIHN0cm9rZS1saW5lam9pbj0icm91bmQiCj4KICA8bGluZSB4MT0iMTgiIHgyPSIxOCIgeTE9IjIwIiB5Mj0iMTAiIC8+CiAgPGxpbmUgeDE9IjEyIiB4Mj0iMTIiIHkxPSIyMCIgeTI9IjQiIC8+CiAgPGxpbmUgeDE9IjYiIHgyPSI2IiB5MT0iMjAiIHkyPSIxNCIgLz4KPC9zdmc+Cg==) - https://lucide.dev/icons/chart-no-axes-column
			* @see https://lucide.dev/guide/packages/lucide-svelte - Documentation
			*
			* @param {Object} props - Lucide icons props and any valid SVG attribute
			* @returns {FunctionalComponent} Svelte component
			*
			*/
			iconNode: [
				["line", {
					"x1": "18",
					"x2": "18",
					"y1": "20",
					"y2": "10"
				}],
				["line", {
					"x1": "12",
					"x2": "12",
					"y1": "20",
					"y2": "4"
				}],
				["line", {
					"x1": "6",
					"x2": "6",
					"y1": "20",
					"y2": "14"
				}]
			],
			children: ($$renderer) => {
				$$renderer.push(`<!--[-->`);
				slot($$renderer, $$props, "default", {}, null);
				$$renderer.push(`<!--]-->`);
			},
			$$slots: { default: true }
		}
	]));
}
//#endregion
//#region node_modules/lucide-svelte/dist/icons/layers.svelte
function Layers($$renderer, $$props) {
	/**
	* @license lucide-svelte v0.511.0 - ISC
	*
	* ISC License
	*
	* Copyright (c) for portions of Lucide are held by Cole Bemis 2013-2022 as part of Feather (MIT). All other copyright (c) for Lucide are held by Lucide Contributors 2022.
	*
	* Permission to use, copy, modify, and/or distribute this software for any
	* purpose with or without fee is hereby granted, provided that the above
	* copyright notice and this permission notice appear in all copies.
	*
	* THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
	* WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
	* MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
	* ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
	* WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
	* ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
	* OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
	*
	*/
	Icon($$renderer, spread_props([
		{ name: "layers" },
		sanitize_props($$props),
		{
			/**
			* @component @name Layers
			* @description Lucide SVG icon component, renders SVG Element with children.
			*
			* @preview ![img](data:image/svg+xml;base64,PHN2ZyAgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIgogIHdpZHRoPSIyNCIKICBoZWlnaHQ9IjI0IgogIHZpZXdCb3g9IjAgMCAyNCAyNCIKICBmaWxsPSJub25lIgogIHN0cm9rZT0iIzAwMCIgc3R5bGU9ImJhY2tncm91bmQtY29sb3I6ICNmZmY7IGJvcmRlci1yYWRpdXM6IDJweCIKICBzdHJva2Utd2lkdGg9IjIiCiAgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIgogIHN0cm9rZS1saW5lam9pbj0icm91bmQiCj4KICA8cGF0aCBkPSJNMTIuODMgMi4xOGEyIDIgMCAwIDAtMS42NiAwTDIuNiA2LjA4YTEgMSAwIDAgMCAwIDEuODNsOC41OCAzLjkxYTIgMiAwIDAgMCAxLjY2IDBsOC41OC0zLjlhMSAxIDAgMCAwIDAtMS44M3oiIC8+CiAgPHBhdGggZD0iTTIgMTJhMSAxIDAgMCAwIC41OC45MWw4LjYgMy45MWEyIDIgMCAwIDAgMS42NSAwbDguNTgtMy45QTEgMSAwIDAgMCAyMiAxMiIgLz4KICA8cGF0aCBkPSJNMiAxN2ExIDEgMCAwIDAgLjU4LjkxbDguNiAzLjkxYTIgMiAwIDAgMCAxLjY1IDBsOC41OC0zLjlBMSAxIDAgMCAwIDIyIDE3IiAvPgo8L3N2Zz4K) - https://lucide.dev/icons/layers
			* @see https://lucide.dev/guide/packages/lucide-svelte - Documentation
			*
			* @param {Object} props - Lucide icons props and any valid SVG attribute
			* @returns {FunctionalComponent} Svelte component
			*
			*/
			iconNode: [
				["path", { "d": "M12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.83z" }],
				["path", { "d": "M2 12a1 1 0 0 0 .58.91l8.6 3.91a2 2 0 0 0 1.65 0l8.58-3.9A1 1 0 0 0 22 12" }],
				["path", { "d": "M2 17a1 1 0 0 0 .58.91l8.6 3.91a2 2 0 0 0 1.65 0l8.58-3.9A1 1 0 0 0 22 17" }]
			],
			children: ($$renderer) => {
				$$renderer.push(`<!--[-->`);
				slot($$renderer, $$props, "default", {}, null);
				$$renderer.push(`<!--]-->`);
			},
			$$slots: { default: true }
		}
	]));
}
//#endregion
//#region node_modules/lucide-svelte/dist/icons/list-todo.svelte
function List_todo($$renderer, $$props) {
	/**
	* @license lucide-svelte v0.511.0 - ISC
	*
	* ISC License
	*
	* Copyright (c) for portions of Lucide are held by Cole Bemis 2013-2022 as part of Feather (MIT). All other copyright (c) for Lucide are held by Lucide Contributors 2022.
	*
	* Permission to use, copy, modify, and/or distribute this software for any
	* purpose with or without fee is hereby granted, provided that the above
	* copyright notice and this permission notice appear in all copies.
	*
	* THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
	* WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
	* MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
	* ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
	* WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
	* ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
	* OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
	*
	*/
	Icon($$renderer, spread_props([
		{ name: "list-todo" },
		sanitize_props($$props),
		{
			/**
			* @component @name ListTodo
			* @description Lucide SVG icon component, renders SVG Element with children.
			*
			* @preview ![img](data:image/svg+xml;base64,PHN2ZyAgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIgogIHdpZHRoPSIyNCIKICBoZWlnaHQ9IjI0IgogIHZpZXdCb3g9IjAgMCAyNCAyNCIKICBmaWxsPSJub25lIgogIHN0cm9rZT0iIzAwMCIgc3R5bGU9ImJhY2tncm91bmQtY29sb3I6ICNmZmY7IGJvcmRlci1yYWRpdXM6IDJweCIKICBzdHJva2Utd2lkdGg9IjIiCiAgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIgogIHN0cm9rZS1saW5lam9pbj0icm91bmQiCj4KICA8cmVjdCB4PSIzIiB5PSI1IiB3aWR0aD0iNiIgaGVpZ2h0PSI2IiByeD0iMSIgLz4KICA8cGF0aCBkPSJtMyAxNyAyIDIgNC00IiAvPgogIDxwYXRoIGQ9Ik0xMyA2aDgiIC8+CiAgPHBhdGggZD0iTTEzIDEyaDgiIC8+CiAgPHBhdGggZD0iTTEzIDE4aDgiIC8+Cjwvc3ZnPgo=) - https://lucide.dev/icons/list-todo
			* @see https://lucide.dev/guide/packages/lucide-svelte - Documentation
			*
			* @param {Object} props - Lucide icons props and any valid SVG attribute
			* @returns {FunctionalComponent} Svelte component
			*
			*/
			iconNode: [
				["rect", {
					"x": "3",
					"y": "5",
					"width": "6",
					"height": "6",
					"rx": "1"
				}],
				["path", { "d": "m3 17 2 2 4-4" }],
				["path", { "d": "M13 6h8" }],
				["path", { "d": "M13 12h8" }],
				["path", { "d": "M13 18h8" }]
			],
			children: ($$renderer) => {
				$$renderer.push(`<!--[-->`);
				slot($$renderer, $$props, "default", {}, null);
				$$renderer.push(`<!--]-->`);
			},
			$$slots: { default: true }
		}
	]));
}
//#endregion
//#region node_modules/lucide-svelte/dist/icons/settings.svelte
function Settings($$renderer, $$props) {
	/**
	* @license lucide-svelte v0.511.0 - ISC
	*
	* ISC License
	*
	* Copyright (c) for portions of Lucide are held by Cole Bemis 2013-2022 as part of Feather (MIT). All other copyright (c) for Lucide are held by Lucide Contributors 2022.
	*
	* Permission to use, copy, modify, and/or distribute this software for any
	* purpose with or without fee is hereby granted, provided that the above
	* copyright notice and this permission notice appear in all copies.
	*
	* THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
	* WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
	* MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
	* ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
	* WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
	* ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
	* OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
	*
	*/
	Icon($$renderer, spread_props([
		{ name: "settings" },
		sanitize_props($$props),
		{
			/**
			* @component @name Settings
			* @description Lucide SVG icon component, renders SVG Element with children.
			*
			* @preview ![img](data:image/svg+xml;base64,PHN2ZyAgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIgogIHdpZHRoPSIyNCIKICBoZWlnaHQ9IjI0IgogIHZpZXdCb3g9IjAgMCAyNCAyNCIKICBmaWxsPSJub25lIgogIHN0cm9rZT0iIzAwMCIgc3R5bGU9ImJhY2tncm91bmQtY29sb3I6ICNmZmY7IGJvcmRlci1yYWRpdXM6IDJweCIKICBzdHJva2Utd2lkdGg9IjIiCiAgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIgogIHN0cm9rZS1saW5lam9pbj0icm91bmQiCj4KICA8cGF0aCBkPSJNMTIuMjIgMmgtLjQ0YTIgMiAwIDAgMC0yIDJ2LjE4YTIgMiAwIDAgMS0xIDEuNzNsLS40My4yNWEyIDIgMCAwIDEtMiAwbC0uMTUtLjA4YTIgMiAwIDAgMC0yLjczLjczbC0uMjIuMzhhMiAyIDAgMCAwIC43MyAyLjczbC4xNS4xYTIgMiAwIDAgMSAxIDEuNzJ2LjUxYTIgMiAwIDAgMS0xIDEuNzRsLS4xNS4wOWEyIDIgMCAwIDAtLjczIDIuNzNsLjIyLjM4YTIgMiAwIDAgMCAyLjczLjczbC4xNS0uMDhhMiAyIDAgMCAxIDIgMGwuNDMuMjVhMiAyIDAgMCAxIDEgMS43M1YyMGEyIDIgMCAwIDAgMiAyaC40NGEyIDIgMCAwIDAgMi0ydi0uMThhMiAyIDAgMCAxIDEtMS43M2wuNDMtLjI1YTIgMiAwIDAgMSAyIDBsLjE1LjA4YTIgMiAwIDAgMCAyLjczLS43M2wuMjItLjM5YTIgMiAwIDAgMC0uNzMtMi43M2wtLjE1LS4wOGEyIDIgMCAwIDEtMS0xLjc0di0uNWEyIDIgMCAwIDEgMS0xLjc0bC4xNS0uMDlhMiAyIDAgMCAwIC43My0yLjczbC0uMjItLjM4YTIgMiAwIDAgMC0yLjczLS43M2wtLjE1LjA4YTIgMiAwIDAgMS0yIDBsLS40My0uMjVhMiAyIDAgMCAxLTEtMS43M1Y0YTIgMiAwIDAgMC0yLTJ6IiAvPgogIDxjaXJjbGUgY3g9IjEyIiBjeT0iMTIiIHI9IjMiIC8+Cjwvc3ZnPgo=) - https://lucide.dev/icons/settings
			* @see https://lucide.dev/guide/packages/lucide-svelte - Documentation
			*
			* @param {Object} props - Lucide icons props and any valid SVG attribute
			* @returns {FunctionalComponent} Svelte component
			*
			*/
			iconNode: [["path", { "d": "M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" }], ["circle", {
				"cx": "12",
				"cy": "12",
				"r": "3"
			}]],
			children: ($$renderer) => {
				$$renderer.push(`<!--[-->`);
				slot($$renderer, $$props, "default", {}, null);
				$$renderer.push(`<!--]-->`);
			},
			$$slots: { default: true }
		}
	]));
}
//#endregion
//#region node_modules/lucide-svelte/dist/icons/timer.svelte
function Timer($$renderer, $$props) {
	/**
	* @license lucide-svelte v0.511.0 - ISC
	*
	* ISC License
	*
	* Copyright (c) for portions of Lucide are held by Cole Bemis 2013-2022 as part of Feather (MIT). All other copyright (c) for Lucide are held by Lucide Contributors 2022.
	*
	* Permission to use, copy, modify, and/or distribute this software for any
	* purpose with or without fee is hereby granted, provided that the above
	* copyright notice and this permission notice appear in all copies.
	*
	* THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
	* WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
	* MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
	* ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
	* WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
	* ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
	* OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
	*
	*/
	Icon($$renderer, spread_props([
		{ name: "timer" },
		sanitize_props($$props),
		{
			/**
			* @component @name Timer
			* @description Lucide SVG icon component, renders SVG Element with children.
			*
			* @preview ![img](data:image/svg+xml;base64,PHN2ZyAgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIgogIHdpZHRoPSIyNCIKICBoZWlnaHQ9IjI0IgogIHZpZXdCb3g9IjAgMCAyNCAyNCIKICBmaWxsPSJub25lIgogIHN0cm9rZT0iIzAwMCIgc3R5bGU9ImJhY2tncm91bmQtY29sb3I6ICNmZmY7IGJvcmRlci1yYWRpdXM6IDJweCIKICBzdHJva2Utd2lkdGg9IjIiCiAgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIgogIHN0cm9rZS1saW5lam9pbj0icm91bmQiCj4KICA8bGluZSB4MT0iMTAiIHgyPSIxNCIgeTE9IjIiIHkyPSIyIiAvPgogIDxsaW5lIHgxPSIxMiIgeDI9IjE1IiB5MT0iMTQiIHkyPSIxMSIgLz4KICA8Y2lyY2xlIGN4PSIxMiIgY3k9IjE0IiByPSI4IiAvPgo8L3N2Zz4K) - https://lucide.dev/icons/timer
			* @see https://lucide.dev/guide/packages/lucide-svelte - Documentation
			*
			* @param {Object} props - Lucide icons props and any valid SVG attribute
			* @returns {FunctionalComponent} Svelte component
			*
			*/
			iconNode: [
				["line", {
					"x1": "10",
					"x2": "14",
					"y1": "2",
					"y2": "2"
				}],
				["line", {
					"x1": "12",
					"x2": "15",
					"y1": "14",
					"y2": "11"
				}],
				["circle", {
					"cx": "12",
					"cy": "14",
					"r": "8"
				}]
			],
			children: ($$renderer) => {
				$$renderer.push(`<!--[-->`);
				slot($$renderer, $$props, "default", {}, null);
				$$renderer.push(`<!--]-->`);
			},
			$$slots: { default: true }
		}
	]));
}
//#endregion
//#region src/components/BottomNav.svelte
function BottomNav($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const TASKS_TABS = [
			{
				to: "/",
				label: "Tasks",
				Icon: List_todo
			},
			{
				to: "/calendar",
				label: "Calendar",
				Icon: Calendar
			},
			{
				to: "/timer",
				label: "Timer",
				Icon: Timer
			},
			{
				to: "/stats",
				label: "Stats",
				Icon: Chart_no_axes_column
			}
		];
		const FLASHCARDS_TABS = [{
			to: "/cards",
			label: "Cards",
			Icon: Layers
		}];
		const SETTINGS_TABS = [{
			to: "/settings",
			label: "Settings",
			Icon: Settings
		}];
		function getSection(pathname) {
			if (pathname.startsWith("/cards")) return "flashcards";
			if (pathname.startsWith("/settings")) return "settings";
			return "tasks";
		}
		let section = derived(() => getSection(store_get($$store_subs ??= {}, "$page", page).url.pathname));
		let tabs = derived(() => section() === "tasks" ? TASKS_TABS : section() === "flashcards" ? FLASHCARDS_TABS : SETTINGS_TABS);
		function isActive(to) {
			return to === "/" ? store_get($$store_subs ??= {}, "$page", page).url.pathname === "/" : store_get($$store_subs ??= {}, "$page", page).url.pathname.startsWith(to);
		}
		$$renderer.push(`<nav class="fixed bottom-0 left-0 right-0 z-20 border-t border-surface-700 bg-surface-950"><div class="grid"${attr_style(`grid-template-columns: repeat(${stringify(tabs().length)}, minmax(0, 1fr))`)}><!--[-->`);
		const each_array = ensure_array_like(tabs());
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let tab = each_array[$$index];
			const Icon = tab.Icon;
			$$renderer.push(`<button${attr_class(clsx(["flex flex-col items-center justify-center gap-1 py-2 text-[10px] font-medium transition-colors", isActive(tab.to) ? "text-primary-400 bg-primary-500/10" : "text-surface-500 hover:text-surface-300"].join(" ")))}>`);
			if (Icon) {
				$$renderer.push("<!--[-->");
				Icon($$renderer, { size: 20 });
				$$renderer.push("<!--]-->");
			} else {
				$$renderer.push("<!--[!-->");
				$$renderer.push("<!--]-->");
			}
			$$renderer.push(` <span>${escape_html(tab.label)}</span></button>`);
		}
		$$renderer.push(`<!--]--></div></nav>`);
		if ($$store_subs) unsubscribe_stores($$store_subs);
	});
}
//#endregion
//#region src/components/SideDrawer.svelte
function SideDrawer($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const { open, onClose } = $$props;
		function getSection(pathname) {
			if (pathname.startsWith("/cards")) return "flashcards";
			if (pathname.startsWith("/settings")) return "settings";
			return "tasks";
		}
		let active = derived(() => getSection(store_get($$store_subs ??= {}, "$page", page).url.pathname));
		const NAV = [
			{
				id: "tasks",
				to: "/",
				label: "Tasks"
			},
			{
				id: "flashcards",
				to: "/cards",
				label: "Flashcards",
				badge: "Soon"
			},
			{
				id: "settings",
				to: "/settings",
				label: "Settings"
			}
		];
		$$renderer.push(`<aside${attr_class(clsx([
			"absolute top-0 left-0 w-[76%] h-full z-40 flex flex-col overflow-hidden",
			"bg-surface-900 border-r border-surface-700",
			"transition-transform duration-200 ease-out",
			open ? "translate-x-0" : "-translate-x-full"
		].join(" ")))}><div class="pt-12 px-5 pb-4 flex items-center gap-3 border-b border-surface-700"><div class="size-[34px] bg-primary-500/15 border border-primary-500 rounded-md grid place-items-center shrink-0 relative after:content-[''] after:absolute after:size-3 after:bg-primary-500 after:rounded"></div> <span class="text-xl font-bold text-surface-50 tracking-tight">Focus<em class="text-primary-400 not-italic">Flow</em></span> <button class="ml-auto btn btn-icon preset-tonal-surface size-[30px]" aria-label="Close"><svg viewBox="0 0 16 16" width="14" height="14" stroke="currentColor" fill="none" stroke-width="1.6"><line x1="4" y1="4" x2="12" y2="12"></line><line x1="12" y1="4" x2="4" y2="12"></line></svg></button></div> <p class="px-5 pt-4 pb-1.5 font-mono text-[10px] text-surface-500 tracking-widest uppercase">Workspace</p> <nav class="px-3 flex flex-col gap-0.5"><!--[-->`);
		const each_array = ensure_array_like(NAV);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let item = each_array[$$index];
			$$renderer.push(`<button${attr_class(clsx(["flex items-center gap-3 px-2.5 py-2.5 rounded-md text-sm font-medium transition-colors w-full text-left", active() === item.id ? "bg-primary-500/15 text-primary-400" : "text-surface-400 hover:bg-surface-800 hover:text-surface-200"].join(" ")))}><span${attr_class(clsx(["size-8 border rounded-sm grid place-items-center shrink-0 transition-colors", active() === item.id ? "bg-primary-500 border-primary-500 text-white" : "bg-surface-800 border-surface-600 text-surface-400"].join(" ")))}>`);
			if (item.id === "tasks") {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<svg viewBox="0 0 16 16" width="14" height="14" stroke="currentColor" fill="none" stroke-width="1.5" stroke-linecap="round"><path d="M3 4h10M3 8h10M3 12h6"></path></svg>`);
			} else if (item.id === "flashcards") {
				$$renderer.push("<!--[1-->");
				$$renderer.push(`<svg viewBox="0 0 16 16" width="14" height="14" stroke="currentColor" fill="none" stroke-width="1.5"><rect x="2" y="3" width="10" height="9" rx="1"></rect><rect x="4" y="5" width="10" height="9" rx="1"></rect></svg>`);
			} else {
				$$renderer.push("<!--[-1-->");
				$$renderer.push(`<svg viewBox="0 0 16 16" width="14" height="14" stroke="currentColor" fill="none" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="8" cy="8" r="2.5"></circle><path d="M8 1v1.5M8 13.5V15M1 8h1.5M13.5 8H15M3.05 3.05l1.06 1.06M11.89 11.89l1.06 1.06M3.05 12.95l1.06-1.06M11.89 4.11l1.06-1.06"></path></svg>`);
			}
			$$renderer.push(`<!--]--></span> <span class="flex-1">${escape_html(item.label)}</span> `);
			if (item.badge) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<span class="font-mono text-[10px] text-surface-500 tracking-widest uppercase">${escape_html(item.badge)}</span>`);
			} else $$renderer.push("<!--[-1-->");
			$$renderer.push(`<!--]--></button>`);
		}
		$$renderer.push(`<!--]--></nav> <div class="mt-auto px-5 pt-4 pb-8 flex items-center gap-3 border-t border-surface-700"><div class="size-[34px] bg-surface-700 border border-surface-600 rounded-full grid place-items-center font-mono text-xs font-semibold text-surface-300 shrink-0">FF</div> <div class="flex-1 min-w-0"><p class="text-sm font-medium text-surface-100 truncate">${escape_html(store_get($$store_subs ??= {}, "$authStore", authStore).username ?? "User")}</p></div> <button class="btn btn-icon size-8 preset-tonal-surface hover:preset-tonal-error" aria-label="Logout"><svg viewBox="0 0 16 16" width="14" height="14" stroke="currentColor" fill="none" stroke-width="1.5" stroke-linecap="round"><path d="M6 3H3a1 1 0 0 0-1 1v8a1 1 0 0 0 1 1h3"></path><polyline points="10 11 13 8 10 5" stroke-linejoin="round"></polyline><line x1="13" y1="8" x2="6" y2="8"></line></svg></button></div></aside>`);
		if ($$store_subs) unsubscribe_stores($$store_subs);
	});
}
//#endregion
//#region src/routes/(app)/+layout.svelte
function _layout($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const { children } = $$props;
		let drawerOpen = false;
		const ROUTE_TITLES = {
			"/": "Tasks",
			"/calendar": "Calendar",
			"/timer": "Pomodoro",
			"/stats": "Statistics",
			"/categories": "Categories",
			"/cards": "Flashcards",
			"/settings": "Settings"
		};
		let pathname = derived(() => store_get($$store_subs ??= {}, "$page", page).url.pathname);
		let title = derived(() => ROUTE_TITLES[pathname()] ?? "FocusFlow");
		let isTasksSection = derived(() => !pathname().startsWith("/cards") && !pathname().startsWith("/settings"));
		let onCategories = derived(() => pathname() === "/categories");
		$$renderer.push(`<div class="h-dvh flex flex-col relative overflow-hidden bg-surface-950">`);
		if (drawerOpen) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<div class="absolute inset-0 bg-black/55 backdrop-blur-sm z-30" role="presentation"></div>`);
		} else $$renderer.push("<!--[-1-->");
		$$renderer.push(`<!--]--> `);
		SideDrawer($$renderer, {
			open: drawerOpen,
			onClose: () => drawerOpen = false
		});
		$$renderer.push(`<!----> <header class="shrink-0 flex items-center gap-3 px-4 pt-2 pb-4 bg-surface-950 relative z-10"><button class="btn btn-icon preset-tonal-surface size-9" aria-label="Open menu"><svg viewBox="0 0 16 16" width="16" height="16" stroke="currentColor" fill="none" stroke-width="1.6"><line x1="3" y1="5" x2="13" y2="5"></line><line x1="3" y1="8" x2="13" y2="8"></line><line x1="3" y1="11" x2="13" y2="11"></line></svg></button> <h1 class="flex-1 text-2xl font-bold tracking-tight text-surface-50">${escape_html(title())}</h1> `);
		if (isTasksSection()) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<a href="/categories"${attr_class(`btn btn-icon size-9 ${onCategories() ? "preset-filled-primary-500" : "preset-tonal-surface"}`)} title="Categories"><svg viewBox="0 0 16 16" width="16" height="16" stroke="currentColor" fill="none" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"><path d="M9 3H3a1 1 0 00-1 1v2a1 1 0 001 1h6l3 3 3-3V4a1 1 0 00-1-1h-2"></path><path d="M9 9H3a1 1 0 00-1 1v2a1 1 0 001 1h6"></path></svg></a>`);
		} else $$renderer.push("<!--[-1-->");
		$$renderer.push(`<!--]--></header> <div class="flex-1 min-h-0 flex flex-col overflow-hidden">`);
		children($$renderer);
		$$renderer.push(`<!----></div> `);
		BottomNav($$renderer, {});
		$$renderer.push(`<!----></div>`);
		if ($$store_subs) unsubscribe_stores($$store_subs);
	});
}
//#endregion
export { _layout as default };
