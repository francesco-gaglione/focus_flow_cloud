

export const index = 0;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_layout.svelte.js')).default;
export const universal = {
  "ssr": false,
  "prerender": false
};
export const universal_id = "src/routes/+layout.ts";
export const imports = ["_app/immutable/nodes/0.DVUG7SEV.js","_app/immutable/chunks/CqF68UAt.js","_app/immutable/chunks/PcpVdzpd.js","_app/immutable/chunks/B_ndAkh-.js","_app/immutable/chunks/TvYuZPd6.js","_app/immutable/chunks/b1RuDTak.js","_app/immutable/chunks/xihTtKlq.js","_app/immutable/chunks/C-SdEcAj.js"];
export const stylesheets = ["_app/immutable/assets/0.CtE5dG5c.css"];
export const fonts = [];
