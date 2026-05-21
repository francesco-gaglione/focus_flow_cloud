#![allow(dead_code)]
use dioxus::prelude::*;

mod clients;
mod components;
mod i18n;
mod presentation;
mod services;
mod state;
mod use_cases;

use presentation::views::{
    Calendar, Categories, Flashcards, FlashcardsLayout, Layout, Pomodoro, Stats, TasksLayout, Todo,
};

use crate::{
    clients::http_client::ApiClient,
    components::toast::ToastProvider,
    i18n::{load_locale, I18n},
    services::storage::get_item,
    state::{app_state::AppState, auth_state::AuthState},
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Layout)]
        #[layout(TasksLayout)]
            #[route("/")]              Todo {},
            #[route("/calendar")]      Calendar {},
            #[route("/stats")]         Stats {},
            #[route("/timer")]         Pomodoro {},
            #[route("/categories")]    Categories {},
        #[end_layout]
        #[layout(FlashcardsLayout)]
            #[route("/cards")]   Flashcards {},
}

const CSS_TAILWIND: Asset = asset!("/assets/tailwind.css");
const CSS_DX_THEME: Asset = asset!("/assets/dx-components-theme.css");
const CSS_COMPONENTS: Asset = asset!("/assets/styling/components.css");
const CSS_LAYOUT: Asset = asset!("/assets/styling/layout.css");
const CSS_TASKS: Asset = asset!("/assets/styling/views/tasks.css");
const CSS_CALENDAR: Asset = asset!("/assets/styling/views/calendar.css");
const CSS_STATS: Asset = asset!("/assets/styling/views/stats.css");
const CSS_FLASHCARDS: Asset = asset!("/assets/styling/views/flashcards.css");
const CSS_POMODORO: Asset = asset!("/assets/styling/views/pomodoro.css");

fn main() {
    dioxus_std::set_dir!();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let stored_server_url = get_item("server_url");
    let stored_auth_token = get_item("auth_token");
    let stored_refresh_token = get_item("refresh_token");

    let mut app_state = AppState::default();
    if let Some(ref url) = stored_server_url {
        app_state.set_server_url(url.clone());
    }

    let mut auth_state = AuthState::default();
    auth_state.set_auth_token(stored_auth_token);
    auth_state.set_refresh_token(stored_refresh_token);

    let base_url = stored_server_url
        .as_deref()
        .unwrap_or("http://192.168.1.135:8080");

    let mut api_client = ApiClient::new(base_url);
    if let Some(token) = auth_state.auth_token() {
        api_client.set_auth_token(Some(token.to_string()));
    }

    use_context_provider(|| Signal::new(auth_state));
    use_context_provider(|| Signal::new(app_state));
    use_context_provider(|| Signal::new(api_client));
    use_context_provider(|| Signal::new(Option::<(String, String)>::None)); // (task_id, task_title) for timer
    use_context_provider(|| Signal::new(I18n::new(load_locale())));

    rsx! {
        document::Link { rel: "stylesheet", href: CSS_TAILWIND }
        document::Link { rel: "stylesheet", href: CSS_DX_THEME }
        document::Link { rel: "stylesheet", href: CSS_COMPONENTS }
        document::Link { rel: "stylesheet", href: CSS_LAYOUT }
        document::Link { rel: "stylesheet", href: CSS_TASKS }
        document::Link { rel: "stylesheet", href: CSS_CALENDAR }
        document::Link { rel: "stylesheet", href: CSS_STATS }
        document::Link { rel: "stylesheet", href: CSS_FLASHCARDS }
        document::Link { rel: "stylesheet", href: CSS_POMODORO }
        ToastProvider{
            Router::<Route> {}
        }
    }
}
