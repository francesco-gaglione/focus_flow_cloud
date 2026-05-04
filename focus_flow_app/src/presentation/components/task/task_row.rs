use dioxus::prelude::*;

use crate::use_cases::tasks::task_list_uc::TodoTask;

#[derive(Props, Clone, PartialEq)]
pub struct TaskRowProps {
    pub task: TodoTask,
    pub on_toggle: EventHandler<String>,
    #[props(optional)]
    pub on_delete: Option<EventHandler<String>>,
}

#[component]
pub fn TaskRow(props: TaskRowProps) -> Element {
    let task = &props.task;
    let mut expanded = use_signal(|| false);
    let mut show_menu = use_signal(|| false);
    let mut menu_x: Signal<f64> = use_signal(|| 0.0);
    let mut menu_y: Signal<f64> = use_signal(|| 0.0);

    let row_class = if task.done {
        "todo-row done"
    } else {
        "todo-row"
    };
    let due_label = task.due.to_string();
    let cat_color = task.cat_color.as_deref().unwrap_or("white").to_string();
    let id = task.id.clone();
    let delete_id = task.id.clone();
    let on_delete = props.on_delete.clone();
    let subtask_total = task.subtasks.len();
    let subtask_done = task.subtasks.iter().filter(|s| s.is_completed).count();
    let has_subtasks = subtask_total > 0;
    let done = task.done;

    let wrap_class = if has_subtasks && *expanded.read() {
        "todo-row-wrap expanded"
    } else {
        "todo-row-wrap"
    };
    let expand_btn_class = if *expanded.read() {
        "todo-expand-btn open"
    } else {
        "todo-expand-btn"
    };

    rsx! {
        div {
            class: "{wrap_class}",
            oncontextmenu: move |e| {
                e.prevent_default();
                let p = e.client_coordinates();
                menu_x.set(p.x);
                menu_y.set(p.y);
                show_menu.set(true);
            },
            div {
                class: "{row_class}",
                style: "--cat: {cat_color}",
                div {
                    class: "todo-check",
                    onclick: move |_| props.on_toggle.call(id.clone()),
                    svg { view_box: "0 0 16 16", class: "todo-check-icon",
                        path { d: "M3 8l3 3 7-7" }
                    }
                }
                div { class: "todo-body",
                    div { class: "todo-title", "{task.title}" }
                    if task.description.is_some() {
                        div { class: "todo-description", "{task.description.clone().unwrap()}" }
                    }
                    div { class: "todo-sub",
                        if let Some(cat) = task.cat.as_deref() {
                            span { class: "todo-cat", "@{cat}" }
                        }
                        span { "·" }
                        span { "{due_label}" }
                        if has_subtasks {
                            span { "·" }
                            span { class: "todo-subtask-ct", "{subtask_done}/{subtask_total}" }
                        }
                    }
                }
                if has_subtasks {
                    button {
                        class: "{expand_btn_class}",
                        onclick: move |_| {
                            let cur = *expanded.read();
                            expanded.set(!cur);
                        },
                        svg { view_box: "0 0 16 16",
                            polyline {
                                points: "4 6 8 10 12 6",
                                stroke: "currentColor",
                                stroke_width: "1.6",
                                fill: "none",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                            }
                        }
                    }
                }
            }
            if has_subtasks && *expanded.read() {
                div {
                    class: "todo-subtask-list",
                    style: "--cat: {cat_color}",
                    for subtask in props.task.subtasks.iter() {
                        div { class: "todo-subtask-item",
                            div {
                                class: if subtask.is_completed { "todo-subtask-check done" } else { "todo-subtask-check" },
                                if subtask.is_completed {
                                    svg { view_box: "0 0 16 16",
                                        path { d: "M3 8l3 3 7-7", stroke: "currentColor", stroke_width: "2.5", fill: "none", stroke_linecap: "round", stroke_linejoin: "round" }
                                    }
                                }
                            }
                            span {
                                class: if subtask.is_completed { "todo-subtask-title done" } else { "todo-subtask-title" },
                                "{subtask.title}"
                            }
                        }
                    }
                }
            }
        }

        if *show_menu.read() {
            div {
                class: "ctx-overlay",
                onclick: move |_| show_menu.set(false),
                oncontextmenu: move |e| {
                    e.prevent_default();
                    show_menu.set(false);
                },
            }
            div {
                class: "ctx-menu",
                style: "left: {menu_x}px; top: {menu_y}px;",
                button {
                    class: "ctx-item danger",
                    onclick: move |_| {
                        if let Some(cb) = &on_delete {
                            cb.call(delete_id.clone());
                        }
                        show_menu.set(false);
                    },
                    svg { view_box: "0 0 16 16", class: "ctx-item-icon",
                        polyline { points: "3 6 13 6", stroke: "currentColor", stroke_width: "1.6", fill: "none", stroke_linecap: "round" }
                        path { d: "M5 6V4h6v2M4 6v8a1 1 0 001 1h6a1 1 0 001-1V6", stroke: "currentColor", stroke_width: "1.6", fill: "none", stroke_linecap: "round", stroke_linejoin: "round" }
                    }
                    "Delete"
                }
            }
        }
    }
}
