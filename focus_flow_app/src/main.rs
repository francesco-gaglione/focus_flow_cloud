use dioxus::prelude::*;

mod model;
mod presentation;
mod services;
mod state;

use presentation::views::{
    Calendar, Flashcards, FlashcardsLayout, Layout, Stats, TasksLayout, Todo,
};

use crate::{
    services::{api_client::ApiClient, storage::get_item},
    state::{app_state::AppState, auth_state::AuthState},
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Layout)]
        #[layout(TasksLayout)]
            #[route("/")]        Todo {},
            #[route("/calendar")] Calendar {},
            #[route("/stats")]   Stats {},
        #[end_layout]
        #[layout(FlashcardsLayout)]
            #[route("/cards")]   Flashcards {},
}

const CSS_TOKENS: Asset      = asset!("/assets/styling/tokens.css");
const CSS_BASE: Asset        = asset!("/assets/styling/base.css");
const CSS_COMPONENTS: Asset  = asset!("/assets/styling/components.css");
const CSS_LAYOUT: Asset      = asset!("/assets/styling/layout.css");
const CSS_SHEET: Asset       = asset!("/assets/styling/sheet.css");
const CSS_TASKS: Asset       = asset!("/assets/styling/views/tasks.css");
const CSS_CALENDAR: Asset    = asset!("/assets/styling/views/calendar.css");
const CSS_STATS: Asset       = asset!("/assets/styling/views/stats.css");
const CSS_AUTH: Asset        = asset!("/assets/styling/views/auth.css");
const CSS_FLASHCARDS: Asset  = asset!("/assets/styling/views/flashcards.css");

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

    rsx! {
        document::Link { rel: "stylesheet", href: CSS_TOKENS }
        document::Link { rel: "stylesheet", href: CSS_BASE }
        document::Link { rel: "stylesheet", href: CSS_COMPONENTS }
        document::Link { rel: "stylesheet", href: CSS_LAYOUT }
        document::Link { rel: "stylesheet", href: CSS_SHEET }
        document::Link { rel: "stylesheet", href: CSS_TASKS }
        document::Link { rel: "stylesheet", href: CSS_CALENDAR }
        document::Link { rel: "stylesheet", href: CSS_STATS }
        document::Link { rel: "stylesheet", href: CSS_AUTH }
        document::Link { rel: "stylesheet", href: CSS_FLASHCARDS }
        Router::<Route> {}
    }
}
