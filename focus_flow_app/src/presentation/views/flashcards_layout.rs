use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn FlashcardsLayout() -> Element {
    let mut drawer_open = use_context::<Signal<bool>>();

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
                div { class: "app-crumb",
                    span { "Flashcards" }
                    span { class: "sep", "/" }
                    span { "Decks" }
                }
                div { class: "app-title", dangerous_inner_html: "Memory, <em>tomorrow</em>." }
            }
            button { class: "icon-btn",
                svg { view_box: "0 0 16 16",
                    circle { cx: "7", cy: "7", r: "4.5", stroke: "currentColor", stroke_width: "1.6", fill: "none" }
                    line { x1: "10.5", y1: "10.5", x2: "13.5", y2: "13.5", stroke: "currentColor", stroke_width: "1.6" }
                }
            }
        }

        Outlet::<Route> {}

        nav { class: "bottom-nav bottom-nav-3",
            button { class: "nav-tab active",
                span { class: "ico",
                    svg { view_box: "0 0 24 24",
                        rect { x: "3", y: "5", width: "14", height: "13", rx: "1", stroke: "currentColor", stroke_width: "1.6", fill: "none" }
                        rect { x: "7", y: "9", width: "14", height: "13", rx: "1", stroke: "currentColor", stroke_width: "1.6", fill: "none" }
                    }
                }
                span { "Decks" }
            }
            button { class: "nav-tab",
                span { class: "ico",
                    svg { view_box: "0 0 24 24",
                        path { d: "M12 2L2 7l10 5 10-5-10-5z", stroke: "currentColor", stroke_width: "1.6", fill: "none" }
                        path { d: "M2 17l10 5 10-5", stroke: "currentColor", stroke_width: "1.6", fill: "none" }
                        path { d: "M2 12l10 5 10-5", stroke: "currentColor", stroke_width: "1.6", fill: "none" }
                    }
                }
                span { "Practice" }
            }
            button { class: "nav-tab",
                span { class: "ico",
                    svg { view_box: "0 0 24 24",
                        path { d: "M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10z", stroke: "currentColor", stroke_width: "1.6", fill: "none" }
                        path { d: "M12 8v4l3 3", stroke: "currentColor", stroke_width: "1.6" }
                    }
                }
                span { "Review" }
            }
        }
    }
}
