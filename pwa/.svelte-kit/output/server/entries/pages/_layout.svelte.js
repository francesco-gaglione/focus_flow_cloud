import "../../chunks/index-server.js";
import "../../chunks/dev.js";
import { n as QueryClientProvider, t as dist_exports } from "../../chunks/dist.js";
import { i as init, n as addMessages, r as getLocaleFromNavigator } from "../../chunks/runtime.js";
var common_default$1 = { loading: "Loading..." };
var login_default$1 = {
	login_title: "Sign in to continue",
	username: "Username",
	password: "Password",
	sign_in: "Sign in"
};
var todo_default$1 = {
	completed: "Completed",
	no_tasks: "No tasks yet",
	no_category: "No category",
	all_categories: "All",
	category_filter: "Category"
};
var common_default = { loading: "Loading..." };
var login_default = {
	login_title: "Accedi per continuare",
	username: "Username",
	password: "Password",
	sign_in: "Accedi"
};
var todo_default = {
	completed: "Completati",
	no_tasks: "Ancora nessun task",
	no_category: "Senza categoria",
	all_categories: "Tutte",
	category_filter: "Categoria"
};
//#endregion
//#region src/lib/i18n.ts
var initialized = false;
function setupI18n() {
	if (initialized) return;
	initialized = true;
	addMessages("en", {
		common: common_default$1,
		login: login_default$1,
		todo: todo_default$1
	});
	addMessages("it", {
		common: common_default,
		login: login_default,
		todo: todo_default
	});
	init({
		fallbackLocale: "en",
		initialLocale: getLocaleFromNavigator() ?? "en"
	});
}
//#endregion
//#region src/routes/+layout.svelte
function _layout($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const { children } = $$props;
		setupI18n();
		QueryClientProvider($$renderer, {
			client: new dist_exports.QueryClient({ defaultOptions: { queries: {
				staleTime: 3e4,
				retry: (count, error) => {
					if (error instanceof Error && error.message.includes("401")) return false;
					return count < 2;
				}
			} } }),
			children: ($$renderer) => {
				children($$renderer);
				$$renderer.push(`<!---->`);
			},
			$$slots: { default: true }
		});
	});
}
//#endregion
export { _layout as default };
