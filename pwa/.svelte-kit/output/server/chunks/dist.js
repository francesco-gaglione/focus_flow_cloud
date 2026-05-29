import { n as onDestroy, o as __exportAll, s as __reExport } from "./index-server.js";
import { F as readable, N as derived, P as get, a as bind_props, at as fallback, f as slot, ft as setContext, ut as getContext } from "./dev.js";
import "./index-server2.js";
import { InfiniteQueryObserver, MutationObserver, QueriesObserver, QueryClient, QueryObserver, hydrate, noop, notifyManager, replaceEqualDeep } from "@tanstack/query-core";
//#region node_modules/@tanstack/svelte-query/dist/context.js
var _contextKey = "$$_queryClient";
/** Retrieves a Client from Svelte's context */
var getQueryClientContext = () => {
	const client = getContext(_contextKey);
	if (!client) throw new Error("No QueryClient was found in Svelte context. Did you forget to wrap your component with QueryClientProvider?");
	return client;
};
/** Sets a QueryClient on Svelte's context */
var setQueryClientContext = (client) => {
	setContext(_contextKey, client);
};
var _isRestoringContextKey = "$$_isRestoring";
/** Retrieves a `isRestoring` from Svelte's context */
var getIsRestoringContext = () => {
	try {
		const isRestoring = getContext(_isRestoringContextKey);
		return isRestoring ? isRestoring : readable(false);
	} catch (error) {
		return readable(false);
	}
};
/** Sets a `isRestoring` on Svelte's context */
var setIsRestoringContext = (isRestoring) => {
	setContext(_isRestoringContextKey, isRestoring);
};
//#endregion
//#region node_modules/@tanstack/svelte-query/dist/useIsRestoring.js
function useIsRestoring() {
	return getIsRestoringContext();
}
//#endregion
//#region node_modules/@tanstack/svelte-query/dist/useQueryClient.js
function useQueryClient(queryClient) {
	if (queryClient) return queryClient;
	return getQueryClientContext();
}
//#endregion
//#region node_modules/@tanstack/svelte-query/dist/utils.js
function isSvelteStore(obj) {
	return "subscribe" in obj && typeof obj.subscribe === "function";
}
//#endregion
//#region node_modules/@tanstack/svelte-query/dist/createBaseQuery.js
function createBaseQuery(options, Observer, queryClient) {
	/** Load query client */
	const client = useQueryClient(queryClient);
	const isRestoring = useIsRestoring();
	/** Creates a store that has the default options applied */
	const defaultedOptionsStore = derived([isSvelteStore(options) ? options : readable(options), isRestoring], ([$optionsStore, $isRestoring]) => {
		const defaultedOptions = client.defaultQueryOptions($optionsStore);
		defaultedOptions._optimisticResults = $isRestoring ? "isRestoring" : "optimistic";
		return defaultedOptions;
	});
	/** Creates the observer */
	const observer = new Observer(client, get(defaultedOptionsStore));
	defaultedOptionsStore.subscribe(($defaultedOptions) => {
		observer.setOptions($defaultedOptions);
	});
	/** Subscribe to changes in result and defaultedOptionsStore */
	const { subscribe } = derived([derived(isRestoring, ($isRestoring, set) => {
		const unsubscribe = $isRestoring ? noop : observer.subscribe(notifyManager.batchCalls(set));
		observer.updateResult();
		return unsubscribe;
	}), defaultedOptionsStore], ([$result, $defaultedOptionsStore]) => {
		$result = observer.getOptimisticResult($defaultedOptionsStore);
		return !$defaultedOptionsStore.notifyOnChangeProps ? observer.trackResult($result) : $result;
	});
	return { subscribe };
}
//#endregion
//#region node_modules/@tanstack/svelte-query/dist/createQuery.js
function createQuery(options, queryClient) {
	return createBaseQuery(options, QueryObserver, queryClient);
}
//#endregion
//#region node_modules/@tanstack/svelte-query/dist/queryOptions.js
function queryOptions(options) {
	return options;
}
//#endregion
//#region node_modules/@tanstack/svelte-query/dist/createQueries.js
function createQueries({ queries, ...options }, queryClient) {
	const client = useQueryClient(queryClient);
	const isRestoring = useIsRestoring();
	const defaultedQueriesStore = derived([isSvelteStore(queries) ? queries : readable(queries), isRestoring], ([$queries, $isRestoring]) => {
		return $queries.map((opts) => {
			const defaultedOptions = client.defaultQueryOptions(opts);
			defaultedOptions._optimisticResults = $isRestoring ? "isRestoring" : "optimistic";
			return defaultedOptions;
		});
	});
	const observer = new QueriesObserver(client, get(defaultedQueriesStore), options);
	defaultedQueriesStore.subscribe(($defaultedQueries) => {
		observer.setQueries($defaultedQueries, options);
	});
	const { subscribe } = derived([derived([isRestoring], ([$isRestoring], set) => {
		const unsubscribe = $isRestoring ? noop : observer.subscribe(notifyManager.batchCalls(set));
		return () => unsubscribe();
	}), defaultedQueriesStore], ([$result, $defaultedQueriesStore]) => {
		const [rawResult, combineResult, trackResult] = observer.getOptimisticResult($defaultedQueriesStore, options.combine);
		$result = rawResult;
		return combineResult(trackResult());
	});
	return { subscribe };
}
//#endregion
//#region node_modules/@tanstack/svelte-query/dist/createInfiniteQuery.js
function createInfiniteQuery(options, queryClient) {
	return createBaseQuery(options, InfiniteQueryObserver, queryClient);
}
//#endregion
//#region node_modules/@tanstack/svelte-query/dist/infiniteQueryOptions.js
function infiniteQueryOptions(options) {
	return options;
}
//#endregion
//#region node_modules/@tanstack/svelte-query/dist/createMutation.js
function createMutation(options, queryClient) {
	const client = useQueryClient(queryClient);
	const optionsStore = isSvelteStore(options) ? options : readable(options);
	const observer = new MutationObserver(client, get(optionsStore));
	let mutate;
	optionsStore.subscribe(($options) => {
		mutate = (variables, mutateOptions) => {
			observer.mutate(variables, mutateOptions).catch(noop);
		};
		observer.setOptions($options);
	});
	const { subscribe } = derived(readable(observer.getCurrentResult(), (set) => {
		return observer.subscribe(notifyManager.batchCalls((val) => set(val)));
	}), ($result) => ({
		...$result,
		mutate,
		mutateAsync: $result.mutate
	}));
	return { subscribe };
}
//#endregion
//#region node_modules/@tanstack/svelte-query/dist/useMutationState.js
function getResult(mutationCache, options) {
	return mutationCache.findAll(options.filters).map((mutation) => options.select ? options.select(mutation) : mutation.state);
}
function useMutationState(options = {}, queryClient) {
	const mutationCache = useQueryClient(queryClient).getMutationCache();
	let result = getResult(mutationCache, options);
	const { subscribe } = readable(result, (set) => {
		return mutationCache.subscribe(notifyManager.batchCalls(() => {
			const nextResult = replaceEqualDeep(result, getResult(mutationCache, options));
			if (result !== nextResult) {
				result = nextResult;
				set(result);
			}
		}));
	});
	return { subscribe };
}
//#endregion
//#region node_modules/@tanstack/svelte-query/dist/useIsFetching.js
function useIsFetching(filters, queryClient) {
	const client = useQueryClient(queryClient);
	const cache = client.getQueryCache();
	let isFetching = client.isFetching(filters);
	const { subscribe } = readable(isFetching, (set) => {
		return cache.subscribe(notifyManager.batchCalls(() => {
			const newIsFetching = client.isFetching(filters);
			if (isFetching !== newIsFetching) {
				isFetching = newIsFetching;
				set(isFetching);
			}
		}));
	});
	return { subscribe };
}
//#endregion
//#region node_modules/@tanstack/svelte-query/dist/useIsMutating.js
function useIsMutating(filters, queryClient) {
	const client = useQueryClient(queryClient);
	const cache = client.getMutationCache();
	let isMutating = client.isMutating(filters);
	const { subscribe } = readable(isMutating, (set) => {
		return cache.subscribe(notifyManager.batchCalls(() => {
			const newIisMutating = client.isMutating(filters);
			if (isMutating !== newIisMutating) {
				isMutating = newIisMutating;
				set(isMutating);
			}
		}));
	});
	return { subscribe };
}
//#endregion
//#region node_modules/@tanstack/svelte-query/dist/useHydrate.js
function useHydrate(state, options, queryClient) {
	const client = useQueryClient(queryClient);
	if (state) hydrate(client, state, options);
}
//#endregion
//#region node_modules/@tanstack/svelte-query/dist/HydrationBoundary.svelte
function HydrationBoundary($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let state = $$props["state"];
		let options = fallback($$props["options"], void 0);
		let queryClient = fallback($$props["queryClient"], void 0);
		useHydrate(state, options, queryClient);
		$$renderer.push(`<!--[-->`);
		slot($$renderer, $$props, "default", {}, null);
		$$renderer.push(`<!--]-->`);
		bind_props($$props, {
			state,
			options,
			queryClient
		});
	});
}
//#endregion
//#region node_modules/@tanstack/svelte-query/dist/QueryClientProvider.svelte
function QueryClientProvider($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let client = fallback($$props["client"], () => new QueryClient(), true);
		setQueryClientContext(client);
		onDestroy(() => {
			client.unmount();
		});
		$$renderer.push(`<!--[-->`);
		slot($$renderer, $$props, "default", {}, null);
		$$renderer.push(`<!--]-->`);
		bind_props($$props, { client });
	});
}
//#endregion
//#region node_modules/@tanstack/svelte-query/dist/index.js
var dist_exports = /* @__PURE__ */ __exportAll({
	HydrationBoundary: () => HydrationBoundary,
	QueryClientProvider: () => QueryClientProvider,
	createInfiniteQuery: () => createInfiniteQuery,
	createMutation: () => createMutation,
	createQueries: () => createQueries,
	createQuery: () => createQuery,
	getIsRestoringContext: () => getIsRestoringContext,
	getQueryClientContext: () => getQueryClientContext,
	infiniteQueryOptions: () => infiniteQueryOptions,
	queryOptions: () => queryOptions,
	setIsRestoringContext: () => setIsRestoringContext,
	setQueryClientContext: () => setQueryClientContext,
	useHydrate: () => useHydrate,
	useIsFetching: () => useIsFetching,
	useIsMutating: () => useIsMutating,
	useIsRestoring: () => useIsRestoring,
	useMutationState: () => useMutationState,
	useQueryClient: () => useQueryClient
});
import * as import__tanstack_query_core from "@tanstack/query-core";
__reExport(dist_exports, import__tanstack_query_core);
//#endregion
export { useQueryClient as a, createQuery as i, QueryClientProvider as n, createMutation as r, dist_exports as t };
