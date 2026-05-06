use crate::{
    clients::auth_client::logout,
    presentation::views::auth_page::AuthPage,
    state::auth_state::AuthState,
    Route,
};
use dioxus::prelude::*;

#[component]
pub fn Layout() -> Element {
    let mut drawer_open: Signal<bool> = use_context_provider(|| Signal::new(false));

    let mut auth_state = use_context::<Signal<AuthState>>();

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
        Route::Todo {} | Route::Calendar {} | Route::Stats {} | Route::Pomodoro {} => "tasks",
        Route::Flashcards {} => "flashcards",
    };

    rsx! {
        div { class: "app-shell",
            div { class: "page-content",
                if auth_state.read().is_authenticated() {
                    Outlet::<Route> {}
                } else {
                    div {
                        AuthPage {  }
                    }
                }
            }

            div {
                class: if *drawer_open.read() { "drawer-overlay open" } else { "drawer-overlay" },
                onclick: move |_| drawer_open.set(false),
            }

            aside { class: if *drawer_open.read() { "side-drawer open" } else { "side-drawer" },
                div { class: "sd-head",
                    div { class: "sd-mark" }
                    div { class: "sd-name", "Focus", em { "Flow" } }
                    button {
                        class: "sd-close",
                        onclick: move |_| drawer_open.set(false),
                        svg { view_box: "0 0 16 16",
                            line { x1: "4", y1: "4", x2: "12", y2: "12", stroke: "currentColor", stroke_width: "1.6" }
                            line { x1: "12", y1: "4", x2: "4", y2: "12", stroke: "currentColor", stroke_width: "1.6" }
                        }
                    }
                }
                div { class: "sd-eyebrow", "// workspace" }
                nav { class: "sd-nav",
                    Link {
                        to: Route::Todo {},
                        class: if active_section == "tasks" { "sd-item active" } else { "sd-item" },
                        onclick: move |_| drawer_open.set(false),
                        span { class: "icon",
                            svg { view_box: "0 0 16 16",
                                path { d: "M3 4h10M3 8h10M3 12h6", stroke: "currentColor", stroke_width: "1.5", fill: "none" }
                            }
                        }
                        span { class: "label", "Tasks" }
                    }
                    Link {
                        to: Route::Flashcards {},
                        class: if active_section == "flashcards" { "sd-item active" } else { "sd-item" },
                        onclick: move |_| drawer_open.set(false),
                        span { class: "icon",
                            svg { view_box: "0 0 16 16",
                                rect { x: "2", y: "3", width: "10", height: "9", stroke: "currentColor", stroke_width: "1.5", fill: "none" }
                                rect { x: "4", y: "5", width: "10", height: "9", stroke: "currentColor", stroke_width: "1.5", fill: "none" }
                            }
                        }
                        span { class: "label", "Flashcards" }
                        span { class: "meta", "soon" }
                    }
                }
                div { class: "sd-foot",
                    div { class: "avatar", "FF" }
                    div { class: "who",
                        span { class: "n", "You" }
                        span { class: "e", "local · synced" }
                    }
                    button {
                        class: "sd-logout",
                        r#type: "button",
                        onclick: logout_handler,
                        svg { view_box: "0 0 16 16",
                            path { d: "M6 3H3a1 1 0 0 0-1 1v8a1 1 0 0 0 1 1h3", stroke: "currentColor", stroke_width: "1.5", fill: "none", stroke_linecap: "round" }
                            polyline { points: "10 11 13 8 10 5", stroke: "currentColor", stroke_width: "1.5", fill: "none", stroke_linecap: "round", stroke_linejoin: "round" }
                            line { x1: "13", y1: "8", x2: "6", y2: "8", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round" }
                        }
                    }
                }
            }
        }
    }
}
