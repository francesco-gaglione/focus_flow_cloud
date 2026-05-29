import { I as writable, N as derived } from "./dev.js";
import "./index-server2.js";
import deepmerge from "deepmerge";
import { IntlMessageFormat } from "intl-messageformat";
//#region node_modules/svelte-i18n/dist/runtime.js
function delve(obj, fullKey) {
	if (fullKey == null) return void 0;
	if (fullKey in obj) return obj[fullKey];
	const keys = fullKey.split(".");
	let result = obj;
	for (let p = 0; p < keys.length; p++) if (typeof result === "object") {
		if (p > 0) {
			const partialKey = keys.slice(p, keys.length).join(".");
			if (partialKey in result) {
				result = result[partialKey];
				break;
			}
		}
		result = result[keys[p]];
	} else result = void 0;
	return result;
}
var lookupCache = {};
var addToCache = (path, locale, message) => {
	if (!message) return message;
	if (!(locale in lookupCache)) lookupCache[locale] = {};
	if (!(path in lookupCache[locale])) lookupCache[locale][path] = message;
	return message;
};
var lookup = (path, refLocale) => {
	if (refLocale == null) return void 0;
	if (refLocale in lookupCache && path in lookupCache[refLocale]) return lookupCache[refLocale][path];
	const locales = getPossibleLocales(refLocale);
	for (let i = 0; i < locales.length; i++) {
		const locale = locales[i];
		const message = getMessageFromDictionary(locale, path);
		if (message) return addToCache(path, refLocale, message);
	}
};
var dictionary;
var $dictionary = writable({});
function getLocaleDictionary(locale) {
	return dictionary[locale] || null;
}
function hasLocaleDictionary(locale) {
	return locale in dictionary;
}
function getMessageFromDictionary(locale, id) {
	if (!hasLocaleDictionary(locale)) return null;
	return delve(getLocaleDictionary(locale), id);
}
function getClosestAvailableLocale(refLocale) {
	if (refLocale == null) return void 0;
	const relatedLocales = getPossibleLocales(refLocale);
	for (let i = 0; i < relatedLocales.length; i++) {
		const locale = relatedLocales[i];
		if (hasLocaleDictionary(locale)) return locale;
	}
}
function addMessages(locale, ...partials) {
	delete lookupCache[locale];
	$dictionary.update((d) => {
		d[locale] = deepmerge.all([d[locale] || {}, ...partials]);
		return d;
	});
}
derived([$dictionary], ([dictionary2]) => Object.keys(dictionary2));
$dictionary.subscribe((newDictionary) => dictionary = newDictionary);
var queue = {};
function removeLoaderFromQueue(locale, loader) {
	queue[locale].delete(loader);
	if (queue[locale].size === 0) delete queue[locale];
}
function getLocaleQueue(locale) {
	return queue[locale];
}
function getLocalesQueues(locale) {
	return getPossibleLocales(locale).map((localeItem) => {
		const localeQueue = getLocaleQueue(localeItem);
		return [localeItem, localeQueue ? [...localeQueue] : []];
	}).filter(([, localeQueue]) => localeQueue.length > 0);
}
function hasLocaleQueue(locale) {
	if (locale == null) return false;
	return getPossibleLocales(locale).some((localeQueue) => {
		var _a;
		return (_a = getLocaleQueue(localeQueue)) == null ? void 0 : _a.size;
	});
}
function loadLocaleQueue(locale, localeQueue) {
	return Promise.all(localeQueue.map((loader) => {
		removeLoaderFromQueue(locale, loader);
		return loader().then((partial) => partial.default || partial);
	})).then((partials) => addMessages(locale, ...partials));
}
var activeFlushes = {};
function flush(locale) {
	if (!hasLocaleQueue(locale)) {
		if (locale in activeFlushes) return activeFlushes[locale];
		return Promise.resolve();
	}
	const queues = getLocalesQueues(locale);
	activeFlushes[locale] = Promise.all(queues.map(([localeName, localeQueue]) => loadLocaleQueue(localeName, localeQueue))).then(() => {
		if (hasLocaleQueue(locale)) return flush(locale);
		delete activeFlushes[locale];
	});
	return activeFlushes[locale];
}
var __getOwnPropSymbols$2 = Object.getOwnPropertySymbols;
var __hasOwnProp$2 = Object.prototype.hasOwnProperty;
var __propIsEnum$2 = Object.prototype.propertyIsEnumerable;
var __objRest$1 = (source, exclude) => {
	var target = {};
	for (var prop in source) if (__hasOwnProp$2.call(source, prop) && exclude.indexOf(prop) < 0) target[prop] = source[prop];
	if (source != null && __getOwnPropSymbols$2) {
		for (var prop of __getOwnPropSymbols$2(source)) if (exclude.indexOf(prop) < 0 && __propIsEnum$2.call(source, prop)) target[prop] = source[prop];
	}
	return target;
};
var defaultFormats = {
	number: {
		scientific: { notation: "scientific" },
		engineering: { notation: "engineering" },
		compactLong: {
			notation: "compact",
			compactDisplay: "long"
		},
		compactShort: {
			notation: "compact",
			compactDisplay: "short"
		}
	},
	date: {
		short: {
			month: "numeric",
			day: "numeric",
			year: "2-digit"
		},
		medium: {
			month: "short",
			day: "numeric",
			year: "numeric"
		},
		long: {
			month: "long",
			day: "numeric",
			year: "numeric"
		},
		full: {
			weekday: "long",
			month: "long",
			day: "numeric",
			year: "numeric"
		}
	},
	time: {
		short: {
			hour: "numeric",
			minute: "numeric"
		},
		medium: {
			hour: "numeric",
			minute: "numeric",
			second: "numeric"
		},
		long: {
			hour: "numeric",
			minute: "numeric",
			second: "numeric",
			timeZoneName: "short"
		},
		full: {
			hour: "numeric",
			minute: "numeric",
			second: "numeric",
			timeZoneName: "short"
		}
	}
};
function defaultMissingKeyHandler({ locale, id }) {
	console.warn(`[svelte-i18n] The message "${id}" was not found in "${getPossibleLocales(locale).join("\", \"")}".${hasLocaleQueue(getCurrentLocale()) ? `

Note: there are at least one loader still registered to this locale that wasn't executed.` : ""}`);
}
var options = {
	fallbackLocale: null,
	loadingDelay: 200,
	formats: defaultFormats,
	warnOnMissingMessages: true,
	handleMissingMessage: void 0,
	ignoreTag: true
};
function getOptions() {
	return options;
}
function init(opts) {
	const _a = opts, { formats } = _a, rest = __objRest$1(_a, ["formats"]);
	let initialLocale = opts.fallbackLocale;
	if (opts.initialLocale) try {
		if (IntlMessageFormat.resolveLocale(opts.initialLocale)) initialLocale = opts.initialLocale;
	} catch (e) {
		console.warn(`[svelte-i18n] The initial locale "${opts.initialLocale}" is not a valid locale.`);
	}
	if (rest.warnOnMissingMessages) {
		delete rest.warnOnMissingMessages;
		if (rest.handleMissingMessage == null) rest.handleMissingMessage = defaultMissingKeyHandler;
		else console.warn("[svelte-i18n] The \"warnOnMissingMessages\" option is deprecated. Please use the \"handleMissingMessage\" option instead.");
	}
	Object.assign(options, rest, { initialLocale });
	if (formats) {
		if ("number" in formats) Object.assign(options.formats.number, formats.number);
		if ("date" in formats) Object.assign(options.formats.date, formats.date);
		if ("time" in formats) Object.assign(options.formats.time, formats.time);
	}
	return $locale.set(initialLocale);
}
var $isLoading = writable(false);
var __defProp$1 = Object.defineProperty;
var __defProps = Object.defineProperties;
var __getOwnPropDescs = Object.getOwnPropertyDescriptors;
var __getOwnPropSymbols$1 = Object.getOwnPropertySymbols;
var __hasOwnProp$1 = Object.prototype.hasOwnProperty;
var __propIsEnum$1 = Object.prototype.propertyIsEnumerable;
var __defNormalProp$1 = (obj, key, value) => key in obj ? __defProp$1(obj, key, {
	enumerable: true,
	configurable: true,
	writable: true,
	value
}) : obj[key] = value;
var __spreadValues$1 = (a, b) => {
	for (var prop in b || (b = {})) if (__hasOwnProp$1.call(b, prop)) __defNormalProp$1(a, prop, b[prop]);
	if (__getOwnPropSymbols$1) {
		for (var prop of __getOwnPropSymbols$1(b)) if (__propIsEnum$1.call(b, prop)) __defNormalProp$1(a, prop, b[prop]);
	}
	return a;
};
var __spreadProps = (a, b) => __defProps(a, __getOwnPropDescs(b));
var current;
var internalLocale = writable(null);
function getSubLocales(refLocale) {
	return refLocale.split("-").map((_, i, arr) => arr.slice(0, i + 1).join("-")).reverse();
}
function getPossibleLocales(refLocale, fallbackLocale = getOptions().fallbackLocale) {
	const locales = getSubLocales(refLocale);
	if (fallbackLocale) return [.../* @__PURE__ */ new Set([...locales, ...getSubLocales(fallbackLocale)])];
	return locales;
}
function getCurrentLocale() {
	return current != null ? current : void 0;
}
internalLocale.subscribe((newLocale) => {
	current = newLocale != null ? newLocale : void 0;
	if (typeof window !== "undefined" && newLocale != null) document.documentElement.setAttribute("lang", newLocale);
});
var set = (newLocale) => {
	if (newLocale && getClosestAvailableLocale(newLocale) && hasLocaleQueue(newLocale)) {
		const { loadingDelay } = getOptions();
		let loadingTimer;
		if (typeof window !== "undefined" && getCurrentLocale() != null && loadingDelay) loadingTimer = window.setTimeout(() => $isLoading.set(true), loadingDelay);
		else $isLoading.set(true);
		return flush(newLocale).then(() => {
			internalLocale.set(newLocale);
		}).finally(() => {
			clearTimeout(loadingTimer);
			$isLoading.set(false);
		});
	}
	return internalLocale.set(newLocale);
};
var $locale = __spreadProps(__spreadValues$1({}, internalLocale), { set });
var getLocaleFromNavigator = () => {
	if (typeof window === "undefined") return null;
	return window.navigator.language || window.navigator.languages[0];
};
var monadicMemoize = (fn) => {
	const cache = /* @__PURE__ */ Object.create(null);
	const memoizedFn = (arg) => {
		const cacheKey = JSON.stringify(arg);
		if (cacheKey in cache) return cache[cacheKey];
		return cache[cacheKey] = fn(arg);
	};
	return memoizedFn;
};
var __defProp = Object.defineProperty;
var __getOwnPropSymbols = Object.getOwnPropertySymbols;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __propIsEnum = Object.prototype.propertyIsEnumerable;
var __defNormalProp = (obj, key, value) => key in obj ? __defProp(obj, key, {
	enumerable: true,
	configurable: true,
	writable: true,
	value
}) : obj[key] = value;
var __spreadValues = (a, b) => {
	for (var prop in b || (b = {})) if (__hasOwnProp.call(b, prop)) __defNormalProp(a, prop, b[prop]);
	if (__getOwnPropSymbols) {
		for (var prop of __getOwnPropSymbols(b)) if (__propIsEnum.call(b, prop)) __defNormalProp(a, prop, b[prop]);
	}
	return a;
};
var __objRest = (source, exclude) => {
	var target = {};
	for (var prop in source) if (__hasOwnProp.call(source, prop) && exclude.indexOf(prop) < 0) target[prop] = source[prop];
	if (source != null && __getOwnPropSymbols) {
		for (var prop of __getOwnPropSymbols(source)) if (exclude.indexOf(prop) < 0 && __propIsEnum.call(source, prop)) target[prop] = source[prop];
	}
	return target;
};
var getIntlFormatterOptions = (type, name) => {
	const { formats } = getOptions();
	if (type in formats && name in formats[type]) return formats[type][name];
	throw new Error(`[svelte-i18n] Unknown "${name}" ${type} format.`);
};
var createNumberFormatter = monadicMemoize((_a) => {
	var _b = _a, { locale, format } = _b, options = __objRest(_b, ["locale", "format"]);
	if (locale == null) throw new Error("[svelte-i18n] A \"locale\" must be set to format numbers");
	if (format) options = getIntlFormatterOptions("number", format);
	return new Intl.NumberFormat(locale, options);
});
var createDateFormatter = monadicMemoize((_c) => {
	var _d = _c, { locale, format } = _d, options = __objRest(_d, ["locale", "format"]);
	if (locale == null) throw new Error("[svelte-i18n] A \"locale\" must be set to format dates");
	if (format) options = getIntlFormatterOptions("date", format);
	else if (Object.keys(options).length === 0) options = getIntlFormatterOptions("date", "short");
	return new Intl.DateTimeFormat(locale, options);
});
var createTimeFormatter = monadicMemoize((_e) => {
	var _f = _e, { locale, format } = _f, options = __objRest(_f, ["locale", "format"]);
	if (locale == null) throw new Error("[svelte-i18n] A \"locale\" must be set to format time values");
	if (format) options = getIntlFormatterOptions("time", format);
	else if (Object.keys(options).length === 0) options = getIntlFormatterOptions("time", "short");
	return new Intl.DateTimeFormat(locale, options);
});
var getNumberFormatter = (_g = {}) => {
	var _h = _g, { locale = getCurrentLocale() } = _h, args = __objRest(_h, ["locale"]);
	return createNumberFormatter(__spreadValues({ locale }, args));
};
var getDateFormatter = (_i = {}) => {
	var _j = _i, { locale = getCurrentLocale() } = _j, args = __objRest(_j, ["locale"]);
	return createDateFormatter(__spreadValues({ locale }, args));
};
var getTimeFormatter = (_k = {}) => {
	var _l = _k, { locale = getCurrentLocale() } = _l, args = __objRest(_l, ["locale"]);
	return createTimeFormatter(__spreadValues({ locale }, args));
};
var getMessageFormatter = monadicMemoize((message, locale = getCurrentLocale()) => new IntlMessageFormat(message, locale, getOptions().formats, { ignoreTag: getOptions().ignoreTag }));
var formatMessage = (id, options = {}) => {
	var _a, _b, _c, _d;
	let messageObj = options;
	if (typeof id === "object") {
		messageObj = id;
		id = messageObj.id;
	}
	const { values, locale = getCurrentLocale(), default: defaultValue } = messageObj;
	if (locale == null) throw new Error("[svelte-i18n] Cannot format a message without first setting the initial locale.");
	let message = lookup(id, locale);
	if (!message) message = (_d = (_c = (_b = (_a = getOptions()).handleMissingMessage) == null ? void 0 : _b.call(_a, {
		locale,
		id,
		defaultValue
	})) != null ? _c : defaultValue) != null ? _d : id;
	else if (typeof message !== "string") {
		console.warn(`[svelte-i18n] Message with id "${id}" must be of type "string", found: "${typeof message}". Gettin its value through the "$format" method is deprecated; use the "json" method instead.`);
		return message;
	}
	if (!values) return message;
	let result = message;
	try {
		result = getMessageFormatter(message, locale).format(values);
	} catch (e) {
		if (e instanceof Error) console.warn(`[svelte-i18n] Message "${id}" has syntax error:`, e.message);
	}
	return result;
};
var formatTime = (t, options) => {
	return getTimeFormatter(options).format(t);
};
var formatDate = (d, options) => {
	return getDateFormatter(options).format(d);
};
var formatNumber = (n, options) => {
	return getNumberFormatter(options).format(n);
};
var getJSON = (id, locale = getCurrentLocale()) => {
	return lookup(id, locale);
};
var $format = derived([$locale, $dictionary], () => formatMessage);
derived([$locale], () => formatTime);
derived([$locale], () => formatDate);
derived([$locale], () => formatNumber);
derived([$locale, $dictionary], () => getJSON);
//#endregion
export { init as i, addMessages as n, getLocaleFromNavigator as r, $format as t };
