use crate::{
    clients::auth_client::logout,
    i18n::{save_locale, use_i18n, I18n, Locale},
    presentation::views::auth_page::AuthPage,
    state::auth_state::AuthState,
    Route,
};
use dioxus::prelude::*;

fn sd_item_cls(active: bool) -> &'static str {
    if active {
        "appearance-none bg-accent-soft border-0 rounded-md cursor-pointer flex items-center gap-3 px-[10px] py-[10px] text-accent font-sans text-sm font-medium text-left w-full no-underline transition-[background,color] duration-fast ease-tech"
    } else {
        "appearance-none bg-transparent border-0 rounded-md cursor-pointer flex items-center gap-3 px-[10px] py-[10px] text-subtle font-sans text-sm font-medium text-left w-full no-underline transition-[background,color] duration-fast ease-tech hover:bg-surface-card hover:text-muted active:bg-gray-200"
    }
}

fn sd_icon_cls(active: bool) -> &'static str {
    if active {
        "size-8 border border-accent rounded-sm bg-accent text-white grid place-items-center shrink-0 transition-[background,color,border-color] duration-fast ease-tech"
    } else {
        "size-8 border border-border rounded-sm bg-surface-card text-subtle grid place-items-center shrink-0 transition-[background,color,border-color] duration-fast ease-tech"
    }
}

#[component]
pub fn Layout() -> Element {
    let mut drawer_open: Signal<bool> = use_context_provider(|| Signal::new(false));
    let mut auth_state = use_context::<Signal<AuthState>>();
    let mut i18n = use_i18n();

    let logout_handler = move |_| {
        auth_state.write().delete_auth_token();
        auth_state.write().delete_refresh_token();
        drawer_open.set(false);
        spawn(async move {
            let _ = logout().await;
        });
    };

    let route = use_route::<Route>();
    let active_section = match route {
        Route::Todo {}
        | Route::Calendar {}
        | Route::Stats {}
        | Route::Pomodoro {}
        | Route::Categories {} => "tasks",
        Route::Flashcards {} => "flashcards",
    };

    let is_en = i18n.read().locale == Locale::En;
    let is_it = i18n.read().locale == Locale::It;

    rsx! {
        div { class: "h-screen flex flex-col relative overflow-hidden",
            div { class: "flex-1 min-h-0 flex flex-col overflow-hidden",
                if auth_state.read().is_authenticated() {
                    Outlet::<Route> {}
                } else {
                    div {
                        AuthPage {}
                    }
                }
            }

            // Overlay
            div {
                class: if *drawer_open.read() {
                    "absolute inset-0 bg-black/55 backdrop-blur-[4px] opacity-100 z-[30] pointer-events-auto transition-opacity duration-base ease-tech"
                } else {
                    "absolute inset-0 bg-black/55 backdrop-blur-[4px] opacity-0 z-[30] pointer-events-none transition-opacity duration-base ease-tech"
                },
                onclick: move |_| drawer_open.set(false),
            }

            // Side drawer
            aside {
                class: if *drawer_open.read() {
                    "absolute top-0 left-0 w-[76%] h-full bg-surface-raised border-r border-border translate-x-0 transition-transform duration-base ease-snap z-[31] flex flex-col overflow-hidden"
                } else {
                    "absolute top-0 left-0 w-[76%] h-full bg-surface-raised border-r border-border -translate-x-full transition-transform duration-base ease-snap z-[31] flex flex-col overflow-hidden"
                },

                // Header
                div { class: "pt-[52px] px-5 pb-[18px] flex items-center gap-3 border-b border-border",
                    div { class: "size-[34px] bg-accent-soft border border-accent rounded-sm grid place-items-center shrink-0 relative after:content-[''] after:absolute after:size-3 after:bg-accent after:rounded-[3px]" }
                    div { class: "text-[20px] font-bold text-foreground tracking-[-0.025em]",
                        "{i18n.read().t(\"layout.focus\")}"
                        em { class: "text-accent not-italic", "{i18n.read().t(\"layout.flow\")}" }
                    }
                    button {
                        class: "ml-auto size-[30px] bg-surface-card border border-border rounded-sm text-subtle grid place-items-center cursor-pointer transition-[background,color] duration-fast ease-tech hover:bg-gray-200 hover:text-foreground",
                        onclick: move |_| drawer_open.set(false),
                        svg { view_box: "0 0 16 16", width: "14", height: "14", stroke: "currentColor", fill: "none", stroke_width: "1.6",
                            line { x1: "4", y1: "4", x2: "12", y2: "12" }
                            line { x1: "12", y1: "4", x2: "4", y2: "12" }
                        }
                    }
                }

                // Eyebrow
                div { class: "px-5 pt-4 pb-1.5 font-mono text-[10px] text-subtle tracking-[0.02em] uppercase",
                    "{i18n.read().t(\"layout.comment_workspace\")}"
                }

                // Nav
                nav { class: "px-3 flex flex-col gap-0.5",
                    Link {
                        to: Route::Todo {},
                        class: sd_item_cls(active_section == "tasks"),
                        onclick: move |_| drawer_open.set(false),
                        span { class: sd_icon_cls(active_section == "tasks"),
                            svg { view_box: "0 0 16 16", width: "14", height: "14", stroke: "currentColor", fill: "none", stroke_width: "1.5",
                                path { d: "M3 4h10M3 8h10M3 12h6" }
                            }
                        }
                        span { class: "flex-1", "{i18n.read().t(\"layout.tasks\")}" }
                    }
                    Link {
                        to: Route::Flashcards {},
                        class: sd_item_cls(active_section == "flashcards"),
                        onclick: move |_| drawer_open.set(false),
                        span { class: sd_icon_cls(active_section == "flashcards"),
                            svg { view_box: "0 0 16 16", width: "14", height: "14", stroke: "currentColor", fill: "none", stroke_width: "1.5",
                                rect { x: "2", y: "3", width: "10", height: "9" }
                                rect { x: "4", y: "5", width: "10", height: "9" }
                            }
                        }
                        span { class: "flex-1", "{i18n.read().t(\"layout.flashcards\")}" }
                        span { class: "font-mono text-[10px] text-subtle tracking-[0.02em] uppercase", "{i18n.read().t(\"layout.soon\")}" }
                    }
                }

                // Footer
                div { class: "mt-auto px-5 pt-4 pb-8 flex items-center gap-3 border-t border-border",
                    div { class: "size-[34px] bg-gray-300 border border-border rounded-full text-muted grid place-items-center font-mono text-xs font-semibold shrink-0",
                        "FF"
                    }
                    div { class: "flex flex-col flex-1 min-w-0",
                        span { class: "text-sm font-medium text-foreground", "{i18n.read().t(\"layout.user_label\")}" }
                        span { class: "font-mono text-xs text-subtle whitespace-nowrap overflow-hidden text-ellipsis", "{i18n.read().t(\"layout.user_status\")}" }
                    }

                    // Language switcher
                    div { class: "flex items-center gap-1 shrink-0",
                        button {
                            class: if is_en {
                                "px-2 py-1 font-mono text-[10px] rounded-sm cursor-pointer border border-accent bg-accent-soft text-accent tracking-[0.04em] uppercase"
                            } else {
                                "px-2 py-1 font-mono text-[10px] rounded-sm cursor-pointer border border-border text-subtle tracking-[0.04em] uppercase hover:border-border-strong hover:text-muted"
                            },
                            onclick: move |_| {
                                let locale = Locale::En;
                                save_locale(&locale);
                                *i18n.write() = I18n::new(locale);
                            },
                            "EN"
                        }
                        button {
                            class: if is_it {
                                "px-2 py-1 font-mono text-[10px] rounded-sm cursor-pointer border border-accent bg-accent-soft text-accent tracking-[0.04em] uppercase"
                            } else {
                                "px-2 py-1 font-mono text-[10px] rounded-sm cursor-pointer border border-border text-subtle tracking-[0.04em] uppercase hover:border-border-strong hover:text-muted"
                            },
                            onclick: move |_| {
                                let locale = Locale::It;
                                save_locale(&locale);
                                *i18n.write() = I18n::new(locale);
                            },
                            "IT"
                        }
                    }

                    button {
                        class: "size-8 bg-transparent border border-border rounded-sm text-subtle grid place-items-center cursor-pointer shrink-0 ml-1 transition-[background,border-color,color] duration-fast ease-tech hover:bg-danger/10 hover:border-danger hover:text-danger",
                        r#type: "button",
                        onclick: logout_handler,
                        svg { view_box: "0 0 16 16", width: "14", height: "14", stroke: "currentColor", fill: "none", stroke_width: "1.5", stroke_linecap: "round",
                            path { d: "M6 3H3a1 1 0 0 0-1 1v8a1 1 0 0 0 1 1h3" }
                            polyline { points: "10 11 13 8 10 5", stroke_linejoin: "round" }
                            line { x1: "13", y1: "8", x2: "6", y2: "8" }
                        }
                    }
                }
            }
        }
    }
}
