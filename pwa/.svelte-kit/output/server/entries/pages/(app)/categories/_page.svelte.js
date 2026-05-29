import { J as attr, X as escape_html, c as ensure_array_like, g as unsubscribe_stores, h as stringify, m as store_get, o as derived, r as attr_style } from "../../../../chunks/dev.js";
import { a as useQueryClient, i as createQuery, r as createMutation } from "../../../../chunks/dist.js";
import { r as categories } from "../../../../chunks/api.js";
//#region src/components/categories/ColorSwatch.svelte
var PRESET_COLORS = [
	"#12a594",
	"#46a758",
	"#ffb224",
	"#f2056f",
	"#8e4ec6",
	"#e5484d",
	"#0070f3",
	"#d97706",
	"#7c3aed",
	"#6b7280"
];
function ColorSwatch($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const { colors, selected, onSelect } = $$props;
		$$renderer.push(`<div class="flex gap-1.5 flex-wrap"><!--[-->`);
		const each_array = ensure_array_like(colors);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let c = each_array[$$index];
			$$renderer.push(`<button class="size-6 rounded-full transition-all"${attr_style(`background: ${stringify(c)}; outline: ${selected === c ? "2px solid white" : "2px solid transparent"}; outline-offset: 1px`)}></button>`);
		}
		$$renderer.push(`<!--]--></div>`);
	});
}
//#endregion
//#region src/components/categories/CategoryRow.svelte
function CategoryRow($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const { cat, onDelete } = $$props;
		const qc = useQueryClient();
		let editing = false;
		let name = cat.name;
		let color = cat.color;
		createMutation({
			mutationFn: () => categories.update(cat.id, {
				name,
				color
			}),
			onSuccess: () => {
				qc.invalidateQueries({ queryKey: ["categories"] });
				editing = false;
			}
		});
		if (editing) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<div class="p-3 mb-2 rounded-r-md border border-surface-700 bg-surface-900 flex flex-col gap-3"${attr_style(`border-left: 4px solid ${stringify(color)}`)}><input${attr("value", name)} class="input text-sm" placeholder="Category name"/> `);
			ColorSwatch($$renderer, {
				colors: PRESET_COLORS,
				selected: color,
				onSelect: (c) => color = c
			});
			$$renderer.push(`<!----> <div class="flex gap-2"><button class="btn preset-filled-primary-500 text-xs h-8 px-4 flex-1">Save</button> <button class="btn preset-tonal-surface text-xs h-8 px-4">Cancel</button></div></div>`);
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<div class="flex items-center gap-3 p-3 mb-2 rounded-r-md border border-surface-700 bg-surface-900"${attr_style(`border-left: 4px solid ${stringify(cat.color)}`)}><div class="size-3 rounded-full shrink-0"${attr_style(`background: ${stringify(cat.color)}`)}></div> <span class="flex-1 text-sm text-surface-100">${escape_html(cat.name)}</span> <button class="btn btn-icon preset-tonal-surface size-7" aria-label="Edit"><svg viewBox="0 0 16 16" width="13" height="13" stroke="currentColor" fill="none" stroke-width="1.5"><path d="M11 2l3 3-9 9H2v-3l9-9z"></path></svg></button> <button class="btn btn-icon preset-tonal-error size-7" aria-label="Delete"><svg viewBox="0 0 16 16" width="13" height="13" stroke="currentColor" fill="none" stroke-width="1.5"><polyline points="3 4 13 4"></polyline><path d="M5 4V2h6v2"></path><path d="M4 4l1 10h6l1-10"></path></svg></button></div>`);
		}
		$$renderer.push(`<!--]-->`);
		if ($$store_subs) unsubscribe_stores($$store_subs);
	});
}
//#endregion
//#region src/routes/(app)/categories/+page.svelte
function _page($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const qc = useQueryClient();
		let newName = "";
		let newColor = PRESET_COLORS[0];
		const catsQuery = createQuery({
			queryKey: ["categories"],
			queryFn: categories.getAll
		});
		let cats = derived(() => store_get($$store_subs ??= {}, "$catsQuery", catsQuery).data?.categories ?? []);
		createMutation({
			mutationFn: () => categories.create({
				name: newName.trim(),
				color: newColor
			}),
			onSuccess: () => {
				qc.invalidateQueries({ queryKey: ["categories"] });
				newName = "";
			}
		});
		const del = createMutation({
			mutationFn: (id) => categories.delete(id),
			onSuccess: () => qc.invalidateQueries({ queryKey: ["categories"] })
		});
		$$renderer.push(`<div class="flex-1 min-h-0 flex flex-col overflow-hidden"><div class="flex-1 overflow-y-auto pb-32 px-4 pt-3"><!--[-->`);
		const each_array = ensure_array_like(cats());
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let cat = each_array[$$index];
			CategoryRow($$renderer, {
				cat,
				onDelete: (id) => store_get($$store_subs ??= {}, "$del", del).mutate(id)
			});
		}
		$$renderer.push(`<!--]--> `);
		if (cats().length === 0) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<div class="flex flex-col items-center justify-center py-16 gap-3 text-surface-500"><svg viewBox="0 0 20 20" width="28" height="28" stroke="currentColor" fill="none" stroke-width="1.5"><circle cx="10" cy="10" r="3"></circle><circle cx="10" cy="10" r="8"></circle></svg> <p class="text-sm">No categories yet</p></div>`);
		} else $$renderer.push("<!--[-1-->");
		$$renderer.push(`<!--]--> <div class="card bg-surface-900 border border-surface-700 p-4 mt-4"><p class="text-xs font-mono text-surface-500 uppercase tracking-widest mb-3">New Category</p> <div class="flex flex-col gap-3"><input class="input text-sm" placeholder="Category name"${attr("value", newName)}/> `);
		ColorSwatch($$renderer, {
			colors: PRESET_COLORS,
			selected: newColor,
			onSelect: (c) => newColor = c
		});
		$$renderer.push(`<!----> <button${attr("disabled", !newName.trim(), true)} class="btn preset-filled-primary-500 w-full disabled:opacity-50">Create Category</button></div></div></div></div>`);
		if ($$store_subs) unsubscribe_stores($$store_subs);
	});
}
//#endregion
export { _page as default };
