import { X as escape_html, c as ensure_array_like, g as unsubscribe_stores, h as stringify, m as store_get, n as attr_class, r as attr_style } from "../../../../chunks/dev.js";
import { i as createQuery } from "../../../../chunks/dist.js";
import { n as statsApi } from "../../../../chunks/api.js";
import { t as fmtDuration } from "../../../../chunks/utils2.js";
//#region src/routes/(app)/stats/+page.svelte
function _page($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const DAYS = [
			"Mo",
			"Tu",
			"We",
			"Th",
			"Fr",
			"Sa",
			"Su"
		];
		const PRIORITY_COLORS = {
			urgent: "#7c3aed",
			high: "#ef4444",
			medium: "#d97706",
			low: "#6b7280"
		};
		const statsQuery = createQuery({
			queryKey: ["stats"],
			queryFn: statsApi.get
		});
		let today = (/* @__PURE__ */ new Date()).toISOString().slice(0, 10);
		if (store_get($$store_subs ??= {}, "$statsQuery", statsQuery).isPending || !store_get($$store_subs ??= {}, "$statsQuery", statsQuery).data) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<div class="flex-1 flex items-center justify-center text-surface-500 text-sm font-mono">Loading…</div>`);
		} else {
			$$renderer.push("<!--[-1-->");
			const data = store_get($$store_subs ??= {}, "$statsQuery", statsQuery).data;
			const cts = data.completedTasksCounts;
			const last14d = data.last14d;
			const completedFocusSessions = data.completedFocusSessions;
			const overdueTrend = data.overdueTrend;
			const peakWindow = data.peakWindow;
			const completedByPriority = data.completedByPriority;
			const max14d = Math.max(...last14d.map((d) => d.count), 1);
			const totalByPriority = Object.values(completedByPriority).reduce((a, b) => a + b, 0) || 1;
			$$renderer.push(`<div class="flex-1 min-h-0 flex flex-col overflow-hidden"><div class="flex-1 overflow-y-auto pb-32 px-4 pt-3 flex flex-col gap-3"><div class="card bg-surface-900 border border-surface-700 p-4"><div class="flex items-start justify-between mb-1"><span class="text-xs font-mono text-surface-500 uppercase tracking-widest">Tasks today</span> `);
			if (cts.weekDelta !== 0) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<span${attr_class(`text-[10px] font-mono ${cts.weekDelta > 0 ? "text-green-400" : "text-red-400"}`)}>${escape_html(cts.weekDelta > 0 ? "+" : "")}${escape_html(cts.weekDelta)}% vs last wk</span>`);
			} else $$renderer.push("<!--[-1-->");
			$$renderer.push(`<!--]--></div> <div class="flex items-baseline gap-1.5 mt-1"><span class="text-4xl font-bold text-surface-50">${escape_html(cts.completedToday)}</span> <span class="text-sm text-surface-500">done</span></div> <p class="text-xs text-surface-500 mt-1">Daily avg: ${escape_html(cts.dayAvg.toFixed(1))} · Month: ${escape_html(cts.completedThisMonth)}</p></div> <div class="card bg-surface-900 border border-surface-700 p-4"><p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-3">Last 14 days</p> <div class="flex items-end gap-1 h-[80px]"><!--[-->`);
			const each_array = ensure_array_like(last14d);
			for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
				let d = each_array[$$index];
				const isTodayBar = d.day === today;
				const h = Math.max(2, d.count / max14d * 72);
				$$renderer.push(`<div class="flex-1 flex flex-col items-center justify-end gap-1"><div${attr_class(`w-full rounded-sm transition-all ${isTodayBar ? "bg-primary-500" : d.count === 0 ? "bg-surface-800" : "bg-primary-500/40"}`)}${attr_style(`height: ${stringify(h)}px`)}></div></div>`);
			}
			$$renderer.push(`<!--]--></div> <div class="flex gap-1 mt-1"><!--[-->`);
			const each_array_1 = ensure_array_like(last14d);
			for (let $$index_1 = 0, $$length = each_array_1.length; $$index_1 < $$length; $$index_1++) {
				let d = each_array_1[$$index_1];
				const isTodayBar = d.day === today;
				const dow = ((/* @__PURE__ */ new Date(d.day + "T00:00:00")).getDay() + 6) % 7;
				$$renderer.push(`<div class="flex-1 text-center"><span${attr_class(`text-[8px] font-mono ${isTodayBar ? "text-primary-400" : "text-surface-600"}`)}>${escape_html(DAYS[dow])}</span></div>`);
			}
			$$renderer.push(`<!--]--></div></div> <div class="grid grid-cols-2 gap-3"><div class="card bg-surface-900 border border-surface-700 p-4"><p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-2">Sessions</p> <span class="text-3xl font-bold text-surface-50">${escape_html(completedFocusSessions.count)}</span> <p class="text-xs text-surface-500 mt-1">focus sessions</p></div> <div class="card bg-surface-900 border border-surface-700 p-4"><p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-2">Avg duration</p> <span class="text-3xl font-bold text-surface-50">${escape_html(fmtDuration(completedFocusSessions.avgDurationSecs))}</span> <p class="text-xs text-surface-500 mt-1">per session</p></div></div> <div class="card bg-surface-900 border border-surface-700 p-4"><p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-3">By priority</p> <div class="flex flex-col gap-2"><!--[-->`);
			const each_array_2 = ensure_array_like([
				"urgent",
				"high",
				"medium",
				"low"
			]);
			for (let $$index_2 = 0, $$length = each_array_2.length; $$index_2 < $$length; $$index_2++) {
				let p = each_array_2[$$index_2];
				const count = completedByPriority[p];
				const pct = Math.round(count / totalByPriority * 100);
				$$renderer.push(`<div class="flex items-center gap-2"><span class="text-xs text-surface-400 w-14 capitalize">${escape_html(p)}</span> <div class="flex-1 h-1.5 bg-surface-800 rounded-full overflow-hidden"><div class="h-full rounded-full"${attr_style(`width: ${stringify(pct)}%; background: ${stringify(PRIORITY_COLORS[p])}`)}></div></div> <span class="text-xs font-mono text-surface-500 w-8 text-right">${escape_html(pct)}%</span></div>`);
			}
			$$renderer.push(`<!--]--></div></div> `);
			if (peakWindow.length > 0) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<div class="card bg-surface-900 border border-surface-700 p-4"><p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-3">Peak hours</p> <div class="flex flex-col gap-2"><!--[-->`);
				const each_array_3 = ensure_array_like(peakWindow.slice(0, 5));
				for (let i = 0, $$length = each_array_3.length; i < $$length; i++) {
					let w = each_array_3[i];
					const maxCount = Math.max(...peakWindow.map((x) => x.count), 1);
					const pct = w.count / maxCount * 100;
					$$renderer.push(`<div class="flex items-center gap-2"><span class="text-xs font-mono text-surface-400 w-12">${escape_html(w.start.slice(0, 5))}</span> <div class="flex-1 h-1.5 bg-surface-800 rounded-full overflow-hidden"><div${attr_class(`h-full rounded-full ${i === 0 ? "bg-primary-500" : "bg-primary-500/40"}`)}${attr_style(`width: ${stringify(pct)}%`)}></div></div> <span class="text-xs font-mono text-surface-500 w-4 text-right">${escape_html(w.count)}</span></div>`);
				}
				$$renderer.push(`<!--]--></div></div>`);
			} else $$renderer.push("<!--[-1-->");
			$$renderer.push(`<!--]--> <div class="card bg-surface-900 border border-surface-700 p-4"><p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-3">Overdue trend</p> <div class="flex items-center gap-3"><span${attr_class(`text-2xl font-bold ${overdueTrend.trendType === "DECREASING" ? "text-green-400" : overdueTrend.trendType === "INCREASING" ? "text-red-400" : "text-surface-400"}`)}>${escape_html(overdueTrend.trendType === "DECREASING" ? "↓" : overdueTrend.trendType === "INCREASING" ? "↑" : "→")}</span> <div><p class="text-sm text-surface-200 capitalize">${escape_html(overdueTrend.trendType.toLowerCase())}</p> <p class="text-xs text-surface-500">${escape_html(Math.abs(overdueTrend.trendValue).toFixed(1))}% change</p></div></div></div></div></div>`);
		}
		$$renderer.push(`<!--]-->`);
		if ($$store_subs) unsubscribe_stores($$store_subs);
	});
}
//#endregion
export { _page as default };
