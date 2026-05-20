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
            class: "fixed inset-0 bg-black/60 backdrop-blur-[4px] z-40",
            onclick: move |_| props.on_close.call(()),
        }
        div { class: "fixed bottom-0 left-0 right-0 z-[41] bg-surface-raised border border-border border-b-0 rounded-t-xl pb-11 animate-sheet-in",
            onclick: move |e| e.stop_propagation(),
            div { class: "w-9 h-1 bg-border-strong rounded-full mx-auto mt-[14px]" }
            div { class: "flex items-center justify-between px-5 py-4 border-b border-border",
                span { class: "text-xl font-bold text-foreground tracking-[-0.025em]", "{props.title}" }
                button {
                    class: "size-8 bg-surface-card border border-border rounded-sm text-subtle grid place-items-center cursor-pointer transition-[background,color] duration-fast ease-tech hover:bg-gray-200 hover:text-foreground",
                    onclick: move |_| props.on_close.call(()),
                    svg { view_box: "0 0 16 16", width: "14", height: "14", stroke: "currentColor", fill: "none", stroke_width: "1.6",
                        line { x1: "4", y1: "4", x2: "12", y2: "12" }
                        line { x1: "12", y1: "4", x2: "4", y2: "12" }
                    }
                }
            }
            {props.children}
        }
    }
}
