use dioxus::prelude::*;

#[component]
pub fn Flashcards() -> Element {
    rsx! {
        div { class: "scroll",
            div { class: "flash-empty",
                div { class: "ico",
                    svg { view_box: "0 0 16 16",
                        rect { x: "2", y: "3", width: "10", height: "9", stroke: "currentColor", fill: "none", stroke_width: "1.5" }
                        rect { x: "4", y: "5", width: "10", height: "9", stroke: "currentColor", fill: "none", stroke_width: "1.5" }
                    }
                }
                h3 { "Tomorrow's ", em { "memory" }, ", today." }
                p { "Spaced repetition, gentle review queues, and decks that don't shame you for missed days. Coming next." }
            }
        }
    }
}
