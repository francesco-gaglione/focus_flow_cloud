use crate::{
    components::select::{Select, SelectList, SelectOption, SelectTrigger, SelectValue},
    i18n::{save_locale, use_i18n, I18n, Locale},
    presentation::components::common_components::bottom_sheet::BottomSheet,
    services::storage::set_item,
    state::app_state::AppState,
    use_cases::auth::update_base_url_uc::update_base_url_uc,
};
use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
    let mut drawer_open = use_context::<Signal<bool>>();
    let mut app_state = use_context::<Signal<AppState>>();
    let mut i18n = use_i18n();

    let mut sheet_open = use_signal(|| false);
    let mut edit_url = use_signal(|| app_state.read().server_url().unwrap_or("").to_string());

    let version = env!("CARGO_PKG_VERSION");
    let current_locale = i18n.read().locale.code().to_string();

    rsx! {
        div { class: "flex-1 min-h-0 flex flex-col overflow-hidden",
            // Header
            div { class: "shrink-0 px-4 pt-2 pb-4 flex items-center gap-3 bg-surface",
                button {
                    class: "size-9 bg-surface-card border border-border rounded-sm text-subtle grid place-items-center cursor-pointer shrink-0 transition-[background,border-color,color] duration-fast ease-tech hover:bg-gray-200 hover:border-border-strong hover:text-foreground active:bg-gray-300",
                    onclick: move |_| drawer_open.set(true),
                    svg {
                        view_box: "0 0 16 16",
                        width: "16",
                        height: "16",
                        stroke: "currentColor",
                        fill: "none",
                        stroke_width: "1.6",
                        line { x1: "3", y1: "5", x2: "13", y2: "5" }
                        line { x1: "3", y1: "8", x2: "13", y2: "8" }
                        line { x1: "3", y1: "11", x2: "13", y2: "11" }
                    }
                }
                div { class: "flex-1 min-w-0",
                    div { class: "text-[22px] font-bold leading-[1.15] tracking-[-0.03em] text-foreground",
                        "{i18n.read().t(\"layout.settings\")}"
                    }
                }
            }

            div { class: "flex-1 overflow-y-auto px-4 pb-8",
                div { class: "pt-5 pb-2 font-mono text-[10px] text-subtle tracking-[0.02em] uppercase",
                    "{i18n.read().t(\"layout.comment_app_info\")}"
                }
                div { class: "flex flex-col rounded-md border border-border overflow-hidden",
                    div { class: "flex items-center justify-between px-4 py-3 bg-surface-card border-b border-border",
                        span { class: "font-mono text-xs text-subtle",
                            "{i18n.read().t(\"layout.version_label\")}"
                        }
                        span { class: "font-mono text-xs text-foreground", "v{version}" }
                    }
                    button {
                        class: "appearance-none flex items-center justify-between px-4 py-3 bg-surface-card text-left w-full cursor-pointer border-0 border-b border-border transition-[background] duration-fast ease-tech hover:bg-gray-100 active:bg-gray-200",
                        onclick: move |_| {
                            edit_url.set(app_state.read().server_url().unwrap_or("").to_string());
                            sheet_open.set(true);
                        },
                        span { class: "font-mono text-xs text-subtle",
                            "{i18n.read().t(\"layout.server_label\")}"
                        }
                        div { class: "flex items-center gap-2",
                            span { class: "font-mono text-xs text-foreground truncate max-w-[180px]",
                                "{app_state.read().server_url().unwrap_or(\"-\")}"
                            }
                            svg {
                                view_box: "0 0 16 16",
                                width: "12",
                                height: "12",
                                stroke: "currentColor",
                                fill: "none",
                                stroke_width: "1.5",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M6 3l5 5-5 5" }
                            }
                        }
                    }
                    div { class: "flex items-center justify-between px-4 py-3 bg-surface-card",
                        span { class: "font-mono text-xs text-subtle",
                            "{i18n.read().t(\"layout.language_label\")}"
                        }
                        div { class: "min-w-[140px]",
                            Select::<String> {
                                default_value: Some(current_locale),
                                on_value_change: move |v: Option<String>| {
                                    if let Some(code) = v {
                                        let locale = match code.as_str() {
                                            "it" => Locale::It,
                                            _ => Locale::En,
                                        };
                                        save_locale(&locale);
                                        *i18n.write() = I18n::new(locale);
                                    }
                                },
                                SelectTrigger {
                                    SelectValue { placeholder: "{i18n.read().t(\"layout.language_label\")}" }
                                }
                                SelectList {
                                    SelectOption::<String> {
                                        index: 0_usize,
                                        value: "en".to_string(),
                                        text_value: "English",
                                        "{i18n.read().t(\"layout.language_en\")}"
                                    }
                                    SelectOption::<String> {
                                        index: 1_usize,
                                        value: "it".to_string(),
                                        text_value: "Italiano",
                                        "{i18n.read().t(\"layout.language_it\")}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            BottomSheet {
                show: *sheet_open.read(),
                title: i18n.read().t("layout.edit_server_title"),
                on_close: move |_| sheet_open.set(false),
                div { class: "px-5 pt-4 pb-2 flex flex-col gap-4",
                    input {
                        class: "w-full px-3 py-2 text-sm font-mono bg-surface-card border border-border rounded-md text-foreground placeholder:text-subtle focus:outline-none focus:border-accent",
                        r#type: "text",
                        value: "{edit_url.read()}",
                        oninput: move |e| edit_url.set(e.value().clone()),
                        placeholder: "http://192.168.1.x:8080",
                    }
                    div { class: "flex gap-2 pb-2",
                        button {
                            class: "flex-1 px-4 py-2 text-sm font-medium rounded-md border border-border text-subtle bg-surface-card cursor-pointer transition-[background] duration-fast ease-tech hover:bg-gray-100 active:bg-gray-200",
                            onclick: move |_| sheet_open.set(false),
                            "{i18n.read().t(\"layout.cancel\")}"
                        }
                        button {
                            class: "flex-1 px-4 py-2 text-sm font-medium rounded-md bg-accent text-white cursor-pointer border-0 transition-[background] duration-fast ease-tech hover:opacity-90 active:opacity-80",
                            onclick: move |_| {
                                let url = edit_url.read().clone();
                                set_item("server_url", &url);
                                app_state.write().set_server_url(url.clone());
                                let _ = update_base_url_uc(&url);
                                sheet_open.set(false);
                            },
                            "{i18n.read().t(\"layout.save\")}"
                        }
                    }
                }
            }
        }
    }
}
