import { J as attr, X as escape_html, Y as clsx, c as ensure_array_like, d as sanitize_props, f as slot, g as unsubscribe_stores, h as stringify, m as store_get, n as attr_class, o as derived, p as spread_props, r as attr_style } from "../../../chunks/dev.js";
import { a as useQueryClient, i as createQuery, r as createMutation } from "../../../chunks/dist.js";
import { t as $format } from "../../../chunks/runtime.js";
import { i as tasks, r as categories } from "../../../chunks/api.js";
import { t as Icon } from "../../../chunks/Icon.js";
import { a as tsToDate, i as isToday, r as isOverdue } from "../../../chunks/utils2.js";
//#region node_modules/lucide-svelte/dist/icons/chevron-down.svelte
function Chevron_down($$renderer, $$props) {
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
		{ name: "chevron-down" },
		sanitize_props($$props),
		{
			/**
			* @component @name ChevronDown
			* @description Lucide SVG icon component, renders SVG Element with children.
			*
			* @preview ![img](data:image/svg+xml;base64,PHN2ZyAgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIgogIHdpZHRoPSIyNCIKICBoZWlnaHQ9IjI0IgogIHZpZXdCb3g9IjAgMCAyNCAyNCIKICBmaWxsPSJub25lIgogIHN0cm9rZT0iIzAwMCIgc3R5bGU9ImJhY2tncm91bmQtY29sb3I6ICNmZmY7IGJvcmRlci1yYWRpdXM6IDJweCIKICBzdHJva2Utd2lkdGg9IjIiCiAgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIgogIHN0cm9rZS1saW5lam9pbj0icm91bmQiCj4KICA8cGF0aCBkPSJtNiA5IDYgNiA2LTYiIC8+Cjwvc3ZnPgo=) - https://lucide.dev/icons/chevron-down
			* @see https://lucide.dev/guide/packages/lucide-svelte - Documentation
			*
			* @param {Object} props - Lucide icons props and any valid SVG attribute
			* @returns {FunctionalComponent} Svelte component
			*
			*/
			iconNode: [["path", { "d": "m6 9 6 6 6-6" }]],
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
//#region node_modules/lucide-svelte/dist/icons/eye-closed.svelte
function Eye_closed($$renderer, $$props) {
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
		{ name: "eye-closed" },
		sanitize_props($$props),
		{
			/**
			* @component @name EyeClosed
			* @description Lucide SVG icon component, renders SVG Element with children.
			*
			* @preview ![img](data:image/svg+xml;base64,PHN2ZyAgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIgogIHdpZHRoPSIyNCIKICBoZWlnaHQ9IjI0IgogIHZpZXdCb3g9IjAgMCAyNCAyNCIKICBmaWxsPSJub25lIgogIHN0cm9rZT0iIzAwMCIgc3R5bGU9ImJhY2tncm91bmQtY29sb3I6ICNmZmY7IGJvcmRlci1yYWRpdXM6IDJweCIKICBzdHJva2Utd2lkdGg9IjIiCiAgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIgogIHN0cm9rZS1saW5lam9pbj0icm91bmQiCj4KICA8cGF0aCBkPSJtMTUgMTgtLjcyMi0zLjI1IiAvPgogIDxwYXRoIGQ9Ik0yIDhhMTAuNjQ1IDEwLjY0NSAwIDAgMCAyMCAwIiAvPgogIDxwYXRoIGQ9Im0yMCAxNS0xLjcyNi0yLjA1IiAvPgogIDxwYXRoIGQ9Im00IDE1IDEuNzI2LTIuMDUiIC8+CiAgPHBhdGggZD0ibTkgMTggLjcyMi0zLjI1IiAvPgo8L3N2Zz4K) - https://lucide.dev/icons/eye-closed
			* @see https://lucide.dev/guide/packages/lucide-svelte - Documentation
			*
			* @param {Object} props - Lucide icons props and any valid SVG attribute
			* @returns {FunctionalComponent} Svelte component
			*
			*/
			iconNode: [
				["path", { "d": "m15 18-.722-3.25" }],
				["path", { "d": "M2 8a10.645 10.645 0 0 0 20 0" }],
				["path", { "d": "m20 15-1.726-2.05" }],
				["path", { "d": "m4 15 1.726-2.05" }],
				["path", { "d": "m9 18 .722-3.25" }]
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
//#region node_modules/lucide-svelte/dist/icons/funnel.svelte
function Funnel($$renderer, $$props) {
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
		{ name: "funnel" },
		sanitize_props($$props),
		{
			/**
			* @component @name Funnel
			* @description Lucide SVG icon component, renders SVG Element with children.
			*
			* @preview ![img](data:image/svg+xml;base64,PHN2ZyAgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIgogIHdpZHRoPSIyNCIKICBoZWlnaHQ9IjI0IgogIHZpZXdCb3g9IjAgMCAyNCAyNCIKICBmaWxsPSJub25lIgogIHN0cm9rZT0iIzAwMCIgc3R5bGU9ImJhY2tncm91bmQtY29sb3I6ICNmZmY7IGJvcmRlci1yYWRpdXM6IDJweCIKICBzdHJva2Utd2lkdGg9IjIiCiAgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIgogIHN0cm9rZS1saW5lam9pbj0icm91bmQiCj4KICA8cGF0aCBkPSJNMTAgMjBhMSAxIDAgMCAwIC41NTMuODk1bDIgMUExIDEgMCAwIDAgMTQgMjF2LTdhMiAyIDAgMCAxIC41MTctMS4zNDFMMjEuNzQgNC42N0ExIDEgMCAwIDAgMjEgM0gzYTEgMSAwIDAgMC0uNzQyIDEuNjdsNy4yMjUgNy45ODlBMiAyIDAgMCAxIDEwIDE0eiIgLz4KPC9zdmc+Cg==) - https://lucide.dev/icons/funnel
			* @see https://lucide.dev/guide/packages/lucide-svelte - Documentation
			*
			* @param {Object} props - Lucide icons props and any valid SVG attribute
			* @returns {FunctionalComponent} Svelte component
			*
			*/
			iconNode: [["path", { "d": "M10 20a1 1 0 0 0 .553.895l2 1A1 1 0 0 0 14 21v-7a2 2 0 0 1 .517-1.341L21.74 4.67A1 1 0 0 0 21 3H3a1 1 0 0 0-.742 1.67l7.225 7.989A2 2 0 0 1 10 14z" }]],
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
//#region src/components/tasks/TaskRow.svelte
function TaskRow($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const { task, categories: cats } = $$props;
		const PRIORITY_BORDER = {
			low: "#46a758",
			medium: "#d97706",
			high: "#ef4444",
			urgent: "#7c3aed"
		};
		const DATE_OPTS = {
			month: "short",
			day: "numeric"
		};
		const TIME_OPTS = {
			hour: "2-digit",
			minute: "2-digit"
		};
		function scheduleStartTs(schedule) {
			if (schedule.type === "allDay") return schedule.date;
			if (schedule.type === "at") return schedule.startsAt;
			if (schedule.type === "span") return schedule.startsAt;
		}
		function scheduleLabel(schedule, overdue, today) {
			if (schedule.type === "allDay") {
				if (overdue) return "Overdue";
				if (today) return "Today";
				return tsToDate(schedule.date).toLocaleDateString(void 0, DATE_OPTS);
			}
			if (schedule.type === "at") {
				const t = tsToDate(schedule.startsAt).toLocaleTimeString(void 0, TIME_OPTS);
				if (overdue) return `Overdue · ${t}`;
				if (today) return `Today · ${t}`;
				return `${tsToDate(schedule.startsAt).toLocaleDateString(void 0, DATE_OPTS)} · ${t}`;
			}
			if (schedule.type === "span") {
				const start = tsToDate(schedule.startsAt);
				const end = tsToDate(schedule.startsAt + schedule.duration);
				const range = `${start.toLocaleTimeString(void 0, TIME_OPTS)}–${end.toLocaleTimeString(void 0, TIME_OPTS)}`;
				if (overdue) return `Overdue · ${range}`;
				if (today) return `Today · ${range}`;
				return `${start.toLocaleDateString(void 0, DATE_OPTS)} · ${range}`;
			}
			return "";
		}
		const qc = useQueryClient();
		let cat = derived(() => cats.find((c) => c.id === task.categoryId));
		let startTs = derived(() => scheduleStartTs(task.schedule));
		let today = derived(() => startTs() ? isToday(startTs()) : false);
		let overdue = derived(() => startTs() && !task.completedAt ? isOverdue(startTs()) : false);
		let completedSubtasks = derived(() => task.subtasks.filter((s) => s.isCompleted).length);
		let totalSubtasks = derived(() => task.subtasks.length);
		createMutation({
			mutationFn: () => tasks.update(task.id, {
				completed: !task.completedAt,
				title: null,
				description: null,
				schedule: null,
				priority: null
			}),
			onSuccess: () => qc.invalidateQueries({ queryKey: ["tasks"] })
		});
		$$renderer.push(`<div${attr_class(`border-b border-surface-800 ${task.completedAt ? "opacity-50" : ""}`)}${attr_style(`border-left: 3px solid ${stringify(task.priority ? PRIORITY_BORDER[task.priority] : "transparent")}`)}><div class="flex items-start gap-3 px-4 py-3"><button${attr("aria-label", task.completedAt ? "Mark incomplete" : "Mark complete")}${attr_class(clsx(["mt-0.5 shrink-0 size-5 rounded-full border-2 grid place-items-center transition-colors", task.completedAt ? "bg-primary-500 border-primary-500" : "border-surface-600 hover:border-primary-400"].join(" ")))}>`);
		if (task.completedAt) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<svg viewBox="0 0 11 11" width="10" height="10" fill="none" stroke="white" stroke-width="1.8" stroke-linecap="round"><polyline points="1.5 5.5 4.5 8.5 9.5 2.5"></polyline></svg>`);
		} else $$renderer.push("<!--[-1-->");
		$$renderer.push(`<!--]--></button> <div class="flex-1 min-w-0"><p${attr_class(`text-sm font-medium leading-snug ${task.completedAt ? "line-through text-surface-500" : "text-surface-50"}`)}>${escape_html(task.title)}</p> `);
		if (task.description) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<p class="text-xs text-surface-500 mt-0.5 leading-snug">${escape_html(task.description)}</p>`);
		} else $$renderer.push("<!--[-1-->");
		$$renderer.push(`<!--]--> <div class="flex flex-wrap items-center gap-1.5 mt-1.5">`);
		if (cat()) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<span class="chip text-[10px]"${attr_style(`background-color: ${stringify(cat().color)}; color: #fff`)}>${escape_html(cat().name)}</span>`);
		} else $$renderer.push("<!--[-1-->");
		$$renderer.push(`<!--]--> `);
		if (task.schedule.type !== "unscheduled") {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<span${attr_class(`chip text-[10px] ${overdue() ? "preset-tonal-error" : today() ? "preset-tonal-warning" : "preset-tonal-surface"}`)}>${escape_html(scheduleLabel(task.schedule, !!overdue(), today()))}</span>`);
		} else $$renderer.push("<!--[-1-->");
		$$renderer.push(`<!--]--> `);
		if (totalSubtasks() > 0) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<span class="text-[10px] text-surface-500 font-mono">${escape_html(completedSubtasks())}/${escape_html(totalSubtasks())}</span>`);
		} else $$renderer.push("<!--[-1-->");
		$$renderer.push(`<!--]--></div></div> <div class="flex items-center gap-1 shrink-0 mt-0.5">`);
		if (totalSubtasks() > 0) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<button class="btn btn-icon preset-tonal-surface size-7" aria-label="Expand subtasks">`);
			$$renderer.push("<!--[-1-->");
			Chevron_down($$renderer, { class: "size-4" });
			$$renderer.push(`<!--]--></button>`);
		} else $$renderer.push("<!--[-1-->");
		$$renderer.push(`<!--]--></div></div> `);
		$$renderer.push("<!--[-1-->");
		$$renderer.push(`<!--]--></div>`);
		if ($$store_subs) unsubscribe_stores($$store_subs);
	});
}
//#endregion
//#region src/components/tasks/CreateTaskSheet.svelte
function CreateTaskSheet($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const { open, onClose } = $$props;
		const PRIORITY_COLORS = {
			low: "#46a758",
			medium: "#d97706",
			high: "#ef4444",
			urgent: "#7c3aed"
		};
		const qc = useQueryClient();
		let title = "";
		let description = "";
		let priority = null;
		let categoryId = "";
		let scheduleType = "Unscheduled";
		let scheduleDate = "";
		let scheduleTime = "";
		let duration = 25;
		let subtasks = [];
		let subtaskInput = "";
		const catsQuery = createQuery({
			queryKey: ["categories"],
			queryFn: categories.getAll
		});
		let cats = derived(() => store_get($$store_subs ??= {}, "$catsQuery", catsQuery).data?.categories ?? []);
		function buildSchedule() {
			if (scheduleType === "Unscheduled") return { type: "unscheduled" };
			if (scheduleType === "AllDay") return {
				type: "allDay",
				date: scheduleDate ? Math.floor(new Date(scheduleDate).getTime() / 1e3) : Math.floor(Date.now() / 1e3)
			};
			const ts = (/* @__PURE__ */ new Date(`${scheduleDate}T${scheduleTime || "00:00"}`)).getTime();
			if (scheduleType === "At") return {
				type: "at",
				startsAt: Math.floor(ts / 1e3)
			};
			return {
				type: "span",
				startsAt: Math.floor(ts / 1e3),
				duration: duration * 60
			};
		}
		const create = createMutation({
			mutationFn: () => {
				const dto = {
					title: title.trim(),
					description: description.trim() || null,
					priority,
					categoryId: categoryId || null,
					schedule: buildSchedule(),
					subtasks: subtasks.map((t) => ({
						title: t,
						description: null
					}))
				};
				return tasks.create(dto);
			},
			onSuccess: () => {
				qc.invalidateQueries({ queryKey: ["tasks"] });
				handleClose();
			}
		});
		function handleClose() {
			title = "";
			description = "";
			priority = null;
			categoryId = "";
			scheduleType = "Unscheduled";
			scheduleDate = "";
			scheduleTime = "";
			duration = 25;
			subtasks = [];
			subtaskInput = "";
			onClose();
		}
		if (open) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<div class="fixed inset-0 z-30 bg-black/60" role="presentation"></div> <div class="fixed bottom-0 left-0 right-0 z-40 bg-surface-900 rounded-t-2xl border-t border-surface-700 flex flex-col max-h-[90vh]"><div class="flex justify-center pt-3 pb-1 shrink-0"><div class="w-10 h-1 rounded-full bg-surface-600"></div></div> <div class="flex items-center justify-between px-4 pb-3 shrink-0"><p class="text-sm font-semibold text-surface-100">New Task</p> <button class="btn btn-icon preset-tonal-surface size-7" aria-label="Close"><svg viewBox="0 0 14 14" width="12" height="12" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><line x1="2" y1="2" x2="12" y2="12"></line><line x1="12" y1="2" x2="2" y2="12"></line></svg></button></div> <div class="overflow-y-auto flex-1 px-4 pb-4 flex flex-col gap-4"><div><label for="task-title" class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-1 block">Title *</label> <input id="task-title" class="input w-full text-sm" placeholder="Task title"${attr("value", title)}/></div> <div><label for="task-desc" class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-1 block">Description</label> <textarea id="task-desc" class="textarea w-full text-sm resize-none" placeholder="Optional description"${attr("rows", 2)}>`);
			const $$body = escape_html(description);
			if ($$body) $$renderer.push(`${$$body}`);
			$$renderer.push(`</textarea></div> <div><p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-2">Priority</p> <div class="flex gap-2"><!--[-->`);
			const each_array = ensure_array_like([
				"low",
				"medium",
				"high",
				"urgent"
			]);
			for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
				let p = each_array[$$index];
				$$renderer.push(`<button class="chip text-[11px] capitalize flex-1"${attr_style(`background-color: ${stringify(priority === p ? PRIORITY_COLORS[p] : "transparent")}; color: ${stringify(priority === p ? "#fff" : PRIORITY_COLORS[p])}; border: 1px solid ${stringify(PRIORITY_COLORS[p])}`)}>${escape_html(p)}</button>`);
			}
			$$renderer.push(`<!--]--></div></div> `);
			if (cats().length > 0) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<div><label for="task-cat" class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-1 block">Category</label> `);
				$$renderer.select({
					id: "task-cat",
					class: "select w-full text-sm",
					value: categoryId
				}, ($$renderer) => {
					$$renderer.option({ value: "" }, ($$renderer) => {
						$$renderer.push(`None`);
					});
					$$renderer.push(`<!--[-->`);
					const each_array_1 = ensure_array_like(cats());
					for (let $$index_1 = 0, $$length = each_array_1.length; $$index_1 < $$length; $$index_1++) {
						let c = each_array_1[$$index_1];
						$$renderer.option({ value: c.id }, ($$renderer) => {
							$$renderer.push(`${escape_html(c.name)}`);
						});
					}
					$$renderer.push(`<!--]-->`);
				});
				$$renderer.push(`</div>`);
			} else $$renderer.push("<!--[-1-->");
			$$renderer.push(`<!--]--> <div><p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-2">Schedule</p> <div class="flex gap-1.5 mb-3 flex-wrap"><!--[-->`);
			const each_array_2 = ensure_array_like([
				"Unscheduled",
				"AllDay",
				"At",
				"Span"
			]);
			for (let $$index_2 = 0, $$length = each_array_2.length; $$index_2 < $$length; $$index_2++) {
				let t = each_array_2[$$index_2];
				$$renderer.push(`<button${attr_class(`chip text-[11px] ${scheduleType === t ? "preset-filled-primary-500" : "preset-tonal-surface"}`)}>${escape_html(t === "Unscheduled" ? "None" : t === "AllDay" ? "All Day" : t)}</button>`);
			}
			$$renderer.push(`<!--]--></div> `);
			if (scheduleType === "AllDay") {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<input type="date" class="input w-full text-sm"${attr("value", scheduleDate)}/>`);
			} else $$renderer.push("<!--[-1-->");
			$$renderer.push(`<!--]--> `);
			if (scheduleType === "At" || scheduleType === "Span") {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<div class="flex gap-2"><input type="date" class="input flex-1 text-sm"${attr("value", scheduleDate)}/> <input type="time" class="input flex-1 text-sm"${attr("value", scheduleTime)}/></div>`);
			} else $$renderer.push("<!--[-1-->");
			$$renderer.push(`<!--]--> `);
			if (scheduleType === "Span") {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<div class="flex items-center gap-2 mt-2"><label for="task-dur" class="text-xs text-surface-500 shrink-0">Duration (min)</label> <input id="task-dur" type="number" class="input flex-1 text-sm"${attr("min", 1)}${attr("value", duration)}/></div>`);
			} else $$renderer.push("<!--[-1-->");
			$$renderer.push(`<!--]--></div> <div><p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-2">Subtasks</p> <!--[-->`);
			const each_array_3 = ensure_array_like(subtasks);
			for (let i = 0, $$length = each_array_3.length; i < $$length; i++) {
				let s = each_array_3[i];
				$$renderer.push(`<div class="flex items-center gap-2 mb-1.5"><span class="text-xs text-surface-300 flex-1 truncate">${escape_html(s)}</span> <button class="btn btn-icon preset-tonal-error size-6 shrink-0" aria-label="Remove subtask"><svg viewBox="0 0 12 12" width="10" height="10" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"><line x1="2" y1="2" x2="10" y2="10"></line><line x1="10" y1="2" x2="2" y2="10"></line></svg></button></div>`);
			}
			$$renderer.push(`<!--]--> <div class="flex gap-2"><input class="input flex-1 text-sm" placeholder="Add subtask"${attr("value", subtaskInput)}/> <button${attr("disabled", !subtaskInput.trim(), true)} class="btn preset-tonal-surface text-sm px-3 disabled:opacity-50">Add</button></div></div></div> <div class="px-4 py-3 border-t border-surface-800 shrink-0"><button${attr("disabled", !title.trim() || store_get($$store_subs ??= {}, "$create", create).isPending, true)} class="btn preset-filled-primary-500 w-full disabled:opacity-50">${escape_html(store_get($$store_subs ??= {}, "$create", create).isPending ? "Creating…" : "Create Task")}</button></div></div>`);
		} else $$renderer.push("<!--[-1-->");
		$$renderer.push(`<!--]-->`);
		if ($$store_subs) unsubscribe_stores($$store_subs);
	});
}
//#endregion
//#region src/routes/(app)/+page.svelte
function _page($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const PRIORITY_LEGEND = [
			{
				label: "Low",
				color: "#46a758"
			},
			{
				label: "Medium",
				color: "#d97706"
			},
			{
				label: "High",
				color: "#ef4444"
			},
			{
				label: "Urgent",
				color: "#7c3aed"
			}
		];
		let sheetOpen = false;
		let showCompleted = false;
		const tasksQuery = createQuery({
			queryKey: ["tasks"],
			queryFn: tasks.getAll
		});
		const catsQuery = createQuery({
			queryKey: ["categories"],
			queryFn: categories.getAll
		});
		let allTasks = derived(() => store_get($$store_subs ??= {}, "$tasksQuery", tasksQuery).data?.tasks ?? []);
		let allCats = derived(() => store_get($$store_subs ??= {}, "$catsQuery", catsQuery).data?.categories ?? []);
		derived(() => [...allCats().map((c) => c.id), "none"]);
		let filteredTasks = derived(() => allTasks());
		let active = derived(() => filteredTasks().filter((t) => !t.completedAt));
		let done = derived(() => filteredTasks().filter((t) => !!t.completedAt));
		let filterActive = derived(() => false);
		$$renderer.push(`<div class="flex-1 min-h-0 flex flex-col overflow-hidden"><div class="flex-1 overflow-y-auto pb-32">`);
		if (store_get($$store_subs ??= {}, "$tasksQuery", tasksQuery).isPending) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<div class="flex items-center justify-center py-16 text-surface-500 text-sm font-mono"><span>${escape_html(store_get($$store_subs ??= {}, "$_", $format)("common.loading"))}</span></div>`);
		} else $$renderer.push("<!--[-1-->");
		$$renderer.push(`<!--]--> `);
		$$renderer.push("<!--[-1-->");
		$$renderer.push(`<div class="flex flex-row justify-end px-2 pt-2 mb-1"><button type="button" class="btn-icon preset-outlined relative mr-3" aria-label="Apri filtri">`);
		$$renderer.push("<!--[-1-->");
		Eye_closed($$renderer, { size: 18 });
		$$renderer.push(`<!--]--></button> <button type="button" class="btn-icon preset-outlined relative" aria-label="Apri filtri">`);
		Funnel($$renderer, { size: 18 });
		$$renderer.push(`<!----> `);
		if (filterActive()) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<span class="absolute -top-0.5 -right-0.5 size-2 rounded-full bg-primary-500"></span>`);
		} else $$renderer.push("<!--[-1-->");
		$$renderer.push(`<!--]--></button></div>`);
		$$renderer.push(`<!--]--> <!--[-->`);
		const each_array_1 = ensure_array_like(active());
		for (let $$index_1 = 0, $$length = each_array_1.length; $$index_1 < $$length; $$index_1++) {
			let task = each_array_1[$$index_1];
			TaskRow($$renderer, {
				task,
				categories: allCats()
			});
		}
		$$renderer.push(`<!--]--> `);
		if (done().length > 0 && showCompleted);
		else $$renderer.push("<!--[-1-->");
		$$renderer.push(`<!--]--> `);
		if (!store_get($$store_subs ??= {}, "$tasksQuery", tasksQuery).isPending && allTasks().length === 0) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<div class="flex flex-col items-center justify-center py-20 gap-3 text-surface-500"><svg viewBox="0 0 20 20" width="28" height="28" stroke="currentColor" fill="none" stroke-width="1.5"><path d="M3 6h14M3 10h10M3 14h7" stroke-linecap="round"></path></svg> <p class="text-sm">${escape_html(store_get($$store_subs ??= {}, "$_", $format)("todo.no_tasks"))}</p></div>`);
		} else $$renderer.push("<!--[-1-->");
		$$renderer.push(`<!--]--> `);
		if (!store_get($$store_subs ??= {}, "$tasksQuery", tasksQuery).isPending && allTasks().length > 0) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<div class="mt-4 px-4 pb-3"><p class="text-[10px] font-mono text-surface-600 mb-2">Priority legend</p> <div class="flex gap-3"><!--[-->`);
			const each_array_3 = ensure_array_like(PRIORITY_LEGEND);
			for (let $$index_3 = 0, $$length = each_array_3.length; $$index_3 < $$length; $$index_3++) {
				let { label, color } = each_array_3[$$index_3];
				$$renderer.push(`<div class="flex items-center gap-1"><div class="rounded-sm shrink-0"${attr_style(`background-color: ${stringify(color)}; width: 3px; height: 12px`)}></div> <span class="text-[10px] text-surface-500">${escape_html(label)}</span></div>`);
			}
			$$renderer.push(`<!--]--></div></div>`);
		} else $$renderer.push("<!--[-1-->");
		$$renderer.push(`<!--]--></div> <button class="fixed bottom-[84px] right-4 z-20 size-12 rounded-full btn preset-filled-primary-500 shadow-lg" aria-label="New task"><svg viewBox="0 0 16 16" width="20" height="20" stroke="currentColor" fill="none" stroke-width="2" stroke-linecap="round"><line x1="8" y1="2" x2="8" y2="14"></line><line x1="2" y1="8" x2="14" y2="8"></line></svg></button> `);
		CreateTaskSheet($$renderer, {
			open: sheetOpen,
			onClose: () => sheetOpen = false
		});
		$$renderer.push(`<!----></div>`);
		if ($$store_subs) unsubscribe_stores($$store_subs);
	});
}
//#endregion
export { _page as default };
