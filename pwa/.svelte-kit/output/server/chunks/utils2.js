import "clsx";
//#region src/lib/utils.ts
function fmtTime(secs) {
	const s = Math.max(0, secs);
	return `${String(Math.floor(s / 60)).padStart(2, "0")}:${String(s % 60).padStart(2, "0")}`;
}
function fmtDuration(secs) {
	if (secs < 60) return `${secs}s`;
	const m = Math.floor(secs / 60);
	if (m < 60) return `${m}m`;
	return `${Math.floor(m / 60)}h ${m % 60}m`;
}
function tsToDate(ts) {
	return /* @__PURE__ */ new Date(ts * 1e3);
}
function isToday(ts) {
	const d = tsToDate(ts);
	const n = /* @__PURE__ */ new Date();
	return d.getFullYear() === n.getFullYear() && d.getMonth() === n.getMonth() && d.getDate() === n.getDate();
}
function isOverdue(ts) {
	return tsToDate(ts) < /* @__PURE__ */ new Date() && !isToday(ts);
}
//#endregion
export { tsToDate as a, isToday as i, fmtTime as n, isOverdue as r, fmtDuration as t };
