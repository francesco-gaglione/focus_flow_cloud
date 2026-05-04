use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BottomSheetProps {
    pub show: bool,
    pub title: String,
    pub on_close: EventHandler<()>,
    pub children: Element,
}

#[component]
pub fn BottomSheet(props: BottomSheetProps) -> Element {
    if !props.show {
        return rsx! {};
    }

    rsx! {
        div {
            class: "sheet-overlay",
            onclick: move |_| props.on_close.call(()),
        }
        div { class: "bottom-sheet",
            div { class: "sheet-handle" }
            div { class: "sheet-head",
                span { class: "sheet-title", "{props.title}" }
                button {
                    class: "sheet-close",
                    onclick: move |_| props.on_close.call(()),
                    svg { view_box: "0 0 16 16",
                        line { x1: "4", y1: "4", x2: "12", y2: "12", stroke: "currentColor", stroke_width: "1.6" }
                        line { x1: "12", y1: "4", x2: "4", y2: "12", stroke: "currentColor", stroke_width: "1.6" }
                    }
                }
            }
            {props.children}
        }
    }
}
