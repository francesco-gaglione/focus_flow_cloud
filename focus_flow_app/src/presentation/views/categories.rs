use dioxus::prelude::*;

use crate::{
    clients::category_http_client::get_all_categories,
    use_cases::tasks::{
        create_category_uc::create_category_uc,
        delete_category_uc::delete_category_uc,
        update_category_uc::update_category_uc,
    },
};

#[derive(Clone, PartialEq)]
struct CatItem {
    id: String,
    name: String,
    color: String,
}

const PRESET_COLORS: &[(&str, &str)] = &[
    ("#46a758", "Green"),
    ("#12a594", "Cyan"),
    ("#e5484d", "Red"),
    ("#ffb224", "Amber"),
    ("#0070f3", "Blue"),
    ("#7c3aed", "Purple"),
    ("#d97706", "Orange"),
    ("#6b7280", "Gray"),
];

const INPUT_CLS: &str = "flex-1 h-10 px-3 bg-surface-raised border border-border rounded-md text-foreground font-sans text-sm outline-none transition-[border-color,box-shadow] duration-fast ease-tech placeholder:text-subtle focus:border-accent focus:[box-shadow:var(--shadow-focus)]";

#[component]
pub fn Categories() -> Element {
    let mut cats: Signal<Vec<CatItem>> = use_signal(Vec::new);
    let mut loading = use_signal(|| true);

    let mut new_name = use_signal(String::new);
    let mut new_color = use_signal(|| PRESET_COLORS[0].0.to_string());

    let mut editing_id: Signal<Option<String>> = use_signal(|| None);
    let mut editing_name = use_signal(String::new);
    let mut editing_color = use_signal(|| PRESET_COLORS[0].0.to_string());

    let mut fetch = use_resource(move || async move {
        if let Ok(res) = get_all_categories().await {
            cats.set(
                res.categories
                    .into_iter()
                    .map(|c| CatItem { id: c.id, name: c.name, color: c.color })
                    .collect(),
            );
        }
        loading.set(false);
    });

    let cat_list: Vec<CatItem> = cats.read().clone();

    rsx! {
        div { class: "scroll px-4 pt-4 pb-8 flex flex-col gap-6",

            div { class: "flex flex-col gap-3 p-4 bg-surface-card border border-border rounded-lg",
                p { class: "font-mono text-xs font-medium tracking-[0.02em] uppercase text-subtle mb-1", "New category" }
                ColorPicker { selected: new_color.read().clone(), on_pick: move |c: String| new_color.set(c) }
                div { class: "flex gap-2",
                    input {
                        class: INPUT_CLS,
                        placeholder: "Category name…",
                        value: "{new_name}",
                        oninput: move |e| new_name.set(e.value()),
                        onkeydown: move |e| {
                            if e.key() == Key::Enter {
                                let name = new_name.read().trim().to_string();
                                let color = new_color.read().clone();
                                if name.is_empty() { return; }
                                spawn(async move {
                                    if create_category_uc(&name, &color).await.is_ok() {
                                        new_name.set(String::new());
                                        loading.set(true);
                                        fetch.restart();
                                    }
                                });
                            }
                        },
                    }
                    button {
                        r#type: "button",
                        class: "h-10 px-4 bg-accent text-white font-mono text-sm font-medium rounded-md border-0 cursor-pointer shrink-0 transition-[background,opacity] duration-fast ease-tech hover:bg-accent-hover disabled:opacity-40 disabled:cursor-not-allowed",
                        disabled: new_name.read().trim().is_empty(),
                        onclick: move |_| {
                            let name = new_name.read().trim().to_string();
                            let color = new_color.read().clone();
                            if name.is_empty() { return; }
                            spawn(async move {
                                if create_category_uc(&name, &color).await.is_ok() {
                                    new_name.set(String::new());
                                    loading.set(true);
                                    fetch.restart();
                                }
                            });
                        },
                        "Add"
                    }
                }
            }

            if *loading.read() {
                div { class: "empty-state", p { "Loading…" } }
            } else if cat_list.is_empty() {
                div { class: "empty-state", p { "No categories yet. Add one above." } }
            } else {
                div { class: "flex flex-col gap-2",
                    for cat in cat_list {
                        {
                            let is_editing = editing_id.read().as_deref() == Some(cat.id.as_str());
                            let id_for_edit = cat.id.clone();
                            let name_for_edit = cat.name.clone();
                            let color_for_edit = cat.color.clone();
                            rsx! {
                                if is_editing {
                                    div { class: "flex flex-col gap-2 p-3 bg-surface-card border border-accent rounded-lg",
                                        ColorPicker {
                                            selected: editing_color.read().clone(),
                                            on_pick: move |c: String| editing_color.set(c),
                                        }
                                        div { class: "flex gap-2",
                                            input {
                                                class: INPUT_CLS,
                                                value: "{editing_name}",
                                                oninput: move |e| editing_name.set(e.value()),
                                            }
                                            button {
                                                r#type: "button",
                                                class: "h-10 px-3 bg-accent text-white font-mono text-xs font-medium rounded-md border-0 cursor-pointer shrink-0 transition-[background] duration-fast ease-tech hover:bg-accent-hover",
                                                onclick: move |_| {
                                                    let id = id_for_edit.clone();
                                                    let name = editing_name.read().trim().to_string();
                                                    let color = editing_color.read().clone();
                                                    editing_id.set(None);
                                                    spawn(async move {
                                                        let _ = update_category_uc(&id, Some(&name), Some(&color)).await;
                                                        loading.set(true);
                                                        fetch.restart();
                                                    });
                                                },
                                                "Save"
                                            }
                                            button {
                                                r#type: "button",
                                                class: "h-10 px-3 bg-surface-raised border border-border text-subtle font-mono text-xs rounded-md cursor-pointer shrink-0 transition-colors duration-fast ease-tech hover:text-foreground",
                                                onclick: move |_| editing_id.set(None),
                                                "Cancel"
                                            }
                                        }
                                    }
                                } else {
                                    div { class: "flex items-center gap-3 px-3 py-2.5 bg-surface-card border border-border rounded-lg",
                                        div {
                                            class: "w-4 h-4 rounded-full shrink-0",
                                            style: "background: {cat.color};",
                                        }
                                        span { class: "flex-1 text-sm text-foreground font-sans min-w-0 truncate", "{cat.name}" }
                                        button {
                                            r#type: "button",
                                            class: "size-8 grid place-items-center bg-transparent border-0 text-subtle cursor-pointer rounded-sm transition-colors duration-fast ease-tech hover:text-foreground hover:bg-surface-raised shrink-0",
                                            title: "Edit",
                                            onclick: move |_| {
                                                editing_id.set(Some(id_for_edit.clone()));
                                                editing_name.set(name_for_edit.clone());
                                                editing_color.set(color_for_edit.clone());
                                            },
                                            svg { view_box: "0 0 16 16", width: "14", height: "14", stroke: "currentColor", fill: "none", stroke_width: "1.6", stroke_linecap: "round", stroke_linejoin: "round",
                                                path { d: "M11.5 2.5a1.5 1.5 0 012 2L5 13 2 14l1-3 8.5-8.5z" }
                                            }
                                        }
                                        button {
                                            r#type: "button",
                                            class: "size-8 grid place-items-center bg-transparent border-0 text-subtle cursor-pointer rounded-sm transition-colors duration-fast ease-tech hover:text-danger hover:bg-surface-raised shrink-0",
                                            title: "Delete",
                                            onclick: move |_| {
                                                let id = cat.id.clone();
                                                spawn(async move {
                                                    if delete_category_uc(&id).await.is_ok() {
                                                        loading.set(true);
                                                        fetch.restart();
                                                    }
                                                });
                                            },
                                            svg { view_box: "0 0 16 16", width: "14", height: "14", stroke: "currentColor", fill: "none", stroke_width: "1.6", stroke_linecap: "round", stroke_linejoin: "round",
                                                polyline { points: "3 6 13 6" }
                                                path { d: "M5 6V4h6v2M4 6v8a1 1 0 001 1h6a1 1 0 001-1V6" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ColorPickerProps {
    selected: String,
    on_pick: EventHandler<String>,
}

#[component]
fn ColorPicker(props: ColorPickerProps) -> Element {
    rsx! {
        div { class: "flex gap-2 flex-wrap",
            for &(color, label) in PRESET_COLORS.iter() {
                {
                    let c = color.to_string();
                    let selected = props.selected == c;
                    rsx! {
                        button {
                            r#type: "button",
                            title: "{label}",
                            class: "w-7 h-7 rounded-full cursor-pointer border-2 transition-all duration-fast ease-tech hover:scale-110 active:scale-95",
                            style: if selected {
                                format!("background:{c};border-color:white;")
                            } else {
                                format!("background:{c};border-color:transparent;")
                            },
                            onclick: move |_| props.on_pick.call(c.clone()),
                        }
                    }
                }
            }
        }
    }
}
