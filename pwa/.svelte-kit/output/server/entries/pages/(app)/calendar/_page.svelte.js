import { X as escape_html, c as ensure_array_like, g as unsubscribe_stores, h as stringify, m as store_get, n as attr_class, o as derived, r as attr_style } from "../../../../chunks/dev.js";
import { i as createQuery } from "../../../../chunks/dist.js";
import { i as tasks, r as categories } from "../../../../chunks/api.js";
import { a as tsToDate, i as isToday } from "../../../../chunks/utils2.js";
//#region src/routes/(app)/calendar/+page.svelte
function _page($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		function getTaskDate(task) {
			if (task.schedule.type === "allDay") return tsToDate(task.schedule.date);
			if (task.schedule.type === "at") return tsToDate(task.schedule.startsAt);
			if (task.schedule.type === "span") return tsToDate(task.schedule.startsAt);
			return null;
		}
		function isSameDay(a, b) {
			return a.getFullYear() === b.getFullYear() && a.getMonth() === b.getMonth() && a.getDate() === b.getDate();
		}
		let view = "month";
		let current = /* @__PURE__ */ new Date();
		let selected = /* @__PURE__ */ new Date();
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
		function tasksOnDay(d) {
			return allTasks().filter((t) => {
				const td = getTaskDate(t);
				return td && isSameDay(td, d);
			});
		}
		let year = derived(() => current.getFullYear());
		let month = derived(() => current.getMonth());
		let startDow = derived(() => (new Date(year(), month(), 1).getDay() + 6) % 7);
		let daysInMonth = derived(() => new Date(year(), month() + 1, 0).getDate());
		let monthName = derived(() => current.toLocaleString("default", {
			month: "long",
			year: "numeric"
		}));
		$$renderer.push(`<div class="flex-1 min-h-0 flex flex-col overflow-hidden"><div class="flex items-center gap-2 px-4 pt-2 pb-3 border-b border-surface-800"><button class="btn btn-icon preset-tonal-surface size-8" aria-label="Previous month"><svg viewBox="0 0 12 12" width="12" height="12" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"><polyline points="8 2 4 6 8 10"></polyline></svg></button> <span class="flex-1 text-center text-sm font-semibold text-surface-100">${escape_html(monthName())}</span> <button class="btn btn-icon preset-tonal-surface size-8" aria-label="Next month"><svg viewBox="0 0 12 12" width="12" height="12" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"><polyline points="4 2 8 6 4 10"></polyline></svg></button> <button class="btn preset-tonal-surface text-xs h-8 px-3 ml-1">Today</button> <div class="flex rounded-md overflow-hidden border border-surface-700 ml-1"><!--[-->`);
		const each_array = ensure_array_like(["month", "agenda"]);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let v = each_array[$$index];
			$$renderer.push(`<button${attr_class(`px-3 h-8 text-xs font-medium transition-colors ${view === v ? "bg-primary-500 text-white" : "bg-surface-900 text-surface-400 hover:text-surface-200"}`)}>${escape_html(v === "month" ? "M" : "A")}</button>`);
		}
		$$renderer.push(`<!--]--></div></div> <div class="flex-1 overflow-y-auto pb-32">`);
		{
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<div class="grid grid-cols-7 px-2 pt-2"><!--[-->`);
			const each_array_1 = ensure_array_like([
				"Mo",
				"Tu",
				"We",
				"Th",
				"Fr",
				"Sa",
				"Su"
			]);
			for (let $$index_1 = 0, $$length = each_array_1.length; $$index_1 < $$length; $$index_1++) {
				let d = each_array_1[$$index_1];
				$$renderer.push(`<div class="text-center text-[10px] font-mono text-surface-500 uppercase tracking-wider py-1">${escape_html(d)}</div>`);
			}
			$$renderer.push(`<!--]--></div> <div class="grid grid-cols-7 px-2 gap-px"><!--[-->`);
			const each_array_2 = ensure_array_like(Array.from({ length: startDow() }));
			for (let i = 0, $$length = each_array_2.length; i < $$length; i++) {
				each_array_2[i];
				$$renderer.push(`<div></div>`);
			}
			$$renderer.push(`<!--]--> <!--[-->`);
			const each_array_3 = ensure_array_like(Array.from({ length: daysInMonth() }));
			for (let i = 0, $$length = each_array_3.length; i < $$length; i++) {
				each_array_3[i];
				const d = new Date(year(), month(), i + 1);
				const dayTasks = tasksOnDay(d);
				const todayCell = isToday(Math.floor(d.getTime() / 1e3));
				const sel = isSameDay(d, selected);
				$$renderer.push(`<div${attr_class(`min-h-[52px] p-1 rounded-md cursor-pointer transition-colors ${sel ? "bg-primary-500/20 ring-1 ring-primary-500" : "hover:bg-surface-800"}`)} role="button" tabindex="0"><span${attr_class(`text-xs font-medium block text-center w-5 h-5 rounded-full mx-auto leading-5 ${todayCell ? "bg-primary-500 text-white" : "text-surface-300"}`)}>${escape_html(i + 1)}</span> <div class="mt-0.5 flex flex-col gap-0.5"><!--[-->`);
				const each_array_4 = ensure_array_like(dayTasks.slice(0, 2));
				for (let $$index_3 = 0, $$length = each_array_4.length; $$index_3 < $$length; $$index_3++) {
					let t = each_array_4[$$index_3];
					const cat = allCats().find((c) => c.id === t.categoryId);
					$$renderer.push(`<div class="text-[9px] px-1 rounded truncate text-white"${attr_style(`background: ${stringify(cat?.color ?? "#46a758")}`)}>${escape_html(t.title)}</div>`);
				}
				$$renderer.push(`<!--]--> `);
				if (dayTasks.length > 2) {
					$$renderer.push("<!--[0-->");
					$$renderer.push(`<div class="text-[9px] text-surface-500 text-center">+${escape_html(dayTasks.length - 2)}</div>`);
				} else $$renderer.push("<!--[-1-->");
				$$renderer.push(`<!--]--></div></div>`);
			}
			$$renderer.push(`<!--]--></div> <div class="px-4 pt-4"><p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-2">${escape_html(selected.toLocaleString("default", {
				weekday: "long",
				day: "numeric",
				month: "long"
			}))}</p> `);
			if (tasksOnDay(selected).length === 0) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<p class="text-sm text-surface-500 py-4 text-center">No tasks</p>`);
			} else {
				$$renderer.push("<!--[-1-->");
				$$renderer.push(`<!--[-->`);
				const each_array_5 = ensure_array_like(tasksOnDay(selected));
				for (let $$index_5 = 0, $$length = each_array_5.length; $$index_5 < $$length; $$index_5++) {
					let t = each_array_5[$$index_5];
					const cat = allCats().find((c) => c.id === t.categoryId);
					$$renderer.push(`<div${attr_class(`flex items-center gap-3 p-3 mb-2 rounded-r-md border border-surface-700 bg-surface-900 ${t.completedAt ? "opacity-50" : ""}`)}${attr_style(`border-left: 4px solid ${stringify(cat?.color ?? "#46a758")}`)}><span${attr_class(`text-sm ${t.completedAt ? "line-through text-surface-500" : "text-surface-100"}`)}>${escape_html(t.title)}</span></div>`);
				}
				$$renderer.push(`<!--]-->`);
			}
			$$renderer.push(`<!--]--></div>`);
		}
		$$renderer.push(`<!--]--> `);
		$$renderer.push("<!--[-1-->");
		$$renderer.push(`<!--]--></div></div>`);
		if ($$store_subs) unsubscribe_stores($$store_subs);
	});
}
//#endregion
export { _page as default };
