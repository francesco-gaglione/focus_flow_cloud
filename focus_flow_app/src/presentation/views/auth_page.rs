use dioxus::prelude::*;

use crate::{
    components::{
        button::{Button, ButtonVariant},
        input::Input,
    },
    i18n::use_i18n,
    services::storage,
    state::app_state::AppState,
    use_cases::auth::{login_uc::login_uc, update_base_url_uc::update_base_url_uc},
};

const LABEL_CLS: &str = "font-mono text-xs font-medium tracking-[0.02em] uppercase text-subtle";
const CARD_CLS: &str = "w-full max-w-[340px] flex flex-col gap-5 p-6 bg-surface-card border border-border rounded-xl shadow-lg";

#[component]
pub fn AuthPage() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();
    let i18n = use_i18n();

    let mut server_url_input = use_signal(String::new);
    let mut username_input = use_signal(String::new);
    let mut password_input = use_signal(String::new);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut is_loading = use_signal(|| false);

    rsx! {
        div {
            class: "h-full flex flex-col items-center justify-center px-6 py-8 gap-6 bg-surface",
            style: "background-image: radial-gradient(circle, rgba(255,255,255,0.04) 1px, transparent 1px); background-size: 20px 20px;",

            div { class: "flex flex-col items-center gap-2",
                div { class: "font-mono text-xs tracking-[0.02em] uppercase text-subtle", "{i18n.read().t(\"auth.comment\")}" }
                div { class: "text-[44px] font-bold text-foreground tracking-[-0.04em] leading-none",
                    span { "{i18n.read().t(\"layout.focus\")}" }
                    em { class: "text-accent not-italic", "{i18n.read().t(\"layout.flow\")}" }
                }
            }

            if !app_state.read().is_server_url_set() {
                div { class: CARD_CLS,
                    p { class: "text-lg font-semibold text-foreground tracking-[-0.02em] leading-[1.3]",
                        "{i18n.read().t(\"auth.configure_title_main\")}"
                        em { class: "text-subtle font-normal not-italic", "{i18n.read().t(\"auth.configure_title_em\")}" }
                    }

                    div { class: "flex flex-col gap-1",
                        label { class: LABEL_CLS, "{i18n.read().t(\"auth.server_url_label\")}" }
                        Input {
                            r#type: "url",
                            placeholder: i18n.read().t("auth.server_url_placeholder"),
                            value: i18n.read().t("auth.server_url_placeholder"),
                            oninput: move |e: FormEvent| server_url_input.set(e.value()),
                        }
                    }

                    Button {
                        variant: ButtonVariant::Primary,
                        style: "width: 100%; justify-content: center;",
                        onclick: move |_| {
                            let url = server_url_input.read().trim().to_string();
                            if !url.is_empty() {
                                storage::set_item("server_url", &url);
                                app_state.write().set_server_url(url.clone());
                                let _ = update_base_url_uc(&url);
                            }
                        },
                        "{i18n.read().t(\"auth.continue\")}"
                        svg { view_box: "0 0 16 16", width: "14", height: "14",
                            path { d: "M6 4l4 4-4 4", stroke: "currentColor", fill: "none", stroke_width: "1.6" }
                        }
                    }
                }
            } else {
                div { class: CARD_CLS,
                    p { class: "text-lg font-semibold text-foreground tracking-[-0.02em] leading-[1.3]",
                        "{i18n.read().t(\"auth.sign_in_title_main\")}"
                        em { class: "text-subtle font-normal not-italic", "{i18n.read().t(\"auth.sign_in_title_em\")}" }
                    }

                    div { class: "flex flex-col gap-4",
                        div { class: "flex flex-col gap-1",
                            label { class: LABEL_CLS, "{i18n.read().t(\"auth.username_label\")}" }
                            Input {
                                r#type: "text",
                                placeholder: i18n.read().t("auth.username_placeholder"),
                                value: "{username_input}",
                                oninput: move |e: FormEvent| username_input.set(e.value()),
                            }
                        }
                        div { class: "flex flex-col gap-1",
                            label { class: LABEL_CLS, "{i18n.read().t(\"auth.password_label\")}" }
                            Input {
                                r#type: "password",
                                placeholder: i18n.read().t("auth.password_placeholder"),
                                value: "{password_input}",
                                oninput: move |e: FormEvent| password_input.set(e.value()),
                            }
                        }
                    }

                    if let Some(err) = error_msg.read().as_ref() {
                        div { class: "flex items-start gap-2 px-4 py-3 bg-[#2a1314] border border-danger rounded-sm text-[#ff8589] text-sm",
                            svg { view_box: "0 0 16 16", width: "13", height: "13",
                                path { d: "M8 3v5M8 10v1", stroke: "currentColor", fill: "none", stroke_width: "1.6" }
                                circle { cx: "8", cy: "8", r: "6", stroke: "currentColor", fill: "none", stroke_width: "1.4" }
                            }
                            span { "{err}" }
                        }
                    }

                    Button {
                        variant: ButtonVariant::Primary,
                        style: "width: 100%; justify-content: center;",
                        disabled: *is_loading.read(),
                        onclick: move |_| {
                            let username = username_input.read().clone();
                            let password = password_input.read().clone();
                            if username.trim().is_empty() || password.is_empty() {
                                error_msg.set(Some(i18n.read().t("auth.error_required")));
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
                        if *is_loading.read() { "{i18n.read().t(\"auth.signing_in\")}" } else { "{i18n.read().t(\"auth.sign_in_btn\")}" }
                        if !*is_loading.read() {
                            svg { view_box: "0 0 16 16", width: "14", height: "14",
                                path { d: "M6 4l4 4-4 4", stroke: "currentColor", fill: "none", stroke_width: "1.6" }
                            }
                        }
                    }
                }

                div { class: "flex items-center gap-3 max-w-[340px] w-full",
                    span { class: "font-mono text-xs text-subtle flex-1 min-w-0 overflow-hidden text-ellipsis whitespace-nowrap",
                        "↗ {app_state.read().server_url().unwrap_or(\"\")}"
                    }
                    button {
                        class: "bg-transparent border-0 text-accent font-mono text-xs tracking-[0.02em] uppercase cursor-pointer py-1 shrink-0 transition-opacity duration-fast ease-tech active:opacity-60",
                        onclick: move |_| {
                            storage::remove_item("server_url");
                            app_state.write().clear_server_url();
                            error_msg.set(None);
                            is_loading.set(false);
                        },
                        "{i18n.read().t(\"auth.change\")}"
                    }
                }
            }
        }
    }
}
