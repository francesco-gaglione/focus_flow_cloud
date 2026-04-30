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

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const FONTS: &str = "https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500;600&family=Instrument+Serif:ital@0;1&display=swap";

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
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com" }
        document::Link { rel: "stylesheet", href: FONTS }
        Router::<Route> {}
    }
}
