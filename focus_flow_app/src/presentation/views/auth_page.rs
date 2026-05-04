use dioxus::prelude::*;

use crate::{
    services::storage,
    state::app_state::AppState,
    use_cases::auth::{login_uc::login_uc, update_base_url_uc::update_base_url_uc},
};

#[component]
pub fn AuthPage() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();

    let mut server_url_input = use_signal(|| String::new());
    let mut username_input = use_signal(|| String::new());
    let mut password_input = use_signal(|| String::new());
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut is_loading = use_signal(|| false);

    rsx! {
        div { class: "auth-screen",

            div { class: "auth-top",
                div { class: "auth-eyebrow", "// auth" }
                div { class: "auth-brand",
                    span { "Focus" }
                    em { "Flow" }
                }
            }

            if !app_state.read().is_server_url_set() {
                div { class: "auth-card",
                    p { class: "auth-card-sub", "configure ", em { "server." } }

                    div { class: "auth-field",
                        label { class: "auth-label", "Server URL" }
                        input {
                            class: "auth-input",
                            r#type: "url",
                            placeholder: "http://192.168.1.x:8080",
                            value: "http://192.168.1.x:8080",
                            oninput: move |e| server_url_input.set(e.value()),
                        }
                    }

                    button {
                        class: "auth-btn",
                        onclick: move |_| {
                            let url = server_url_input.read().trim().to_string();
                            if !url.is_empty() {
                                storage::set_item("server_url", &url);
                                app_state.write().set_server_url(url.clone());
                                update_base_url_uc(&url); //TODO handle error
                            }
                        },
                        "Continue"
                        svg { view_box: "0 0 16 16", width: "14", height: "14",
                            path { d: "M6 4l4 4-4 4", stroke: "currentColor", fill: "none", stroke_width: "1.6" }
                        }
                    }
                }
            } else {
                div { class: "auth-card",
                    p { class: "auth-card-sub", "sign in to ", em { "continue." } }

                    div { class: "auth-fields",
                        div { class: "auth-field",
                            label { class: "auth-label", "Username" }
                            input {
                                class: "auth-input",
                                r#type: "text",
                                placeholder: "your username",
                                value: "{username_input}",
                                oninput: move |e| username_input.set(e.value()),
                            }
                        }
                        div { class: "auth-field",
                            label { class: "auth-label", "Password" }
                            input {
                                class: "auth-input",
                                r#type: "password",
                                placeholder: "••••••••",
                                value: "{password_input}",
                                oninput: move |e| password_input.set(e.value()),
                            }
                        }
                    }

                    if let Some(err) = error_msg.read().as_ref() {
                        div { class: "auth-error",
                            svg { view_box: "0 0 16 16", width: "13", height: "13",
                                path { d: "M8 3v5M8 10v1", stroke: "currentColor", fill: "none", stroke_width: "1.6" }
                                circle { cx: "8", cy: "8", r: "6", stroke: "currentColor", fill: "none", stroke_width: "1.4" }
                            }
                            span { "{err}" }
                        }
                    }

                    button {
                        class: if *is_loading.read() { "auth-btn loading" } else { "auth-btn" },
                        disabled: *is_loading.read(),
                        onclick: move |_| {
                            let username = username_input.read().clone();
                            let password = password_input.read().clone();
                            if username.trim().is_empty() || password.is_empty() {
                                error_msg.set(Some("Username and password required.".into()));
                                return;
                            }
                            error_msg.set(None);
                            is_loading.set(true);
                            spawn(async move {
                                match login_uc(&username, &password).await {
                                    Ok(_) => {}
                                    Err(e) => {
                                        error!("{}", e.to_string());
                                        error_msg.set(Some(e.to_string()));
                                        is_loading.set(false);
                                    }
                                }
                            });
                        },
                        if *is_loading.read() { "Signing in…" } else { "Sign in" }
                        if !*is_loading.read() {
                            svg { view_box: "0 0 16 16", width: "14", height: "14",
                                path { d: "M6 4l4 4-4 4", stroke: "currentColor", fill: "none", stroke_width: "1.6" }
                            }
                        }
                    }
                }

                div { class: "auth-server-row",
                    span { class: "auth-server-url",
                        "↗ {app_state.read().server_url().unwrap_or(\"\")}"
                    }
                    button {
                        class: "auth-change-btn",
                        onclick: move |_| {
                            storage::remove_item("server_url");
                            app_state.write().clear_server_url();
                            error_msg.set(None);
                            is_loading.set(false);
                        },
                        "change"
                    }
                }
            }
        }
    }
}
