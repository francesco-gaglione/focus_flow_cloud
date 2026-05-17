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
    let cls = if props.active {
        "shrink-0 appearance-none bg-accent-soft border border-accent rounded-full text-accent font-mono text-xs font-medium tracking-[0.02em] uppercase px-3 h-7 cursor-pointer inline-flex items-center gap-1.5 transition-[background,border-color,color] duration-fast ease-tech"
    } else {
        "shrink-0 appearance-none bg-surface-card border border-border rounded-full text-subtle font-mono text-xs font-medium tracking-[0.02em] uppercase px-3 h-7 cursor-pointer inline-flex items-center gap-1.5 transition-[background,border-color,color] duration-fast ease-tech hover:bg-gray-200 hover:border-border-strong hover:text-muted active:bg-gray-300"
    };

    rsx! {
        button {
            class: "{cls}",
            onclick: move |_| props.onclick.call(()),
            "{props.label}"
            if let Some(n) = props.count {
                span {
                    class: if props.active {
                        "bg-accent text-white px-[5px] py-[1px] text-[10px] rounded-full"
                    } else {
                        "bg-gray-300 text-subtle px-[5px] py-[1px] text-[10px] rounded-full"
                    },
                    "{n}"
                }
            }
        }
    }
}
