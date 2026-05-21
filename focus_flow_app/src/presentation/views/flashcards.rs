use dioxus::prelude::*;

use crate::i18n::use_i18n;

#[component]
pub fn Flashcards() -> Element {
    let i18n = use_i18n();
    rsx! {
        div { class: "scroll",
            div { class: "flash-empty",
                div { class: "ico",
                    svg { view_box: "0 0 16 16",
                        rect { x: "2", y: "3", width: "10", height: "9", stroke: "currentColor", fill: "none", stroke_width: "1.5" }
                        rect { x: "4", y: "5", width: "10", height: "9", stroke: "currentColor", fill: "none", stroke_width: "1.5" }
                    }
                }
                h3 { "{i18n.read().t(\"flashcards.title_main\")}" em { "{i18n.read().t(\"flashcards.title_em\")}" } "{i18n.read().t(\"flashcards.title_end\")}" }
                p { "{i18n.read().t(\"flashcards.description\")}" }
            }
        }
    }
}
