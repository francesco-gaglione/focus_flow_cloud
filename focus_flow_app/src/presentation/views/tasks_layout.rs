use crate::{i18n::use_i18n, Route};
use dioxus::prelude::*;

fn tab_cls(active: bool) -> &'static str {
    if active {
        "appearance-none bg-transparent border-0 rounded-md cursor-pointer flex flex-col items-center gap-1 py-2 px-1 font-sans text-[10px] font-medium tracking-[0.01em] text-accent relative transition-colors duration-fast ease-tech active:opacity-70"
    } else {
        "appearance-none bg-transparent border-0 rounded-md cursor-pointer flex flex-col items-center gap-1 py-2 px-1 font-sans text-[10px] font-medium tracking-[0.01em] text-subtle relative transition-colors duration-fast ease-tech active:opacity-70"
    }
}

fn ico_cls(active: bool) -> &'static str {
    if active {
        "size-7 grid place-items-center rounded-sm bg-accent-soft transition-[background] duration-fast ease-tech"
    } else {
        "size-7 grid place-items-center rounded-sm transition-[background] duration-fast ease-tech"
    }
}

#[component]
pub fn TasksLayout() -> Element {
    let mut drawer_open = use_context::<Signal<bool>>();
    let i18n = use_i18n();
    let route = use_route::<Route>();

    let (title_key, active_tab) = match route {
        Route::Todo {}       => ("tasks_layout.title_todo",       "todo"),
        Route::Calendar {}   => ("tasks_layout.title_calendar",   "calendar"),
        Route::Stats {}      => ("tasks_layout.title_stats",      "stats"),
        Route::Pomodoro {}   => ("tasks_layout.title_timer",      "timer"),
        Route::Categories {} => ("tasks_layout.title_categories", "categories"),
        _                    => ("tasks_layout.title_todo",       "todo"),
    };
    let title_html = i18n.read().t(title_key);
    let on_categories = active_tab == "categories";

    rsx! {
        div { class: "shrink-0 px-4 pt-2 pb-4 flex items-center gap-3 bg-surface relative z-[5]",
            button {
                class: "size-9 bg-surface-card border border-border rounded-sm text-subtle grid place-items-center cursor-pointer shrink-0 transition-[background,border-color,color] duration-fast ease-tech hover:bg-gray-200 hover:border-border-strong hover:text-foreground active:bg-gray-300",
                onclick: move |_| drawer_open.set(true),
                svg { view_box: "0 0 16 16", width: "16", height: "16", stroke: "currentColor", fill: "none", stroke_width: "1.6",
                    line { x1: "3", y1: "5", x2: "13", y2: "5" }
                    line { x1: "3", y1: "8", x2: "13", y2: "8" }
                    line { x1: "3", y1: "11", x2: "13", y2: "11" }
                }
            }
            div { class: "flex-1 min-w-0",
                div {
                    class: "text-[22px] font-bold leading-[1.15] tracking-[-0.03em] text-foreground whitespace-nowrap overflow-hidden text-ellipsis [&_em]:text-accent [&_em]:not-italic",
                    dangerous_inner_html: title_html
                }
            }
            Link {
                to: Route::Categories {},
                class: if on_categories {
                    "size-9 bg-accent-soft border border-accent rounded-sm text-accent grid place-items-center cursor-pointer shrink-0 transition-[background,border-color,color] duration-fast ease-tech"
                } else {
                    "size-9 bg-surface-card border border-border rounded-sm text-subtle grid place-items-center cursor-pointer shrink-0 transition-[background,border-color,color] duration-fast ease-tech hover:bg-gray-200 hover:border-border-strong hover:text-foreground active:bg-gray-300"
                },
                title: i18n.read().t("tasks_layout.manage_categories"),
                svg { view_box: "0 0 16 16", width: "16", height: "16", stroke: "currentColor", fill: "none", stroke_width: "1.6", stroke_linecap: "round", stroke_linejoin: "round",
                    path { d: "M9 3H3a1 1 0 00-1 1v2a1 1 0 001 1h6l3 3 3-3V4a1 1 0 00-1-1h-2" }
                    path { d: "M9 9H3a1 1 0 00-1 1v2a1 1 0 001 1h6" }
                }
            }
        }

        Outlet::<Route> {}

        nav { class: "fixed bottom-0 left-0 right-0 z-20 bg-surface-raised border-t border-border grid grid-cols-4 p-1 pb-[18px] gap-0.5",
            Link {
                to: Route::Todo {},
                class: tab_cls(active_tab == "todo"),
                span { class: ico_cls(active_tab == "todo"),
                    svg { view_box: "0 0 24 24", width: "20", height: "20", stroke: "currentColor", stroke_width: if active_tab == "todo" { "2" } else { "1.6" }, fill: "none", stroke_linecap: "round",
                        path { d: "M5 7h14M5 12h14M5 17h8" }
                    }
                }
                span { "{i18n.read().t(\"tasks_layout.tab_tasks\")}" }
            }
            Link {
                to: Route::Calendar {},
                class: tab_cls(active_tab == "calendar"),
                span { class: ico_cls(active_tab == "calendar"),
                    svg { view_box: "0 0 24 24", width: "20", height: "20", stroke: "currentColor", stroke_width: if active_tab == "calendar" { "2" } else { "1.6" }, fill: "none",
                        rect { x: "3", y: "5", width: "18", height: "16", rx: "1" }
                        line { x1: "3", y1: "10", x2: "21", y2: "10" }
                        line { x1: "8", y1: "3", x2: "8", y2: "7" }
                        line { x1: "16", y1: "3", x2: "16", y2: "7" }
                    }
                }
                span { "{i18n.read().t(\"tasks_layout.tab_calendar\")}" }
            }
            Link {
                to: Route::Stats {},
                class: tab_cls(active_tab == "stats"),
                span { class: ico_cls(active_tab == "stats"),
                    svg { view_box: "0 0 24 24", width: "20", height: "20", stroke: "currentColor", stroke_width: if active_tab == "stats" { "2" } else { "1.6" }, fill: "none",
                        line { x1: "5", y1: "20", x2: "5", y2: "14" }
                        line { x1: "12", y1: "20", x2: "12", y2: "6" }
                        line { x1: "19", y1: "20", x2: "19", y2: "11" }
                    }
                }
                span { "{i18n.read().t(\"tasks_layout.tab_stats\")}" }
            }
            Link {
                to: Route::Pomodoro {},
                class: tab_cls(active_tab == "timer"),
                span { class: ico_cls(active_tab == "timer"),
                    svg { view_box: "0 0 24 24", width: "20", height: "20", stroke: "currentColor", stroke_width: if active_tab == "timer" { "2" } else { "1.6" }, fill: "none", stroke_linecap: "round",
                        circle { cx: "12", cy: "13", r: "7" }
                        line { x1: "12", y1: "13", x2: "12", y2: "9" }
                        line { x1: "12", y1: "13", x2: "15", y2: "11" }
                        line { x1: "9", y1: "3", x2: "15", y2: "3" }
                    }
                }
                span { "{i18n.read().t(\"tasks_layout.tab_timer\")}" }
            }
        }
    }
}
