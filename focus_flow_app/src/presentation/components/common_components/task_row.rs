use dioxus::prelude::*;

use crate::model::task::Task;

#[derive(Props, Clone, PartialEq)]
pub struct TaskRowProps {
    pub task: Task,
    pub on_toggle: EventHandler<String>,
}

#[component]
pub fn TaskRow(props: TaskRowProps) -> Element {
    let task = &props.task;
    let row_class = if task.done {
        "todo-row done"
    } else {
        "todo-row"
    };
    let due_label = task.due.to_string();
    let due_class = "";
    let id = task.id.clone();

    rsx! {
        div {
            class: "{row_class}",
            style: "--cat: {task.cat_color}",
            div {
                class: "todo-check",
                onclick: move |_| props.on_toggle.call(id.clone()),
                svg { view_box: "0 0 16 16", class: "todo-check-icon",
                    path { d: "M3 8l3 3 7-7" }
                }
            }
            div { class: "todo-body",
                div { class: "todo-title", "{task.title}" }
                div { class: "todo-sub",
                    span { class: "todo-cat", "@{task.cat}" }
                    span { "·" }
                    span { class: "{due_class}", "{due_label}" }
                }
            }
            button { class: "todo-more", "⋯" }
        }
    }
}
