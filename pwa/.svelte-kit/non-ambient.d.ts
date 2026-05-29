
// this file is generated — do not edit it


declare module "svelte/elements" {
	export interface HTMLAttributes<T> {
		'data-sveltekit-keepfocus'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-noscroll'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-preload-code'?:
			| true
			| ''
			| 'eager'
			| 'viewport'
			| 'hover'
			| 'tap'
			| 'off'
			| undefined
			| null;
		'data-sveltekit-preload-data'?: true | '' | 'hover' | 'tap' | 'off' | undefined | null;
		'data-sveltekit-reload'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-replacestate'?: true | '' | 'off' | undefined | null;
	}
}

export {};


declare module "$app/types" {
	type MatcherParam<M> = M extends (param : string) => param is (infer U extends string) ? U : string;

	export interface AppTypes {
		RouteId(): "/(app)" | "/" | "/(app)/calendar" | "/(app)/cards" | "/(app)/categories" | "/login" | "/(app)/settings" | "/(app)/stats" | "/(app)/timer";
		RouteParams(): {
			
		};
		LayoutParams(): {
			"/(app)": Record<string, never>;
			"/": Record<string, never>;
			"/(app)/calendar": Record<string, never>;
			"/(app)/cards": Record<string, never>;
			"/(app)/categories": Record<string, never>;
			"/login": Record<string, never>;
			"/(app)/settings": Record<string, never>;
			"/(app)/stats": Record<string, never>;
			"/(app)/timer": Record<string, never>
		};
		Pathname(): "/" | "/calendar" | "/cards" | "/categories" | "/login" | "/settings" | "/stats" | "/timer";
		ResolvedPathname(): `${"" | `/${string}`}${ReturnType<AppTypes['Pathname']>}`;
		Asset(): "/favicon.svg" | "/icons.svg" | "/locales/en/common.json" | "/locales/en/login.json" | "/locales/en/todo.json" | "/locales/it/common.json" | "/locales/it/login.json" | "/locales/it/todo.json" | "/manifest.json" | "/sw.js" | string & {};
	}
}