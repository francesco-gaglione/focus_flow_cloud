use dioxus::prelude::*;
use shared::task::TaskPriority;

use crate::{
    components::progress::{Progress, ProgressIndicator},
    use_cases::tasks::task_list_uc::{TaskDue, TaskSchedule, TodoTask},
};

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
    #[props(optional)]
    pub on_priority_change: Option<EventHandler<(String, Option<TaskPriority>)>>,
    #[props(optional)]
    pub on_due_date_change: Option<EventHandler<(String, String)>>,
}

#[component]
pub fn TaskRow(props: TaskRowProps) -> Element {
    let task = &props.task;

    // ── sheet contexts (provided by parent view) ──────────────────────────
    let prio_sheet_ctx =
        dioxus::core::try_consume_context::<Signal<Option<(String, Option<TaskPriority>)>>>();
    let date_sheet_ctx =
        dioxus::core::try_consume_context::<Signal<Option<(String, TaskSchedule)>>>();

    // ── menu state ───────────────────────────────────────────────────────
    let mut expanded = use_signal(|| false);
    let mut show_menu = use_signal(|| false);
    let mut menu_x: Signal<f64> = use_signal(|| 0.0);
    let mut menu_y: Signal<f64> = use_signal(|| 0.0);

    // ── derived display values ───────────────────────────────────────────
    let (due_mod, is_overdue, is_today) = match &task.due {
        TaskDue::Overdue(_) => ("overdue", true, false),
        TaskDue::Today(_) => ("today", false, true),
        _ => ("", false, false),
    };
    let due_class = format!("todo-due {}", due_mod);

    let cat_color = task.cat_color.as_deref().unwrap_or("#888").to_string();
    let stripe_color = match task.priority {
        Some(TaskPriority::Low) => "#6b7280".to_string(),
        Some(TaskPriority::Medium) => "#d97706".to_string(),
        Some(TaskPriority::High) => "#ef4444".to_string(),
        Some(TaskPriority::Urgent) => "#7c3aed".to_string(),
        None => cat_color.clone(),
    };

    let row_class = {
        let mut c = if task.done {
            "todo-row done"
        } else {
            "todo-row"
        }
        .to_string();
        if !task.done {
            if is_overdue {
                c.push_str(" overdue-row");
            } else if is_today {
                c.push_str(" today-row");
            }
        }
        c
    };

    let (p_lvl, p_lbl) = match task.priority {
        Some(TaskPriority::Low) => ("low", "LOW"),
        Some(TaskPriority::Medium) => ("medium", "MED"),
        Some(TaskPriority::High) => ("high", "HIGH"),
        Some(TaskPriority::Urgent) => ("urgent", "URGENT"),
        None => ("none", "—"),
    };

    let subtask_total = task.subtasks.len();
    let subtask_done = task.subtasks.iter().filter(|s| s.is_completed).count();
    let has_subtasks = subtask_total > 0;

    let done = task.done;
    let id = task.id.clone();
    let priority_task_id = task.id.clone();
    let date_task_id = task.id.clone();
    let delete_id = task.id.clone();
    let timer_id = task.id.clone();
    let timer_title = task.title.clone();
    let on_delete = props.on_delete;
    let on_start_timer = props.on_start_timer;
    let on_add_subtask = props.on_add_subtask;
    let current_priority = task.priority;
    let current_schedule = task.schedule.clone();
    let has_timer = on_start_timer.is_some();
    let has_add_subtask = on_add_subtask.is_some();
    let add_task_id = task.id.clone();
    let mut new_subtask_title = use_signal(String::new);

    let has_explicit_date = task.due_date_set;

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
                style: "--cat: {stripe_color}",

                div {
                    class: "todo-check",
                    onclick: move |_| props.on_toggle.call((id.clone(), !done)),
                    svg { view_box: "0 0 16 16", class: "todo-check-icon",
                        path { d: "M3 8l3 3 7-7" }
                    }
                }

                div { class: "todo-body",
                    div { class: "todo-title", "{task.title}" }
                    if let Some(desc) = task.description.as_deref() {
                        div { class: "todo-description", "{desc}" }
                    }

                    div { class: "todo-sub",
                        button {
                            class: "todo-priority todo-priority-{p_lvl}",
                            r#type: "button",
                            title: "Tap to change priority",
                            onclick: move |e| {
                                e.stop_propagation();
                                if let Some(mut ctx) = prio_sheet_ctx {
                                    ctx.set(Some((priority_task_id.clone(), current_priority)));
                                }
                            },
                            "{p_lbl}"
                        }

                        if let Some(cat) = task.cat.as_deref() {
                            {
                                let color = task.cat_color.as_deref().unwrap_or("#888");
                                rsx! {
                                    span {
                                        class: "todo-cat",
                                        style: "color:{color};background:color-mix(in srgb,{color} 15%,transparent);border-color:color-mix(in srgb,{color} 30%,transparent);",
                                        "@{cat}"
                                    }
                                }
                            }
                        }

                        {
                            let btn_class = if has_explicit_date {
                                format!("{} todo-date-wrap", due_class)
                            } else {
                                "todo-due todo-no-date todo-date-wrap".to_string()
                            };
                            let due_display = task.due.to_string();
                            let is_all_day = task.schedule.is_all_day();
                            let end_time = task.schedule.end_time_str();
                            rsx! {
                                button {
                                    class: "{btn_class}",
                                    r#type: "button",
                                    onclick: move |e| {
                                        e.stop_propagation();
                                        if let Some(mut ctx) = date_sheet_ctx {
                                            ctx.set(Some((date_task_id.clone(), current_schedule.clone())));
                                        }
                                    },
                                    svg { class: "todo-meta-icon", view_box: "0 0 12 12",
                                        rect { x: "1", y: "2", width: "10", height: "9", rx: "1.5", stroke: "currentColor", stroke_width: "1.4", fill: "none" }
                                        line { x1: "4", y1: "1", x2: "4", y2: "3", stroke: "currentColor", stroke_width: "1.4", stroke_linecap: "round" }
                                        line { x1: "8", y1: "1", x2: "8", y2: "3", stroke: "currentColor", stroke_width: "1.4", stroke_linecap: "round" }
                                    }
                                    if has_explicit_date {
                                        "{due_display}"
                                        if is_all_day {
                                            span { class: "todo-due-time", " · all day" }
                                        } else if let Some(t) = task.due_time.as_deref() {
                                            span { class: "todo-due-time", " {t}" }
                                            if let Some(ref end) = end_time {
                                                span { class: "todo-due-time", " – {end}" }
                                            }
                                        }
                                    } else {
                                        "No date"
                                    }
                                }
                            }
                        }
                    }

                    if has_subtasks {
                        div { class: "todo-progress",
                            Progress {
                                value: subtask_done as f64,
                                max: subtask_total as f64,
                                style: "width: 100%; height: 3px;",
                                ProgressIndicator {}
                            }
                            span { class: "todo-progress-label", "{subtask_done}/{subtask_total}" }
                        }
                    }
                }

                button {
                    class: "todo-more",
                    r#type: "button",
                    onclick: move |e| {
                        e.stop_propagation();
                        let p = e.client_coordinates();
                        menu_x.set(p.x);
                        menu_y.set(p.y);
                        show_menu.set(true);
                    },
                    "⋯"
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
                    style: "--cat: {stripe_color}",
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
                style: "left: min({menu_x}px, calc(100vw - 184px)); top: {menu_y}px;",
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
