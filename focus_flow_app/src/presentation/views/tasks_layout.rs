use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn TasksLayout() -> Element {
    let mut drawer_open = use_context::<Signal<bool>>();
    let route = use_route::<Route>();

    let (crumb_sub, title_html, active_tab) = match route {
        Route::Todo {} => ("Today", "Tasks, <em>focus please.</em>", "todo"),
        Route::Calendar {} => ("Calendar", "A month, <em>at a glance</em>.", "calendar"),
        Route::Stats {} => ("Progress", "Your <em>quiet</em> wins.", "stats"),
        Route::Pomodoro {} => ("Timer", "Pomodoro, <em>deep work.</em>", "timer"),
        _ => ("Today", "Today.", "todo"),
    };

    rsx! {
        div { class: "app-bar",
            button {
                class: "icon-btn",
                onclick: move |_| drawer_open.set(true),
                svg { view_box: "0 0 16 16",
                    line { x1: "3", y1: "5", x2: "13", y2: "5", stroke: "currentColor", stroke_width: "1.6" }
                    line { x1: "3", y1: "8", x2: "13", y2: "8", stroke: "currentColor", stroke_width: "1.6" }
                    line { x1: "3", y1: "11", x2: "13", y2: "11", stroke: "currentColor", stroke_width: "1.6" }
                }
            }
            div { class: "title-block",
                div { class: "app-title", dangerous_inner_html: title_html }
            }
        }

        Outlet::<Route> {}

        nav { class: "bottom-nav",
            Link {
                to: Route::Todo {},
                class: if active_tab == "todo" { "nav-tab active" } else { "nav-tab" },
                span { class: "ico",
                    svg { view_box: "0 0 24 24",
                        path { d: "M5 7h14M5 12h14M5 17h8", stroke: "currentColor", stroke_width: "1.6", fill: "none", stroke_linecap: "round" }
                    }
                }
                span { "Tasks" }
            }
            Link {
                to: Route::Calendar {},
                class: if active_tab == "calendar" { "nav-tab active" } else { "nav-tab" },
                span { class: "ico",
                    svg { view_box: "0 0 24 24",
                        rect { x: "3", y: "5", width: "18", height: "16", rx: "1", stroke: "currentColor", stroke_width: "1.6", fill: "none" }
                        line { x1: "3", y1: "10", x2: "21", y2: "10", stroke: "currentColor", stroke_width: "1.6" }
                        line { x1: "8", y1: "3", x2: "8", y2: "7", stroke: "currentColor", stroke_width: "1.6" }
                        line { x1: "16", y1: "3", x2: "16", y2: "7", stroke: "currentColor", stroke_width: "1.6" }
                    }
                }
                span { "Calendar" }
            }
            Link {
                to: Route::Stats {},
                class: if active_tab == "stats" { "nav-tab active" } else { "nav-tab" },
                span { class: "ico",
                    svg { view_box: "0 0 24 24",
                        line { x1: "5", y1: "20", x2: "5", y2: "14", stroke: "currentColor", stroke_width: "1.6" }
                        line { x1: "12", y1: "20", x2: "12", y2: "6", stroke: "currentColor", stroke_width: "1.6" }
                        line { x1: "19", y1: "20", x2: "19", y2: "11", stroke: "currentColor", stroke_width: "1.6" }
                    }
                }
                span { "Stats" }
            }
            Link {
                to: Route::Pomodoro {},
                class: if active_tab == "timer" { "nav-tab active" } else { "nav-tab" },
                span { class: "ico",
                    svg { view_box: "0 0 24 24",
                        circle { cx: "12", cy: "13", r: "7", stroke: "currentColor", stroke_width: "1.6", fill: "none" }
                        line { x1: "12", y1: "13", x2: "12", y2: "9", stroke: "currentColor", stroke_width: "1.6", stroke_linecap: "round" }
                        line { x1: "12", y1: "13", x2: "15", y2: "11", stroke: "currentColor", stroke_width: "1.6", stroke_linecap: "round" }
                        line { x1: "9", y1: "3", x2: "15", y2: "3", stroke: "currentColor", stroke_width: "1.6", stroke_linecap: "round" }
                    }
                }
                span { "Timer" }
            }
        }
    }
}
