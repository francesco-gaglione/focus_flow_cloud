use dioxus::prelude::*;
use shared::task::TaskPriority;

use crate::use_cases::tasks::task_list_uc::TodoTask;

#[derive(Props, Clone, PartialEq)]
pub struct TaskRowProps {
    pub task: TodoTask,
    pub on_toggle: EventHandler<(String, bool)>,
    pub on_subtask_toggle: EventHandler<(String, String, bool)>,
    #[props(optional)]
    pub on_delete: Option<EventHandler<String>>,
    #[props(optional)]
    pub on_start_timer: Option<EventHandler<(String, String)>>,
    #[props(optional)]
    pub on_add_subtask: Option<EventHandler<(String, String)>>,
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
    let timer_id = task.id.clone();
    let timer_title = task.title.clone();
    let on_delete = props.on_delete.clone();
    let on_start_timer = props.on_start_timer.clone();
    let on_add_subtask = props.on_add_subtask.clone();
    let has_timer = on_start_timer.is_some();
    let has_add_subtask = on_add_subtask.is_some();
    let add_task_id = task.id.clone();
    let priority_label = task.priority.as_ref().map(|p| match p {
        TaskPriority::Low => ("low", "LOW"),
        TaskPriority::Medium => ("medium", "MED"),
        TaskPriority::High => ("high", "HIGH"),
        TaskPriority::Urgent => ("urgent", "URGENT"),
    });
    let mut new_subtask_title = use_signal(String::new);
    let subtask_total = task.subtasks.len();
    let subtask_done = task.subtasks.iter().filter(|s| s.is_completed).count();
    let has_subtasks = subtask_total > 0;
    let done = task.done;

    struct SubtaskItem {
        task_id: String,
        subtask_id: String,
        is_completed: bool,
        title: String,
    }
    let subtask_items: Vec<SubtaskItem> = task
        .subtasks
        .iter()
        .map(|s| SubtaskItem {
            task_id: task.id.clone(),
            subtask_id: s.id.clone(),
            is_completed: s.is_completed,
            title: s.title.clone(),
        })
        .collect();

    let can_expand = has_subtasks || has_add_subtask;
    let wrap_class = if can_expand && *expanded.read() {
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
                    onclick: move |_| props.on_toggle.call((id.clone(), !done)),
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
                        if let Some((lvl, lbl)) = priority_label {
                            span { class: "todo-priority todo-priority-{lvl}", "{lbl}" }
                            span { "·" }
                        }
                        if let Some(cat) = task.cat.as_deref() {
                            span { class: "todo-cat", "@{cat}" }
                        }
                        span { "·" }
                        span { "{due_label}" }
                        if has_subtasks || has_add_subtask {
                            span { "·" }
                            span { class: "todo-subtask-ct", "{subtask_done}/{subtask_total}" }
                        }
                    }
                }
                if can_expand {
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
            if can_expand && *expanded.read() {
                div {
                    class: "todo-subtask-list",
                    style: "--cat: {cat_color}",
                    for sub in subtask_items {
                        div { class: "todo-subtask-item",
                            div {
                                class: if sub.is_completed { "todo-subtask-check done" } else { "todo-subtask-check" },
                                onclick: move |_| {
                                    props.on_subtask_toggle.call((sub.task_id.clone(), sub.subtask_id.clone(), !sub.is_completed))
                                },
                                if sub.is_completed {
                                    svg { view_box: "0 0 16 16",
                                        path { d: "M3 8l3 3 7-7", stroke: "currentColor", stroke_width: "2.5", fill: "none", stroke_linecap: "round", stroke_linejoin: "round" }
                                    }
                                }
                            }
                            span {
                                class: if sub.is_completed { "todo-subtask-title done" } else { "todo-subtask-title" },
                                "{sub.title}"
                            }
                        }
                    }
                    if on_add_subtask.is_some() {
                        div { class: "todo-subtask-item todo-subtask-add-row",
                            input {
                                class: "todo-subtask-input",
                                r#type: "text",
                                placeholder: "Add subtask…",
                                value: "{new_subtask_title}",
                                oninput: move |e| new_subtask_title.set(e.value()),
                                onkeydown: move |e| {
                                    if e.key() == Key::Enter {
                                        let title = new_subtask_title.read().trim().to_string();
                                        if !title.is_empty() {
                                            if let Some(ref cb) = on_add_subtask {
                                                cb.call((add_task_id.clone(), title));
                                                new_subtask_title.set(String::new());
                                            }
                                        }
                                    }
                                },
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
                if has_timer {
                    button {
                        class: "ctx-item",
                        onclick: move |_| {
                            if let Some(ref cb) = on_start_timer {
                                cb.call((timer_id.clone(), timer_title.clone()));
                            }
                            show_menu.set(false);
                        },
                        svg { view_box: "0 0 16 16", class: "ctx-item-icon",
                            circle { cx: "8", cy: "9", r: "5", stroke: "currentColor", stroke_width: "1.6", fill: "none" }
                            line { x1: "8", y1: "9", x2: "8", y2: "6", stroke: "currentColor", stroke_width: "1.6", stroke_linecap: "round" }
                            line { x1: "8", y1: "9", x2: "10", y2: "8", stroke: "currentColor", stroke_width: "1.6", stroke_linecap: "round" }
                            line { x1: "5", y1: "2", x2: "11", y2: "2", stroke: "currentColor", stroke_width: "1.6", stroke_linecap: "round" }
                        }
                        "Start timer"
                    }
                }
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
