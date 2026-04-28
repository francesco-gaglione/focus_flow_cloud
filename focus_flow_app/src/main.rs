use dioxus::prelude::*;

mod components;
mod views;

use views::{Calendar, Flashcards, FlashcardsLayout, Layout, Stats, TasksLayout, Todo};

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
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com" }
        document::Link { rel: "stylesheet", href: FONTS }
        Router::<Route> {}
    }
}
