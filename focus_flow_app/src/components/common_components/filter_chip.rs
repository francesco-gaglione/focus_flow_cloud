use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FilterChipProps {
    pub label: &'static str,
    pub count: Option<usize>,
    pub active: bool,
    pub onclick: EventHandler<()>,
}

#[component]
pub fn FilterChip(props: FilterChipProps) -> Element {
    let cls = if props.active { "chip active" } else { "chip" };
    rsx! {
        button {
            class: "{cls}",
            onclick: move |_| props.onclick.call(()),
            "{props.label}"
            if let Some(n) = props.count {
                span { class: "count", "{n}" }
            }
        }
    }
}
