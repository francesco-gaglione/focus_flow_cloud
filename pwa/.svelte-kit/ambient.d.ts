
// this file is generated — do not edit it


/// <reference types="@sveltejs/kit" />

/**
 * This module provides access to environment variables that are injected _statically_ into your bundle at build time and are limited to _private_ access.
 * 
 * |         | Runtime                                                                    | Build time                                                               |
 * | ------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
 * | Private | [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private) | [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private) |
 * | Public  | [`$env/dynamic/public`](https://svelte.dev/docs/kit/$env-dynamic-public)   | [`$env/static/public`](https://svelte.dev/docs/kit/$env-static-public)   |
 * 
 * Static environment variables are [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env` at build time and then statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 * 
 * **_Private_ access:**
 * 
 * - This module cannot be imported into client-side code
 * - This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured)
 * 
 * For example, given the following build time environment:
 * 
 * ```env
 * ENVIRONMENT=production
 * PUBLIC_BASE_URL=http://site.com
 * ```
 * 
 * With the default `publicPrefix` and `privatePrefix`:
 * 
 * ```ts
 * import { ENVIRONMENT, PUBLIC_BASE_URL } from '$env/static/private';
 * 
 * console.log(ENVIRONMENT); // => "production"
 * console.log(PUBLIC_BASE_URL); // => throws error during build
 * ```
 * 
 * The above values will be the same _even if_ different values for `ENVIRONMENT` or `PUBLIC_BASE_URL` are set at runtime, as they are statically replaced in your code with their build time values.
 */
declare module '$env/static/private' {
	export const VITE_API_BASE_URL: string;
	export const NVM_INC: string;
	export const STARSHIP_SHELL: string;
	export const rvm_use_flag: string;
	export const ZED_FILE: string;
	export const rvm_bin_path: string;
	export const TERM_PROGRAM: string;
	export const rvm_quiet_flag: string;
	export const NVM_CD_FLAGS: string;
	export const ANDROID_HOME: string;
	export const rvm_gemstone_url: string;
	export const TERM: string;
	export const SHELL: string;
	export const ZED_FILENAME: string;
	export const rvm_docs_type: string;
	export const TMPDIR: string;
	export const PROTO_HOME: string;
	export const HOMEBREW_REPOSITORY: string;
	export const TERM_PROGRAM_VERSION: string;
	export const WINDOWID: string;
	export const FPATH: string;
	export const ANDROID_SDK_ROOT: string;
	export const ZED_SYMBOL: string;
	export const OLDPWD: string;
	export const rvm_hook: string;
	export const ZED_RELATIVE_FILE: string;
	export const USER: string;
	export const NVM_DIR: string;
	export const rvm_gemstone_package_file: string;
	export const COMMAND_MODE: string;
	export const rvm_path: string;
	export const ZED_WORKTREE_ROOT: string;
	export const ZED_MAIN_GIT_WORKTREE: string;
	export const SSH_AUTH_SOCK: string;
	export const ENVMAN_LOAD: string;
	export const __CF_USER_TEXT_ENCODING: string;
	export const rvm_proxy: string;
	export const rvm_ruby_file: string;
	export const ZED_STEM: string;
	export const ZED_ENVIRONMENT: string;
	export const rvm_silent_flag: string;
	export const rvm_prefix: string;
	export const ZED_ROW: string;
	export const rvm_ruby_make: string;
	export const PATH: string;
	export const ZED_COLUMN: string;
	export const LaunchInstanceID: string;
	export const __CFBundleIdentifier: string;
	export const PWD: string;
	export const rvm_sdk: string;
	export const LANG: string;
	export const XPC_FLAGS: string;
	export const XPC_SERVICE_NAME: string;
	export const rvm_version: string;
	export const ZED_DIRNAME: string;
	export const SHLVL: string;
	export const rvm_script_name: string;
	export const rvm_pretty_print_flag: string;
	export const HOME: string;
	export const rvm_ruby_mode: string;
	export const HOMEBREW_PREFIX: string;
	export const STARSHIP_SESSION_KEY: string;
	export const LOGNAME: string;
	export const rvm_alias_expanded: string;
	export const ALACRITTY_WINDOW_ID: string;
	export const XDG_DATA_DIRS: string;
	export const ZED_TERM: string;
	export const NVM_BIN: string;
	export const BUN_INSTALL: string;
	export const rvm_nightly_flag: string;
	export const rvm_ruby_make_install: string;
	export const INFOPATH: string;
	export const HOMEBREW_CELLAR: string;
	export const ANDROID_NDK_HOME: string;
	export const rvm_niceness: string;
	export const rvm_ruby_bits: string;
	export const OSLogRateLimit: string;
	export const rvm_bin_flag: string;
	export const ZED_RELATIVE_DIR: string;
	export const rvm_only_path_flag: string;
	export const SECURITYSESSIONID: string;
	export const ZED_LANGUAGE: string;
	export const COLORTERM: string;
	export const _: string;
	export const npm_config_local_prefix: string;
	export const npm_config_user_agent: string;
	export const npm_execpath: string;
	export const npm_package_name: string;
	export const npm_package_json: string;
	export const npm_package_version: string;
	export const NODE: string;
	export const npm_node_execpath: string;
	export const npm_command: string;
	export const npm_lifecycle_event: string;
	export const npm_lifecycle_script: string;
	export const NODE_ENV: string;
}

/**
 * This module provides access to environment variables that are injected _statically_ into your bundle at build time and are _publicly_ accessible.
 * 
 * |         | Runtime                                                                    | Build time                                                               |
 * | ------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
 * | Private | [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private) | [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private) |
 * | Public  | [`$env/dynamic/public`](https://svelte.dev/docs/kit/$env-dynamic-public)   | [`$env/static/public`](https://svelte.dev/docs/kit/$env-static-public)   |
 * 
 * Static environment variables are [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env` at build time and then statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 * 
 * **_Public_ access:**
 * 
 * - This module _can_ be imported into client-side code
 * - **Only** variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`) are included
 * 
 * For example, given the following build time environment:
 * 
 * ```env
 * ENVIRONMENT=production
 * PUBLIC_BASE_URL=http://site.com
 * ```
 * 
 * With the default `publicPrefix` and `privatePrefix`:
 * 
 * ```ts
 * import { ENVIRONMENT, PUBLIC_BASE_URL } from '$env/static/public';
 * 
 * console.log(ENVIRONMENT); // => throws error during build
 * console.log(PUBLIC_BASE_URL); // => "http://site.com"
 * ```
 * 
 * The above values will be the same _even if_ different values for `ENVIRONMENT` or `PUBLIC_BASE_URL` are set at runtime, as they are statically replaced in your code with their build time values.
 */
declare module '$env/static/public' {
	
}

/**
 * This module provides access to environment variables set _dynamically_ at runtime and that are limited to _private_ access.
 * 
 * |         | Runtime                                                                    | Build time                                                               |
 * | ------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
 * | Private | [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private) | [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private) |
 * | Public  | [`$env/dynamic/public`](https://svelte.dev/docs/kit/$env-dynamic-public)   | [`$env/static/public`](https://svelte.dev/docs/kit/$env-static-public)   |
 * 
 * Dynamic environment variables are defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/main/packages/adapter-node) (or running [`vite preview`](https://svelte.dev/docs/kit/cli)), this is equivalent to `process.env`.
 * 
 * **_Private_ access:**
 * 
 * - This module cannot be imported into client-side code
 * - This module includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured)
 * 
 * > [!NOTE] In `dev`, `$env/dynamic` includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 * 
 * > [!NOTE] To get correct types, environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 * >
 * > ```env
 * > MY_FEATURE_FLAG=
 * > ```
 * >
 * > You can override `.env` values from the command line like so:
 * >
 * > ```sh
 * > MY_FEATURE_FLAG="enabled" npm run dev
 * > ```
 * 
 * For example, given the following runtime environment:
 * 
 * ```env
 * ENVIRONMENT=production
 * PUBLIC_BASE_URL=http://site.com
 * ```
 * 
 * With the default `publicPrefix` and `privatePrefix`:
 * 
 * ```ts
 * import { env } from '$env/dynamic/private';
 * 
 * console.log(env.ENVIRONMENT); // => "production"
 * console.log(env.PUBLIC_BASE_URL); // => undefined
 * ```
 */
declare module '$env/dynamic/private' {
	export const env: {
		VITE_API_BASE_URL: string;
		NVM_INC: string;
		STARSHIP_SHELL: string;
		rvm_use_flag: string;
		ZED_FILE: string;
		rvm_bin_path: string;
		TERM_PROGRAM: string;
		rvm_quiet_flag: string;
		NVM_CD_FLAGS: string;
		ANDROID_HOME: string;
		rvm_gemstone_url: string;
		TERM: string;
		SHELL: string;
		ZED_FILENAME: string;
		rvm_docs_type: string;
		TMPDIR: string;
		PROTO_HOME: string;
		HOMEBREW_REPOSITORY: string;
		TERM_PROGRAM_VERSION: string;
		WINDOWID: string;
		FPATH: string;
		ANDROID_SDK_ROOT: string;
		ZED_SYMBOL: string;
		OLDPWD: string;
		rvm_hook: string;
		ZED_RELATIVE_FILE: string;
		USER: string;
		NVM_DIR: string;
		rvm_gemstone_package_file: string;
		COMMAND_MODE: string;
		rvm_path: string;
		ZED_WORKTREE_ROOT: string;
		ZED_MAIN_GIT_WORKTREE: string;
		SSH_AUTH_SOCK: string;
		ENVMAN_LOAD: string;
		__CF_USER_TEXT_ENCODING: string;
		rvm_proxy: string;
		rvm_ruby_file: string;
		ZED_STEM: string;
		ZED_ENVIRONMENT: string;
		rvm_silent_flag: string;
		rvm_prefix: string;
		ZED_ROW: string;
		rvm_ruby_make: string;
		PATH: string;
		ZED_COLUMN: string;
		LaunchInstanceID: string;
		__CFBundleIdentifier: string;
		PWD: string;
		rvm_sdk: string;
		LANG: string;
		XPC_FLAGS: string;
		XPC_SERVICE_NAME: string;
		rvm_version: string;
		ZED_DIRNAME: string;
		SHLVL: string;
		rvm_script_name: string;
		rvm_pretty_print_flag: string;
		HOME: string;
		rvm_ruby_mode: string;
		HOMEBREW_PREFIX: string;
		STARSHIP_SESSION_KEY: string;
		LOGNAME: string;
		rvm_alias_expanded: string;
		ALACRITTY_WINDOW_ID: string;
		XDG_DATA_DIRS: string;
		ZED_TERM: string;
		NVM_BIN: string;
		BUN_INSTALL: string;
		rvm_nightly_flag: string;
		rvm_ruby_make_install: string;
		INFOPATH: string;
		HOMEBREW_CELLAR: string;
		ANDROID_NDK_HOME: string;
		rvm_niceness: string;
		rvm_ruby_bits: string;
		OSLogRateLimit: string;
		rvm_bin_flag: string;
		ZED_RELATIVE_DIR: string;
		rvm_only_path_flag: string;
		SECURITYSESSIONID: string;
		ZED_LANGUAGE: string;
		COLORTERM: string;
		_: string;
		npm_config_local_prefix: string;
		npm_config_user_agent: string;
		npm_execpath: string;
		npm_package_name: string;
		npm_package_json: string;
		npm_package_version: string;
		NODE: string;
		npm_node_execpath: string;
		npm_command: string;
		npm_lifecycle_event: string;
		npm_lifecycle_script: string;
		NODE_ENV: string;
		[key: `PUBLIC_${string}`]: undefined;
		[key: `${string}`]: string | undefined;
	}
}

/**
 * This module provides access to environment variables set _dynamically_ at runtime and that are _publicly_ accessible.
 * 
 * |         | Runtime                                                                    | Build time                                                               |
 * | ------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
 * | Private | [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private) | [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private) |
 * | Public  | [`$env/dynamic/public`](https://svelte.dev/docs/kit/$env-dynamic-public)   | [`$env/static/public`](https://svelte.dev/docs/kit/$env-static-public)   |
 * 
 * Dynamic environment variables are defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/main/packages/adapter-node) (or running [`vite preview`](https://svelte.dev/docs/kit/cli)), this is equivalent to `process.env`.
 * 
 * **_Public_ access:**
 * 
 * - This module _can_ be imported into client-side code
 * - **Only** variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`) are included
 * 
 * > [!NOTE] In `dev`, `$env/dynamic` includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 * 
 * > [!NOTE] To get correct types, environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 * >
 * > ```env
 * > MY_FEATURE_FLAG=
 * > ```
 * >
 * > You can override `.env` values from the command line like so:
 * >
 * > ```sh
 * > MY_FEATURE_FLAG="enabled" npm run dev
 * > ```
 * 
 * For example, given the following runtime environment:
 * 
 * ```env
 * ENVIRONMENT=production
 * PUBLIC_BASE_URL=http://example.com
 * ```
 * 
 * With the default `publicPrefix` and `privatePrefix`:
 * 
 * ```ts
 * import { env } from '$env/dynamic/public';
 * console.log(env.ENVIRONMENT); // => undefined, not public
 * console.log(env.PUBLIC_BASE_URL); // => "http://example.com"
 * ```
 * 
 * ```
 * 
 * ```
 */
declare module '$env/dynamic/public' {
	export const env: {
		[key: `PUBLIC_${string}`]: string | undefined;
	}
}
