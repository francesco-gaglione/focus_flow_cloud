use dioxus::prelude::*;
use shared::task::TaskPriority;

use crate::use_cases::tasks::task_list_uc::{TaskDue, TodoTask};

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
    /// Called with (task_id, new_priority) when priority is changed locally.
    /// Wire up when API is ready.
    #[props(optional)]
    pub on_priority_change: Option<EventHandler<(String, Option<TaskPriority>)>>,
    /// Called with (task_id, iso_date "YYYY-MM-DD" or empty) when date is changed.
    /// Wire up when API is ready.
    #[props(optional)]
    pub on_due_date_change: Option<EventHandler<(String, String)>>,
}

#[component]
pub fn TaskRow(props: TaskRowProps) -> Element {
    let task = &props.task;

    // ── local editable state ─────────────────────────────────────────────
    let initial_date = match &task.due {
        TaskDue::Overdue(d) | TaskDue::Today(d) | TaskDue::Tomorrow(d) | TaskDue::Upcoming(d) => {
            d.format("%Y-%m-%d").to_string()
        }
    };
    let mut local_date_str = use_signal(move || initial_date.clone());
    let has_explicit_date = task.due_date_set;

    // ── priority sheet context (provided by parent view) ─────────────────
    let prio_sheet_ctx =
        dioxus::core::try_consume_context::<Signal<Option<(String, Option<TaskPriority>)>>>();

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

    // Left stripe: priority color overrides category color
    let cat_color = task.cat_color.as_deref().unwrap_or("#888").to_string();
    let stripe_color = match task.priority {
        Some(TaskPriority::Low) => "#6b7280".to_string(),
        Some(TaskPriority::Medium) => "#d97706".to_string(),
        Some(TaskPriority::High) => "#ef4444".to_string(),
        Some(TaskPriority::Urgent) => "#7c3aed".to_string(),
        None => cat_color.clone(),
    };

    // Row class includes urgency tint modifiers
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

    // Priority badge values — always rendered (shows "—" when none)
    let (p_lvl, p_lbl) = match task.priority {
        Some(TaskPriority::Low) => ("low", "LOW"),
        Some(TaskPriority::Medium) => ("medium", "MED"),
        Some(TaskPriority::High) => ("high", "HIGH"),
        Some(TaskPriority::Urgent) => ("urgent", "URGENT"),
        None => ("none", "—"),
    };

    // Subtask progress
    let subtask_total = task.subtasks.len();
    let subtask_done = task.subtasks.iter().filter(|s| s.is_completed).count();
    let has_subtasks = subtask_total > 0;
    let subtask_pct = if subtask_total > 0 {
        subtask_done * 100 / subtask_total
    } else {
        0
    };

    let done = task.done;
    let id = task.id.clone();
    let priority_task_id = task.id.clone();
    let date_task_id = task.id.clone();
    let delete_id = task.id.clone();
    let timer_id = task.id.clone();
    let timer_title = task.title.clone();
    let on_delete = props.on_delete.clone();
    let on_start_timer = props.on_start_timer.clone();
    let on_add_subtask = props.on_add_subtask.clone();
    let on_due_date_change = props.on_due_date_change.clone();
    let current_priority = task.priority;
    let has_timer = on_start_timer.is_some();
    let has_add_subtask = on_add_subtask.is_some();
    let add_task_id = task.id.clone();
    let mut new_subtask_title = use_signal(String::new);

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
                            span { class: "todo-cat", "@{cat}" }
                        }

                        if has_explicit_date {
                            label { class: "{due_class} todo-date-wrap",
                                svg { class: "todo-meta-icon", view_box: "0 0 12 12",
                                    rect { x: "1", y: "2", width: "10", height: "9", rx: "1.5", stroke: "currentColor", stroke_width: "1.4", fill: "none" }
                                    line { x1: "4", y1: "1", x2: "4", y2: "3", stroke: "currentColor", stroke_width: "1.4", stroke_linecap: "round" }
                                    line { x1: "8", y1: "1", x2: "8", y2: "3", stroke: "currentColor", stroke_width: "1.4", stroke_linecap: "round" }
                                }
                                input {
                                    class: "todo-date-input",
                                    r#type: "date",
                                    value: "{local_date_str}",
                                    oninput: move |e| {
                                        let v = e.value();
                                        local_date_str.set(v.clone());
                                        if let Some(ref cb) = on_due_date_change {
                                            cb.call((date_task_id.clone(), v));
                                        }
                                    },
                                }
                            }
                        } else {
                            span { class: "todo-due todo-no-date",
                                svg { class: "todo-meta-icon", view_box: "0 0 12 12",
                                    rect { x: "1", y: "2", width: "10", height: "9", rx: "1.5", stroke: "currentColor", stroke_width: "1.4", fill: "none" }
                                    line { x1: "4", y1: "1", x2: "4", y2: "3", stroke: "currentColor", stroke_width: "1.4", stroke_linecap: "round" }
                                    line { x1: "8", y1: "1", x2: "8", y2: "3", stroke: "currentColor", stroke_width: "1.4", stroke_linecap: "round" }
                                }
                                "No date"
                            }
                        }
                    }

                    // Subtask progress bar
                    if has_subtasks {
                        div { class: "todo-progress",
                            div { class: "todo-progress-bar-track",
                                div { class: "todo-progress-bar", style: "width: {subtask_pct}%" }
                            }
                            span { class: "todo-progress-label", "{subtask_done}/{subtask_total}" }
                        }
                    }
                }

                // ── Actions ───────────────────────────────────────────────
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

            // ── Subtask panel ─────────────────────────────────────────────
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

        // ── Context menu ──────────────────────────────────────────────────
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
