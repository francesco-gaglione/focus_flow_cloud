import { i as tick$1, r as settled, t as index_server_exports } from "./index-server.js";
import { f as get_message, h as base64_decode, n as TRAILING_SLASH_PARAM, p as get_status, r as create_remote_key, t as INVALIDATED_PARAM, y as noop } from "./shared.js";
import { u as base } from "./environment.js";
import { S as compact, f as make_trackable, g as add_data_suffix, h as noop_span, l as decode_params, p as normalize_path, s as hash, u as decode_pathname } from "./exports.js";
import { I as writable, ot as noop$1 } from "./dev.js";
import "./index-server2.js";
import "./internal.js";
import { HttpError, Redirect, SvelteKitError } from "@sveltejs/kit/internal";
import "@sveltejs/kit/internal/server";
import * as devalue from "devalue";
var STATES_KEY = "sveltekit:states";
var HISTORY_INDEX = "sveltekit:history";
var NAVIGATION_INDEX = "sveltekit:navigation";
var PRELOAD_PRIORITIES = {
	tap: 1,
	hover: 2,
	viewport: 3,
	eager: 4,
	off: -1,
	false: -1
};
//#endregion
//#region node_modules/@sveltejs/kit/src/runtime/client/utils.js
var origin = "";
/** @param {string | URL} url */
function resolve_url(url) {
	if (url instanceof URL) return url;
	let baseURI = document.baseURI;
	if (!baseURI) {
		const baseTags = document.getElementsByTagName("base");
		baseURI = baseTags.length ? baseTags[0].href : document.URL;
	}
	return new URL(url, baseURI);
}
function scroll_state() {
	return {
		x: pageXOffset,
		y: pageYOffset
	};
}
({ ...PRELOAD_PRIORITIES }), PRELOAD_PRIORITIES.hover;
/** @param {any} value */
function notifiable_store(value) {
	const store = writable(value);
	let ready = true;
	function notify() {
		ready = true;
		store.update((val) => val);
	}
	/** @param {any} new_value */
	function set(new_value) {
		ready = false;
		store.set(new_value);
	}
	/** @param {(value: any) => void} run */
	function subscribe(run) {
		/** @type {any} */
		let old_value;
		return store.subscribe((new_value) => {
			if (old_value === void 0 || ready && new_value !== old_value) run(old_value = new_value);
		});
	}
	return {
		notify,
		set,
		subscribe
	};
}
var updated_listener = { v: noop };
function create_updated_store() {
	const { set, subscribe } = writable(false);
	return {
		subscribe,
		check: async () => false
	};
}
/**
* Is external if
* - origin different
* - path doesn't start with base
* - uses hash router and pathname is more than base
* @param {URL} url
* @param {string} base
* @param {boolean} hash_routing
*/
function is_external_url(url, base, hash_routing) {
	if (url.origin !== origin || !url.pathname.startsWith(base)) return true;
	if (hash_routing) return url.pathname !== location.pathname;
	return false;
}
//#endregion
//#region node_modules/@sveltejs/kit/src/runtime/client/state.svelte.js
var page;
var navigating;
var updated;
var is_legacy = noop$1.toString().includes("$$") || /function \w+\(\) \{\}/.test(noop$1.toString());
var placeholder_url = "a:";
if (is_legacy) {
	page = {
		data: {},
		form: null,
		error: null,
		params: {},
		route: { id: null },
		state: {},
		status: -1,
		url: new URL(placeholder_url)
	};
	navigating = { current: null };
	updated = { current: false };
} else {
	page = new class Page {
		data = {};
		form = null;
		error = null;
		params = {};
		route = { id: null };
		state = {};
		status = -1;
		url = new URL(placeholder_url);
	}();
	navigating = new class Navigating {
		current = null;
	}();
	updated = new class Updated {
		current = false;
	}();
	updated_listener.v = () => updated.current = true;
}
/**
* @param {import('@sveltejs/kit').Page} new_page
*/
function update(new_page) {
	Object.assign(page, new_page);
}
var cache = /* @__PURE__ */ new Map();
/**
* Should be called on the initial run of load functions that hydrate the page.
* Saves any requests with cache-control max-age to the cache.
* @param {URL | string} resource
* @param {RequestInit} [opts]
*/
function initial_fetch(resource, opts) {
	const selector = build_selector(resource, opts);
	const script = document.querySelector(selector);
	if (script?.textContent) {
		script.remove();
		let { body, ...init } = JSON.parse(script.textContent);
		const ttl = script.getAttribute("data-ttl");
		if (ttl) cache.set(selector, {
			body,
			init,
			ttl: 1e3 * Number(ttl)
		});
		if (script.getAttribute("data-b64") !== null) body = base64_decode(body);
		return Promise.resolve(new Response(body, init));
	}
	return window.fetch(resource, opts);
}
/**
* Tries to get the response from the cache, if max-age allows it, else does a fetch.
* @param {URL | string} resource
* @param {string} resolved
* @param {RequestInit} [opts]
*/
function subsequent_fetch(resource, resolved, opts) {
	if (cache.size > 0) {
		const selector = build_selector(resource, opts);
		const cached = cache.get(selector);
		if (cached) {
			if (performance.now() < cached.ttl && [
				"default",
				"force-cache",
				"only-if-cached",
				void 0
			].includes(opts?.cache)) return new Response(cached.body, cached.init);
			cache.delete(selector);
		}
	}
	return window.fetch(resolved, opts);
}
/**
* Build the cache key for a given request
* @param {URL | RequestInfo} resource
* @param {RequestInit} [opts]
*/
function build_selector(resource, opts) {
	let selector = `script[data-sveltekit-fetched][data-url=${JSON.stringify(resource instanceof Request ? resource.url : resource)}]`;
	if (opts?.headers || opts?.body) {
		/** @type {import('types').StrictBody[]} */
		const values = [];
		if (opts.headers) values.push([...new Headers(opts.headers)].join(","));
		if (opts.body && (typeof opts.body === "string" || ArrayBuffer.isView(opts.body))) values.push(opts.body);
		selector += `[data-hash="${hash(...values)}"]`;
	}
	return selector;
}
//#endregion
//#region node_modules/@sveltejs/kit/src/runtime/client/session-storage.js
/**
* Read a value from `sessionStorage`
* @param {string} key
* @param {(value: string) => any} parse
*/
/* @__NO_SIDE_EFFECTS__ */
function get(key, parse = JSON.parse) {
	try {
		return parse(sessionStorage[key]);
	} catch {}
}
//#endregion
//#region node_modules/@sveltejs/kit/src/runtime/client/ndjson.js
/**
* Yields parsed JSON objects from a ReadableStream of newline-delimited JSON.
* Each yielded value is the raw `JSON.parse`'d object — callers handle deserialization.
* @param {ReadableStreamDefaultReader<Uint8Array>} reader
*/
async function* read_ndjson(reader) {
	let done = false;
	let buffer = "";
	const decoder = new TextDecoder();
	while (true) {
		let split = buffer.indexOf("\n");
		while (split !== -1) {
			const line = buffer.slice(0, split).trim();
			buffer = buffer.slice(split + 1);
			if (line) yield JSON.parse(line);
			split = buffer.indexOf("\n");
		}
		if (done) {
			const line = buffer.trim();
			if (line) yield JSON.parse(line);
			return;
		}
		const chunk = await reader.read();
		done = chunk.done;
		if (chunk.value) buffer += decoder.decode(chunk.value, { stream: true });
		if (done) buffer += decoder.decode();
	}
}
//#endregion
//#region node_modules/@sveltejs/kit/src/runtime/client/client.js
/** @import { CacheEntry } from './remote-functions/cache.svelte.js' */
/** @import { Query } from './remote-functions/query/instance.svelte.js' */
/** @import { LiveQuery } from './remote-functions/query-live/instance.svelte.js' */
var { onMount, tick } = index_server_exports;
/**
* Set via transformError, reset and read at the end of navigate.
* Necessary because a navigation might succeed loading but during rendering
* an error occurs, at which point the navigation result needs to be overridden with the error result.
* TODO this is all very hacky, rethink for SvelteKit 3 where we can assume Svelte 5 and do an overhaul of client.js
* @type {{ error: App.Error, status: number } | null}
*/
var rendering_error = null;
/**
* history index -> { x, y }
* @type {Record<number, { x: number; y: number }>}
*/
var scroll_positions = /* @__PURE__ */ get("sveltekit:scroll") ?? {};
/**
* navigation index -> any
* @type {Record<string, any[]>}
*/
var snapshots = /* @__PURE__ */ get("sveltekit:snapshot") ?? {};
var stores = {
	url: /* @__PURE__ */ notifiable_store({}),
	page: /* @__PURE__ */ notifiable_store({}),
	navigating: /* @__PURE__ */ writable(null),
	updated: /* @__PURE__ */ create_updated_store()
};
/** @param {number} index */
function update_scroll_positions(index) {
	scroll_positions[index] = scroll_state();
}
/**
* @param {number} current_history_index
* @param {number} current_navigation_index
*/
function clear_onward_history(current_history_index, current_navigation_index) {
	let i = current_history_index + 1;
	while (scroll_positions[i]) {
		delete scroll_positions[i];
		i += 1;
	}
	i = current_navigation_index + 1;
	while (snapshots[i]) {
		delete snapshots[i];
		i += 1;
	}
}
/**
* Loads `href` the old-fashioned way, with a full page reload.
* Returns a `Promise` that never resolves (to prevent any
* subsequent work, e.g. history manipulation, from happening)
* @param {URL} url
* @param {boolean} [replace] If `true`, will replace the current `history` entry rather than creating a new one with `pushState`
*/
function native_navigation(url, replace = false) {
	if (replace) location.replace(url.href);
	else location.href = url.href;
	return new Promise(noop);
}
/**
* Checks whether a service worker is registered, and if it is,
* tries to update it.
*/
async function update_service_worker() {
	if ("serviceWorker" in navigator) {
		const registration = await navigator.serviceWorker.getRegistration(base || "/");
		if (registration) await registration.update();
	}
}
/** @type {import('types').CSRRoute[]} All routes of the app. Only available when kit.router.resolution=client */
var routes;
/** @type {import('types').CSRPageNodeLoader} */
var default_layout_loader;
/** @type {import('types').CSRPageNodeLoader} */
var default_error_loader;
/** @type {HTMLElement} */
var target;
/** @type {import('./types.js').SvelteKitApp} */
var app;
/** @type {Array<((url: URL) => boolean)>} */
var invalidated = [];
/**
* An array of the `+layout.svelte` and `+page.svelte` component instances
* that currently live on the page — used for capturing and restoring snapshots.
* It's updated/manipulated through `bind:this` in `Root.svelte`.
* @type {import('svelte').SvelteComponent[]}
*/
var components = [];
/** @type {{id: string, token: {}, promise: Promise<import('./types.js').NavigationResult>, fork: Promise<import('svelte').Fork | null> | null} | null} */
var load_cache = null;
function discard_load_cache() {
	load_cache?.fork?.then((f) => f?.discard());
	load_cache = null;
}
/**
* @type {Map<string, Promise<URL>>}
* Cache for client-side rerouting, since it could contain async calls which we want to
* avoid running multiple times which would slow down navigations (e.g. else preloading
* wouldn't help because on navigation it would be called again). Since `reroute` should be
* a pure function (i.e. always return the same) value it's safe to cache across navigations.
* The server reroute calls don't need to be cached because they are called using `import(...)`
* which is cached per the JS spec.
*/
var reroute_cache = /* @__PURE__ */ new Map();
/**
* Note on before_navigate_callbacks, on_navigate_callbacks and after_navigate_callbacks:
* do not re-assign as some closures keep references to these Sets
*/
/** @type {Set<(navigation: import('@sveltejs/kit').BeforeNavigate) => void>} */
var before_navigate_callbacks = /* @__PURE__ */ new Set();
/** @type {Set<(navigation: import('@sveltejs/kit').OnNavigate) => import('types').MaybePromise<(() => void) | void>>} */
var on_navigate_callbacks = /* @__PURE__ */ new Set();
/** @type {Set<(navigation: import('@sveltejs/kit').AfterNavigate) => void>} */
var after_navigate_callbacks = /* @__PURE__ */ new Set();
/** @type {import('./types.js').NavigationState & { nav: import('@sveltejs/kit').NavigationEvent }} */
var current = {
	branch: [],
	error: null,
	url: null,
	nav: null
};
/** this being true means we SSR'd */
var hydrated = false;
var started = false;
var autoscroll = true;
var is_navigating = false;
var force_invalidation = false;
/** @type {import('svelte').SvelteComponent} */
var root;
/** @type {number} keeping track of the history index in order to prevent popstate navigation events if needed */
var current_history_index;
/** @type {number} */
var current_navigation_index;
/** @type {{}} */
var token;
/**
* A set of tokens which are associated to current preloads.
* If a preload becomes a real navigation, it's removed from the set.
* If a preload token is in the set and the preload errors, the error
* handling logic (for example reloading) is skipped.
*/
var preload_tokens = /* @__PURE__ */ new Set();
/**
* @type {Map<string, Map<string, CacheEntry<Query<any>>>>}
* A map of query id -> payload -> query internals for all active queries.
*/
var query_map = /* @__PURE__ */ new Map();
/**
* @type {Map<string, Map<string, CacheEntry<LiveQuery<any>>>>}
* A map of id -> payload -> live query internals for all active queries.
*/
var live_query_map = /* @__PURE__ */ new Map();
function reset_invalidation() {
	invalidated.length = 0;
	force_invalidation = false;
}
/** @param {number} index */
function capture_snapshot(index) {
	if (components.some((c) => c?.snapshot)) snapshots[index] = components.map((c) => c?.snapshot?.capture());
}
/** @param {number} index */
function restore_snapshot(index) {
	snapshots[index]?.forEach((value, i) => {
		components[i]?.snapshot?.restore(value);
	});
}
/**
* @param {string | URL} url
* @param {{ replaceState?: boolean; noScroll?: boolean; keepFocus?: boolean; invalidateAll?: boolean; invalidate?: Array<string | URL | ((url: URL) => boolean)>; state?: Record<string, any> }} options
* @param {number} redirect_count
* @param {{}} [nav_token]
*/
async function _goto(url, options, redirect_count, nav_token) {
	/** @type {Set<string>} */
	let query_keys;
	/** @type {Set<string>} */
	let live_query_keys;
	if (options.invalidateAll) discard_load_cache();
	await navigate({
		type: "goto",
		url: resolve_url(url),
		keepfocus: options.keepFocus,
		noscroll: options.noScroll,
		replace_state: options.replaceState,
		state: options.state,
		redirect_count,
		nav_token,
		accept: () => {
			if (options.invalidateAll) {
				force_invalidation = true;
				query_keys = /* @__PURE__ */ new Set();
				for (const [id, entries] of query_map) for (const payload of entries.keys()) query_keys.add(create_remote_key(id, payload));
				live_query_keys = /* @__PURE__ */ new Set();
				for (const [id, entries] of live_query_map) for (const payload of entries.keys()) live_query_keys.add(create_remote_key(id, payload));
			}
			if (options.invalidate) options.invalidate.forEach(push_invalidated);
		}
	});
	if (options.invalidateAll) tick$1().then(tick$1).then(() => {
		for (const [id, entries] of query_map) for (const [payload, { resource }] of entries) if (query_keys?.has(create_remote_key(id, payload))) resource.refresh();
		for (const [id, entries] of live_query_map) for (const [payload, { resource }] of entries) if (live_query_keys?.has(create_remote_key(id, payload))) resource.reconnect();
	});
}
/**
* @param {import('./types.js').NavigationFinished} result
* @param {HTMLElement} target
* @param {boolean} hydrate
*/
async function initialize(result, target, hydrate) {
	/** @type {import('@sveltejs/kit').NavigationEvent} */
	const nav = {
		params: current.params,
		route: { id: current.route?.id ?? null },
		url: new URL(location.href)
	};
	current = {
		...result.state,
		nav
	};
	update(result.props.page);
	root = new app.root({
		target,
		props: {
			...result.props,
			stores,
			components
		},
		hydrate,
		sync: false,
		transformError: void 0
	});
	await Promise.resolve();
	restore_snapshot(current_navigation_index);
	if (hydrate) {
		/** @type {import('@sveltejs/kit').AfterNavigate} */
		const navigation = {
			from: null,
			to: {
				...nav,
				scroll: scroll_positions[current_history_index] ?? scroll_state()
			},
			willUnload: false,
			type: "enter",
			complete: Promise.resolve()
		};
		after_navigate_callbacks.forEach((fn) => fn(navigation));
	}
	started = true;
}
/**
*
* @param {{
*   url: URL;
*   params: Record<string, string>;
*   branch: Array<import('./types.js').BranchNode | undefined>;
*   errors?: Array<import('types').CSRPageNodeLoader | undefined>;
*   status: number;
*   error: App.Error | null;
*   route: import('types').CSRRoute | null;
*   form?: Record<string, any> | null;
* }} opts
*/
async function get_navigation_result_from_branch({ url, params, branch, errors, status, error, route, form }) {
	/** @type {import('types').TrailingSlash} */
	let slash = "never";
	if (base && (url.pathname === base || url.pathname === base + "/")) slash = "always";
	else for (const node of branch) if (node?.slash !== void 0) slash = node.slash;
	url.pathname = normalize_path(url.pathname, slash);
	url.search = url.search;
	/** @type {import('./types.js').NavigationFinished} */
	const result = {
		type: "loaded",
		state: {
			url,
			params,
			branch,
			error,
			route
		},
		props: {
			constructors: compact(branch).map((branch_node) => branch_node.node.component),
			page: clone_page(page)
		}
	};
	if (form !== void 0) result.props.form = form;
	let data = {};
	let data_changed = !page;
	let p = 0;
	for (let i = 0; i < Math.max(branch.length, current.branch.length); i += 1) {
		const node = branch[i];
		const prev = current.branch[i];
		if (node?.data !== prev?.data) data_changed = true;
		if (!node) continue;
		data = {
			...data,
			...node.data
		};
		if (data_changed) result.props[`data_${p}`] = data;
		p += 1;
	}
	if (!current.url || url.href !== current.url.href || current.error !== error || form !== void 0 && form !== page.form || data_changed) result.props.page = {
		error,
		params,
		route: { id: route?.id ?? null },
		state: {},
		status,
		url: new URL(url),
		form: form ?? null,
		data: data_changed ? data : page.data
	};
	return result;
}
/**
* Call the universal load function of the given node, if it exists.
*
* @param {{
*   loader: import('types').CSRPageNodeLoader;
* 	 parent: () => Promise<Record<string, any>>;
*   url: URL;
*   params: Record<string, string>;
*   route: { id: string | null };
* 	 server_data_node: import('./types.js').DataNode | null;
* }} options
* @returns {Promise<import('./types.js').BranchNode>}
*/
async function load_node({ loader, parent, url, params, route, server_data_node }) {
	/** @type {Record<string, any> | null} */
	let data = null;
	let is_tracking = true;
	/** @type {import('types').Uses} */
	const uses = {
		dependencies: /* @__PURE__ */ new Set(),
		params: /* @__PURE__ */ new Set(),
		parent: false,
		route: false,
		url: false,
		search_params: /* @__PURE__ */ new Set()
	};
	const node = await loader();
	if (node.universal?.load) {
		/** @param {string[]} deps */
		function depends(...deps) {
			for (const dep of deps) {
				const { href } = new URL(dep, url);
				uses.dependencies.add(href);
			}
		}
		/** @type {import('@sveltejs/kit').LoadEvent} */
		const load_input = {
			tracing: {
				enabled: false,
				root: noop_span,
				current: noop_span
			},
			route: new Proxy(route, { get: (target, key) => {
				if (is_tracking) uses.route = true;
				return target[key];
			} }),
			params: new Proxy(params, { get: (target, key) => {
				if (is_tracking) uses.params.add(key);
				return target[key];
			} }),
			data: server_data_node?.data ?? null,
			url: make_trackable(url, () => {
				if (is_tracking) uses.url = true;
			}, (param) => {
				if (is_tracking) uses.search_params.add(param);
			}, app.hash),
			async fetch(resource, init) {
				if (resource instanceof Request) init = {
					body: resource.method === "GET" || resource.method === "HEAD" ? void 0 : await resource.blob(),
					cache: resource.cache,
					credentials: resource.credentials,
					headers: [...resource.headers].length > 0 ? resource?.headers : void 0,
					integrity: resource.integrity,
					keepalive: resource.keepalive,
					method: resource.method,
					mode: resource.mode,
					redirect: resource.redirect,
					referrer: resource.referrer,
					referrerPolicy: resource.referrerPolicy,
					signal: resource.signal,
					...init
				};
				const { resolved, promise } = resolve_fetch_url(resource, init, url);
				if (is_tracking) depends(resolved.href);
				return promise;
			},
			setHeaders: noop,
			depends,
			parent() {
				if (is_tracking) uses.parent = true;
				return parent();
			},
			untrack(fn) {
				is_tracking = false;
				try {
					return fn();
				} finally {
					is_tracking = true;
				}
			}
		};
		data = await node.universal.load.call(null, load_input) ?? null;
	}
	return {
		node,
		loader,
		server: server_data_node,
		universal: node.universal?.load ? {
			type: "data",
			data,
			uses
		} : null,
		data: data ?? server_data_node?.data ?? null,
		slash: node.universal?.trailingSlash ?? server_data_node?.slash
	};
}
/**
* @param {Request | string | URL} input
* @param {RequestInit | undefined} init
* @param {URL} url
*/
function resolve_fetch_url(input, init, url) {
	let requested = input instanceof Request ? input.url : input;
	const resolved = new URL(requested, url);
	if (resolved.origin === url.origin) requested = resolved.href.slice(url.origin.length);
	return {
		resolved,
		promise: started ? subsequent_fetch(requested, resolved.href, init) : initial_fetch(requested, init)
	};
}
/**
* @param {boolean} parent_changed
* @param {boolean} route_changed
* @param {boolean} url_changed
* @param {Set<string>} search_params_changed
* @param {import('types').Uses | undefined} uses
* @param {Record<string, string>} params
*/
function has_changed(parent_changed, route_changed, url_changed, search_params_changed, uses, params) {
	if (force_invalidation) return true;
	if (!uses) return false;
	if (uses.parent && parent_changed) return true;
	if (uses.route && route_changed) return true;
	if (uses.url && url_changed) return true;
	for (const tracked_params of uses.search_params) if (search_params_changed.has(tracked_params)) return true;
	for (const param of uses.params) if (params[param] !== current.params[param]) return true;
	for (const href of uses.dependencies) if (invalidated.some((fn) => fn(new URL(href)))) return true;
	return false;
}
/**
* @param {import('types').ServerDataNode | import('types').ServerDataSkippedNode | null} node
* @param {import('./types.js').DataNode | null} [previous]
* @returns {import('./types.js').DataNode | null}
*/
function create_data_node(node, previous) {
	if (node?.type === "data") return node;
	if (node?.type === "skip") return previous ?? null;
	return null;
}
/**
* @param {URL | null} old_url
* @param {URL} new_url
*/
function diff_search_params(old_url, new_url) {
	if (!old_url) return new Set(new_url.searchParams.keys());
	const changed = new Set([...old_url.searchParams.keys(), ...new_url.searchParams.keys()]);
	for (const key of changed) {
		const old_values = old_url.searchParams.getAll(key);
		const new_values = new_url.searchParams.getAll(key);
		if (old_values.every((value) => new_values.includes(value)) && new_values.every((value) => old_values.includes(value))) changed.delete(key);
	}
	return changed;
}
/**
* @param {Omit<import('./types.js').NavigationFinished['state'], 'branch'> & { error: App.Error }} opts
* @returns {import('./types.js').NavigationFinished}
*/
function preload_error({ error, url, route, params }) {
	return {
		type: "loaded",
		state: {
			error,
			url,
			route,
			params,
			branch: []
		},
		props: {
			page: clone_page(page),
			constructors: []
		}
	};
}
/**
* @param {import('./types.js').NavigationIntent & { preload?: {} }} intent
* @returns {Promise<import('./types.js').NavigationResult>}
*/
async function load_route({ id, invalidating, url, params, route, preload }) {
	if (load_cache?.id === id) {
		preload_tokens.delete(load_cache.token);
		return load_cache.promise;
	}
	const { errors, layouts, leaf } = route;
	const loaders = [...layouts, leaf];
	errors.forEach((loader) => loader?.().catch(noop));
	loaders.forEach((loader) => loader?.[1]().catch(noop));
	/** @type {import('types').ServerNodesResponse | import('types').ServerRedirectNode | null} */
	let server_data = null;
	const url_changed = current.url ? id !== get_page_key(current.url) : false;
	const route_changed = current.route ? route.id !== current.route.id : false;
	const search_params_changed = diff_search_params(current.url, url);
	let parent_invalid = false;
	{
		const invalid_server_nodes = loaders.map((loader, i) => {
			const previous = current.branch[i];
			const invalid = !!loader?.[0] && (previous?.loader !== loader[1] || has_changed(parent_invalid, route_changed, url_changed, search_params_changed, previous.server?.uses, params));
			if (invalid) parent_invalid = true;
			return invalid;
		});
		if (invalid_server_nodes.some(Boolean)) {
			try {
				server_data = await load_data(url, invalid_server_nodes);
			} catch (error) {
				const handled_error = await handle_error(error, {
					url,
					params,
					route: { id }
				});
				if (preload_tokens.has(preload)) return preload_error({
					error: handled_error,
					url,
					params,
					route
				});
				return load_root_error_page({
					status: get_status(error),
					error: handled_error,
					url,
					route
				});
			}
			if (server_data.type === "redirect") return server_data;
		}
	}
	const server_data_nodes = server_data?.nodes;
	let parent_changed = false;
	const branch_promises = loaders.map(async (loader, i) => {
		if (!loader) return;
		/** @type {import('./types.js').BranchNode | undefined} */
		const previous = current.branch[i];
		const server_data_node = server_data_nodes?.[i];
		if ((!server_data_node || server_data_node.type === "skip") && loader[1] === previous?.loader && !has_changed(parent_changed, route_changed, url_changed, search_params_changed, previous.universal?.uses, params)) return previous;
		parent_changed = true;
		if (server_data_node?.type === "error") throw server_data_node;
		return load_node({
			loader: loader[1],
			url,
			params,
			route,
			parent: async () => {
				const data = {};
				for (let j = 0; j < i; j += 1) Object.assign(data, (await branch_promises[j])?.data);
				return data;
			},
			server_data_node: create_data_node(server_data_node === void 0 && loader[0] ? { type: "skip" } : server_data_node ?? null, loader[0] ? previous?.server : void 0)
		});
	});
	for (const p of branch_promises) p.catch(noop);
	/** @type {Array<import('./types.js').BranchNode | undefined>} */
	const branch = [];
	for (let i = 0; i < loaders.length; i += 1) if (loaders[i]) try {
		branch.push(await branch_promises[i]);
	} catch (err) {
		if (err instanceof Redirect) return {
			type: "redirect",
			location: err.location
		};
		if (preload_tokens.has(preload)) return preload_error({
			error: await handle_error(err, {
				params,
				url,
				route: { id: route.id }
			}),
			url,
			params,
			route
		});
		let status = get_status(err);
		/** @type {App.Error} */
		let error;
		if (server_data_nodes?.includes(err)) {
			status = err.status ?? status;
			error = err.error;
		} else if (err instanceof HttpError) error = err.body;
		else {
			if (await stores.updated.check()) {
				await update_service_worker();
				return await native_navigation(url);
			}
			error = await handle_error(err, {
				params,
				url,
				route: { id: route.id }
			});
		}
		const error_load = await load_nearest_error_page(i, branch, errors);
		if (error_load) return get_navigation_result_from_branch({
			url,
			params,
			branch: branch.slice(0, error_load.idx).concat(error_load.node),
			errors,
			status,
			error,
			route
		});
		else return await server_fallback(url, { id: route.id }, error, status);
	}
	else branch.push(void 0);
	return get_navigation_result_from_branch({
		url,
		params,
		branch,
		errors,
		status: 200,
		error: null,
		route,
		form: invalidating ? void 0 : null
	});
}
/**
* @param {number} i Start index to backtrack from
* @param {Array<import('./types.js').BranchNode | undefined>} branch Branch to backtrack
* @param {Array<import('types').CSRPageNodeLoader | undefined>} errors All error pages for this branch
* @returns {Promise<{idx: number; node: import('./types.js').BranchNode} | undefined>}
*/
async function load_nearest_error_page(i, branch, errors) {
	while (i--) if (errors[i]) {
		let j = i;
		while (!branch[j]) j -= 1;
		try {
			return {
				idx: j + 1,
				node: {
					node: await errors[i](),
					loader: errors[i],
					data: {},
					server: null,
					universal: null
				}
			};
		} catch {
			continue;
		}
	}
}
/**
* @param {{
*   status: number;
*   error: App.Error;
*   url: URL;
*   route: { id: string | null }
* }} opts
* @returns {Promise<import('./types.js').NavigationFinished>}
*/
async function load_root_error_page({ status, error, url, route }) {
	/** @type {Record<string, string>} */
	const params = {};
	/** @type {import('types').ServerDataNode | null} */
	let server_data_node = null;
	if (app.server_loads[0] === 0) try {
		const server_data = await load_data(url, [true]);
		if (server_data.type !== "data" || server_data.nodes[0] && server_data.nodes[0].type !== "data") throw 0;
		server_data_node = server_data.nodes[0] ?? null;
	} catch {
		if (url.origin !== origin || url.pathname !== location.pathname || hydrated) await native_navigation(url);
	}
	try {
		return get_navigation_result_from_branch({
			url,
			params,
			branch: [await load_node({
				loader: default_layout_loader,
				url,
				params,
				route,
				parent: () => Promise.resolve({}),
				server_data_node: create_data_node(server_data_node)
			}), {
				node: await default_error_loader(),
				loader: default_error_loader,
				universal: null,
				server: null,
				data: null
			}],
			status,
			error,
			errors: [],
			route: null
		});
	} catch (error) {
		if (error instanceof Redirect) return _goto(new URL(error.location, location.href), {}, 0);
		throw error;
	}
}
/**
* Resolve the relative rerouted URL for a client-side navigation
* @param {URL} url
* @returns {Promise<URL | undefined>}
*/
async function get_rerouted_url(url) {
	const href = url.href;
	if (reroute_cache.has(href)) return reroute_cache.get(href);
	let rerouted;
	try {
		const promise = (async () => {
			let rerouted = await app.hooks.reroute({
				url: new URL(url),
				fetch: async (input, init) => {
					return resolve_fetch_url(input, init, url).promise;
				}
			}) ?? url;
			if (typeof rerouted === "string") {
				const tmp = new URL(url);
				if (app.hash) tmp.hash = rerouted;
				else tmp.pathname = rerouted;
				rerouted = tmp;
			}
			return rerouted;
		})();
		reroute_cache.set(href, promise);
		rerouted = await promise;
	} catch (e) {
		reroute_cache.delete(href);
		return;
	}
	return rerouted;
}
/**
* Resolve the full info (which route, params, etc.) for a client-side navigation from the URL,
* taking the reroute hook into account. If this isn't a client-side-navigation (or the URL is undefined),
* returns undefined.
* @param {URL | undefined} url
* @param {boolean} invalidating
* @returns {Promise<import('./types.js').NavigationIntent | undefined>}
*/
async function get_navigation_intent(url, invalidating) {
	if (!url) return;
	if (is_external_url(url, base, app.hash)) return;
	{
		const rerouted = await get_rerouted_url(url);
		if (!rerouted) return;
		const path = get_url_path(rerouted);
		for (const route of routes) {
			const params = route.exec(path);
			if (params) return {
				id: get_page_key(url),
				invalidating,
				route,
				params: decode_params(params),
				url
			};
		}
	}
}
/** @param {URL} url */
function get_url_path(url) {
	return decode_pathname(app.hash ? url.hash.replace(/^#/, "").replace(/[?#].+/, "") : url.pathname.slice(base.length)) || "/";
}
/** @param {URL} url */
function get_page_key(url) {
	return (app.hash ? url.hash.replace(/^#/, "") : url.pathname) + url.search;
}
/**
* @param {{
*   url: URL;
*   type: import('@sveltejs/kit').Navigation["type"];
*   intent?: import('./types.js').NavigationIntent;
*   delta?: number;
*   event?: PopStateEvent | MouseEvent;
*   scroll?: { x: number, y: number };
* }} opts
*/
function _before_navigate({ url, type, intent, delta, event, scroll }) {
	let should_block = false;
	const nav = create_navigation(current, intent, url, type, scroll ?? null);
	if (delta !== void 0) nav.navigation.delta = delta;
	if (event !== void 0) nav.navigation.event = event;
	const cancellable = {
		...nav.navigation,
		cancel: () => {
			should_block = true;
			nav.reject(/* @__PURE__ */ new Error("navigation cancelled"));
		}
	};
	if (!is_navigating) before_navigate_callbacks.forEach((fn) => fn(cancellable));
	return should_block ? null : nav;
}
/**
* @param {{
*   type: import('@sveltejs/kit').NavigationType;
*   url: URL;
*   popped?: {
*     state: Record<string, any>;
*     scroll: { x: number, y: number };
*     delta: number;
*   };
*   keepfocus?: boolean;
*   noscroll?: boolean;
*   replace_state?: boolean;
*   state?: Record<string, any>;
*   redirect_count?: number;
*   nav_token?: {};
*   accept?: () => void;
*   block?: () => void;
*   event?: Event
* }} opts
*/
async function navigate({ type, url, popped, keepfocus, noscroll, replace_state, state = {}, redirect_count = 0, nav_token = {}, accept = noop, block = noop, event }) {
	const prev_token = token;
	token = nav_token;
	const intent = await get_navigation_intent(url, false);
	const nav = type === "enter" ? create_navigation(current, intent, url, type) : _before_navigate({
		url,
		type,
		delta: popped?.delta,
		intent,
		scroll: popped?.scroll,
		event
	});
	if (!nav) {
		block();
		if (token === nav_token) token = prev_token;
		return;
	}
	const previous_history_index = current_history_index;
	const previous_navigation_index = current_navigation_index;
	accept();
	is_navigating = true;
	if (started && nav.navigation.type !== "enter") stores.navigating.set(navigating.current = nav.navigation);
	let navigation_result = intent && await load_route(intent);
	if (!navigation_result) if (is_external_url(url, base, app.hash)) return await native_navigation(url, replace_state);
	else navigation_result = await server_fallback(url, { id: null }, await handle_error(new SvelteKitError(404, "Not Found", `Not found: ${url.pathname}`), {
		url,
		params: {},
		route: { id: null }
	}), 404, replace_state);
	url = intent?.url || url;
	if (token !== nav_token) {
		nav.reject(/* @__PURE__ */ new Error("navigation aborted"));
		return false;
	}
	if (navigation_result.type === "redirect") {
		if (redirect_count < 20) {
			await navigate({
				type,
				url: new URL(navigation_result.location, url),
				popped,
				keepfocus,
				noscroll,
				replace_state,
				state,
				redirect_count: redirect_count + 1,
				nav_token
			});
			nav.fulfil(void 0);
			return;
		}
		navigation_result = await load_root_error_page({
			status: 500,
			error: await handle_error(/* @__PURE__ */ new Error("Redirect loop"), {
				url,
				params: {},
				route: { id: null }
			}),
			url,
			route: { id: null }
		});
	} else if (navigation_result.props.page.status >= 400) {
		if (await stores.updated.check()) {
			await update_service_worker();
			await native_navigation(url, replace_state);
		}
	}
	reset_invalidation();
	update_scroll_positions(previous_history_index);
	capture_snapshot(previous_navigation_index);
	if (navigation_result.props.page.url.pathname !== url.pathname) url.pathname = navigation_result.props.page.url.pathname;
	state = popped ? popped.state : state;
	if (!popped) {
		const change = replace_state ? 0 : 1;
		const entry = {
			[HISTORY_INDEX]: current_history_index += change,
			[NAVIGATION_INDEX]: current_navigation_index += change,
			[STATES_KEY]: state
		};
		(replace_state ? history.replaceState : history.pushState).call(history, entry, "", url);
		if (!replace_state) clear_onward_history(current_history_index, current_navigation_index);
	}
	const load_cache_fork = intent && load_cache?.id === intent.id ? load_cache.fork : null;
	if (load_cache?.fork && !load_cache_fork) discard_load_cache();
	load_cache = null;
	navigation_result.props.page.state = state;
	/**
	* @type {Promise<void> | undefined}
	*/
	let commit_promise;
	if (started) {
		const after_navigate = (await Promise.all(Array.from(on_navigate_callbacks, (fn) => fn(nav.navigation)))).filter(
			/** @returns {value is () => void} */
			(value) => typeof value === "function"
		);
		if (after_navigate.length > 0) {
			function cleanup() {
				after_navigate.forEach((fn) => {
					after_navigate_callbacks.delete(fn);
				});
			}
			after_navigate.push(cleanup);
			after_navigate.forEach((fn) => {
				after_navigate_callbacks.add(fn);
			});
		}
		const target = nav.navigation.to;
		current = {
			...navigation_result.state,
			nav: {
				params: target.params,
				route: target.route,
				url: target.url
			}
		};
		if (navigation_result.props.page) navigation_result.props.page.url = url;
		const fork = load_cache_fork && await load_cache_fork;
		if (fork) commit_promise = fork.commit();
		else {
			rendering_error = null;
			root.$set(navigation_result.props);
			if (rendering_error) Object.assign(navigation_result.props.page, rendering_error);
			update(navigation_result.props.page);
			commit_promise = settled?.();
		}
	} else await initialize(navigation_result, target, false);
	const { activeElement } = document;
	await commit_promise;
	await tick$1();
	await tick$1();
	if (token !== nav_token) {
		nav.reject(/* @__PURE__ */ new Error("navigation aborted"));
		return false;
	}
	if (navigation_result.props.page && rendering_error) Object.assign(navigation_result.props.page, rendering_error);
	/** @type {Element | null | ''} */
	let deep_linked = null;
	if (autoscroll) {
		const scroll = popped ? popped.scroll : noscroll ? scroll_state() : null;
		if (scroll) scrollTo(scroll.x, scroll.y);
		else if (deep_linked = url.hash && document.getElementById(get_id(url))) deep_linked.scrollIntoView();
		else scrollTo(0, 0);
	}
	const changed_focus = document.activeElement !== activeElement && document.activeElement !== document.body;
	if (!keepfocus && !changed_focus) reset_focus(url, !deep_linked);
	autoscroll = true;
	is_navigating = false;
	if (type === "popstate") restore_snapshot(current_navigation_index);
	nav.fulfil(void 0);
	if (nav.navigation.to) nav.navigation.to.scroll = scroll_state();
	after_navigate_callbacks.forEach((fn) => fn(nav.navigation));
	stores.navigating.set(navigating.current = null);
}
/**
* Does a full page reload if it wouldn't result in an endless loop in the SPA case
* @param {URL} url
* @param {{ id: string | null }} route
* @param {App.Error} error
* @param {number} status
* @param {boolean} [replace_state]
* @returns {Promise<import('./types.js').NavigationFinished>}
*/
async function server_fallback(url, route, error, status, replace_state) {
	if (url.origin === origin && url.pathname === location.pathname && !hydrated) return await load_root_error_page({
		status,
		error,
		url,
		route
	});
	return await native_navigation(url, replace_state);
}
/**
* @param {unknown} error
* @param {import('@sveltejs/kit').NavigationEvent} event
* @returns {import('types').MaybePromise<App.Error>}
*/
function handle_error(error, event) {
	if (error instanceof HttpError) return error.body;
	const status = get_status(error);
	const message = get_message(error);
	return app.hooks.handleError({
		error,
		event,
		status,
		message
	}) ?? { message };
}
/**
* Allows you to navigate programmatically to a given route, with options such as keeping the current element focused.
* Returns a Promise that resolves when SvelteKit navigates (or fails to navigate, in which case the promise rejects) to the specified `url`.
*
* For external URLs, use `window.location = url` instead of calling `goto(url)`.
*
* @param {string | URL} url Where to navigate to. Note that if you've set [`config.kit.paths.base`](https://svelte.dev/docs/kit/configuration#paths) and the URL is root-relative, you need to prepend the base path if you want to navigate within the app.
* @param {Object} [opts] Options related to the navigation
* @param {boolean} [opts.replaceState] If `true`, will replace the current `history` entry rather than creating a new one with `pushState`
* @param {boolean} [opts.noScroll] If `true`, the browser will maintain its scroll position rather than scrolling to the top of the page after navigation
* @param {boolean} [opts.keepFocus] If `true`, the currently focused element will retain focus after navigation. Otherwise, focus will be reset to the body
* @param {boolean} [opts.invalidateAll] If `true`, all `load` functions of the page will be rerun. See https://svelte.dev/docs/kit/load#rerunning-load-functions for more info on invalidation.
* @param {Array<string | URL | ((url: URL) => boolean)>} [opts.invalidate] Causes any load functions to re-run if they depend on one of the urls
* @param {App.PageState} [opts.state] An optional object that will be available as `page.state`
* @returns {Promise<void>}
*/
function goto(url, opts = {}) {
	throw new Error("Cannot call goto(...) on the server");
}
/**
* @param {string | URL | ((url: URL) => boolean)} resource The invalidated URL
*/
function push_invalidated(resource) {
	if (typeof resource === "function") invalidated.push(resource);
	else {
		const { href } = new URL(resource, location.href);
		invalidated.push((url) => url.href === href);
	}
}
/**
* @param {URL} url
* @param {boolean[]} invalid
* @returns {Promise<import('types').ServerNodesResponse | import('types').ServerRedirectNode>}
*/
async function load_data(url, invalid) {
	const data_url = new URL(url);
	data_url.pathname = add_data_suffix(url.pathname);
	if (url.pathname.endsWith("/")) data_url.searchParams.append(TRAILING_SLASH_PARAM, "1");
	data_url.searchParams.append(INVALIDATED_PARAM, invalid.map((i) => i ? "1" : "0").join(""));
	const res = await (0, window.fetch)(data_url.href, {});
	if (!res.ok) {
		/** @type {string | undefined} */
		let message;
		if (res.headers.get("content-type")?.includes("application/json")) message = await res.json();
		else if (res.status === 404) message = "Not Found";
		else if (res.status === 500) message = "Internal Error";
		throw new HttpError(res.status, message);
	}
	return new Promise(async (resolve) => {
		/**
		* Map of deferred promises that will be resolved by a subsequent chunk of data
		* @type {Map<string, import('types').Deferred>}
		*/
		const deferreds = /* @__PURE__ */ new Map();
		const reader = res.body.getReader();
		/**
		* @param {any} data
		*/
		function deserialize(data) {
			return devalue.unflatten(data, {
				...app.decoders,
				Promise: (id) => {
					return new Promise((fulfil, reject) => {
						deferreds.set(id, {
							fulfil,
							reject
						});
					});
				}
			});
		}
		for await (const node of read_ndjson(reader)) {
			if (node.type === "redirect") return resolve(node);
			if (node.type === "data") {
				node.nodes?.forEach((node) => {
					if (node?.type === "data") {
						node.uses = deserialize_uses(node.uses);
						node.data = deserialize(node.data);
					}
				});
				resolve(node);
			} else if (node.type === "chunk") {
				const { id, data, error } = node;
				const deferred = deferreds.get(id);
				deferreds.delete(id);
				if (error) deferred.reject(deserialize(error));
				else deferred.fulfil(deserialize(data));
			}
		}
	});
}
/**
* @param {any} uses
* @return {import('types').Uses}
*/
function deserialize_uses(uses) {
	return {
		dependencies: new Set(uses?.dependencies ?? []),
		params: new Set(uses?.params ?? []),
		parent: !!uses?.parent,
		route: !!uses?.route,
		url: !!uses?.url,
		search_params: new Set(uses?.search_params ?? [])
	};
}
/**
* @param {URL} url
* @param {boolean} [scroll]
*/
function reset_focus(url, scroll = true) {
	const autofocus = document.querySelector("[autofocus]");
	if (autofocus) autofocus.focus();
	else {
		const id = get_id(url);
		if (id && document.getElementById(id)) {
			const { x, y } = scroll_state();
			setTimeout(() => {
				const history_state = history.state;
				location.replace(new URL(`#${id}`, location.href));
				history.replaceState(history_state, "", url);
				if (scroll) scrollTo(x, y);
			});
		} else {
			const root = document.body;
			const tabindex = root.getAttribute("tabindex");
			root.tabIndex = -1;
			root.focus({
				preventScroll: true,
				focusVisible: false
			});
			if (tabindex !== null) root.setAttribute("tabindex", tabindex);
			else root.removeAttribute("tabindex");
		}
		const selection = getSelection();
		if (selection && selection.type !== "None") {
			/** @type {Range[]} */
			const ranges = [];
			for (let i = 0; i < selection.rangeCount; i += 1) ranges.push(selection.getRangeAt(i));
			setTimeout(() => {
				if (selection.rangeCount !== ranges.length) return;
				for (let i = 0; i < selection.rangeCount; i += 1) {
					const a = ranges[i];
					const b = selection.getRangeAt(i);
					if (a.commonAncestorContainer !== b.commonAncestorContainer || a.startContainer !== b.startContainer || a.endContainer !== b.endContainer || a.startOffset !== b.startOffset || a.endOffset !== b.endOffset) return;
				}
				selection.removeAllRanges();
			});
		}
	}
}
/**
* @template {import('@sveltejs/kit').NavigationType} T
* @param {import('./types.js').NavigationState} current
* @param {import('./types.js').NavigationIntent | undefined} intent
* @param {URL | null} url
* @param {T} type
* @param {{ x: number, y: number } | null} [target_scroll] The scroll position for the target (for popstate navigations)
*/
function create_navigation(current, intent, url, type, target_scroll = null) {
	/** @type {(value: any) => void} */
	let fulfil;
	/** @type {(error: any) => void} */
	let reject;
	const complete = new Promise((f, r) => {
		fulfil = f;
		reject = r;
	});
	complete.catch(noop);
	return {
		navigation: {
			from: {
				params: current.params,
				route: { id: current.route?.id ?? null },
				url: current.url,
				scroll: scroll_state()
			},
			to: url && {
				params: intent?.params ?? null,
				route: { id: intent?.route?.id ?? null },
				url,
				scroll: target_scroll
			},
			willUnload: !intent,
			type,
			complete
		},
		fulfil,
		reject
	};
}
/**
* TODO: remove this in 3.0 when the page store is also removed
*
* We need to assign a new page object so that subscribers are correctly notified.
* However, spreading `{ ...page }` returns an empty object so we manually
* assign to each property instead.
*
* @param {import('@sveltejs/kit').Page} page
*/
function clone_page(page) {
	return {
		data: page.data,
		error: page.error,
		form: page.form,
		params: page.params,
		route: page.route,
		state: page.state,
		status: page.status,
		url: page.url
	};
}
/**
* @param {URL} url
* @returns {string}
*/
function get_id(url) {
	let id;
	if (app.hash) {
		const [, , second] = url.hash.split("#", 3);
		id = second ?? "";
	} else id = url.hash.slice(1);
	return decodeURIComponent(id);
}
//#endregion
export { updated as a, page as i, stores as n, navigating as r, goto as t };
