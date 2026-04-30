use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DrawerProps {
    pub open: bool,
    pub on_close: EventHandler<()>,
}

#[component]
pub fn Drawer(props: DrawerProps) -> Element {
    let overlay_class = if props.open { "drawer-overlay open" } else { "drawer-overlay" };
    let drawer_class = if props.open { "side-drawer open" } else { "side-drawer" };
    rsx! {
        div { class: "{overlay_class}", onclick: move |_| props.on_close.call(()) }
        aside { class: "{drawer_class}" }
    }
}
