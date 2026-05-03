use dioxus::prelude::*;

use crate::{
    model::task::{Subtask, Task, TaskDue},
    presentation::components::common_components::TaskRow,
    services::{
        api_client::ApiClient,
        storage,
        task_services::{get_all_tasks, TaskError},
    },
    state::auth_state::AuthState,
};

#[component]
pub fn Todo() -> Element {
    let mut tasks = use_signal(Vec::<Task>::new);
    let mut is_loading = use_signal(|| true);
    let mut load_error: Signal<Option<String>> = use_signal(|| None);
    let mut period_filter = use_signal(|| "all".to_string());
    let mut cat_filter = use_signal(|| "all".to_string());
    let mut show_modal = use_signal(|| false);
    let mut new_title = use_signal(String::new);
    let mut new_cat = use_signal(|| "personal".to_string());
    let mut new_subtask_input = use_signal(String::new);
    let mut new_subtasks: Signal<Vec<String>> = use_signal(Vec::new);

    let mut auth_state = use_context::<Signal<AuthState>>();
    let mut api_client = use_context::<Signal<ApiClient>>();

    let _ = use_resource(move || async move {
        match get_all_tasks().await {
            Ok(fetched) => {
                tasks.set(fetched);
                is_loading.set(false);
            }
            Err(TaskError::Unauthorized) => {
                auth_state.write().set_auth_token(None);
                auth_state.write().set_refresh_token(None);
                api_client.write().set_auth_token(None);
                storage::remove_item("auth_token");
                storage::remove_item("refresh_token");
                is_loading.set(false);
            }
            Err(e) => {
                load_error.set(Some(e.to_string()));
                is_loading.set(false);
            }
        }
    });

    let filtered: Vec<Task> = tasks
        .read()
        .iter()
        .filter(|t| {
            let period_ok = match period_filter.read().as_str() {
                "today" => !t.done && matches!(t.due, TaskDue::Today),
                "upcoming" => !t.done && matches!(t.due, TaskDue::Upcoming(_) | TaskDue::Tomorrow),
                "done" => t.done,
                _ => true,
            };
            let cat_ok = match cat_filter.read().as_str() {
                "work" => t.cat == "work",
                "personal" => t.cat == "personal",
                "errand" => t.cat == "errand",
                _ => true,
            };
            period_ok && cat_ok
        })
        .cloned()
        .collect();

    let overdue: Vec<Task> = filtered
        .iter()
        .filter(|t| !t.done && matches!(t.due, TaskDue::Overdue(_)))
        .cloned()
        .collect();
    let today_tasks: Vec<Task> = filtered
        .iter()
        .filter(|t| !t.done && matches!(t.due, TaskDue::Today))
        .cloned()
        .collect();
    let upcoming_tasks: Vec<Task> = filtered
        .iter()
        .filter(|t| !t.done && matches!(t.due, TaskDue::Upcoming(_) | TaskDue::Tomorrow))
        .cloned()
        .collect();
    let done_tasks: Vec<Task> = filtered.iter().filter(|t| t.done).cloned().collect();

    let toggle_task = move |id: String| {
        let mut t = tasks.write();
        if let Some(task) = t.iter_mut().find(|t| t.id == id) {
            task.done = !task.done;
        }
    };

    let show_sections = *period_filter.read() == "all";

    rsx! {
        // Filter bar — two dropdowns
        div { class: "filter-bar",
            div { class: "filter-select-wrap",
                select {
                    class: "filter-select",
                    onchange: move |e| period_filter.set(e.value()),
                    option { value: "all", "All periods" }
                    option { value: "today", "Today" }
                    option { value: "upcoming", "Upcoming" }
                    option { value: "done", "Done" }
                }
            }
            div { class: "filter-select-wrap",
                select {
                    class: "filter-select",
                    onchange: move |e| cat_filter.set(e.value()),
                    option { value: "all", "All categories" }
                    option { value: "work", "@work" }
                    option { value: "personal", "@personal" }
                    option { value: "errand", "@errand" }
                }
            }
        }

        div { class: "scroll",
            if *is_loading.read() {
                div { class: "empty-state",
                    p { "Loading…" }
                }
            } else if let Some(err) = load_error.read().as_ref() {
                div { class: "empty-state",
                    p { "Failed to load tasks: {err}" }
                }
            } else if filtered.is_empty() {
                div { class: "empty-state",
                    div { class: "ico",
                        svg { view_box: "0 0 16 16",
                            path { d: "M3 8l3 3 7-7", stroke: "currentColor", stroke_width: "1.5", fill: "none" }
                        }
                    }
                    h3 { "Nothing ", em { "here" }, "." }
                    p { "Either you're done, or this filter is too strict. Both are fine." }
                }
            } else if show_sections {
                if !overdue.is_empty() {
                    TaskSection { label: "Overdue", modifier: "danger", tasks: overdue, on_toggle: toggle_task }
                }
                if !today_tasks.is_empty() {
                    TaskSection { label: "Today", modifier: "today", tasks: today_tasks, on_toggle: toggle_task }
                }
                if !upcoming_tasks.is_empty() {
                    TaskSection { label: "Upcoming", modifier: "", tasks: upcoming_tasks, on_toggle: toggle_task }
                }
                if !done_tasks.is_empty() {
                    TaskSection { label: "Done", modifier: "", tasks: done_tasks, on_toggle: toggle_task }
                }
            } else {
                for task in filtered.iter() {
                    TaskRow { task: task.clone(), on_toggle: toggle_task }
                }
            }
        }

        // FAB
        button {
            class: "fab",
            onclick: move |_| show_modal.set(true),
            svg { class: "fab-icon", view_box: "0 0 16 16",
                line { x1: "8", y1: "3", x2: "8", y2: "13", stroke: "currentColor", stroke_width: "1.8", stroke_linecap: "round" }
                line { x1: "3", y1: "8", x2: "13", y2: "8", stroke: "currentColor", stroke_width: "1.8", stroke_linecap: "round" }
            }
            span { "New task" }
        }

        // Bottom sheet modal
        if *show_modal.read() {
            div {
                class: "sheet-overlay",
                onclick: move |_| show_modal.set(false),
            }
            div { class: "bottom-sheet",
                div { class: "sheet-handle" }
                div { class: "sheet-head",
                    span { class: "sheet-title", "New task" }
                    button {
                        class: "sheet-close",
                        onclick: move |_| show_modal.set(false),
                        svg { view_box: "0 0 16 16",
                            line { x1: "4", y1: "4", x2: "12", y2: "12", stroke: "currentColor", stroke_width: "1.6" }
                            line { x1: "12", y1: "4", x2: "4", y2: "12", stroke: "currentColor", stroke_width: "1.6" }
                        }
                    }
                }
                form {
                    class: "sheet-form",
                    onsubmit: move |e| {
                        e.prevent_default();
                        let val = new_title.read().trim().to_string();
                        if !val.is_empty() {
                            let cat = new_cat.read().clone();
                            let cat_color = match cat.as_str() {
                                "work" => "var(--cat-magenta)".into(),
                                "errand" => "var(--cat-amber)".into(),
                                _ => "var(--cat-cyan)".into(),
                            };
                            let id = format!("t{}", tasks.read().len() + 1);
                            let subtasks = new_subtasks
                                .read()
                                .clone()
                                .into_iter()
                                .enumerate()
                                .map(|(i, title)| Subtask {
                                    id: format!("s{i}"),
                                    title,
                                    is_completed: false,
                                    sort_order: i as i16,
                                })
                                .collect();
                            tasks.write().insert(0, Task {
                                id,
                                title: val,
                                description: None,
                                cat,
                                cat_color,
                                priority: None,
                                due: TaskDue::Today,
                                completed_at: None,
                                done: false,
                                subtasks,
                            });
                            new_title.set(String::new());
                            new_subtask_input.set(String::new());
                            new_subtasks.write().clear();
                            show_modal.set(false);
                        }
                    },
                    div { class: "sheet-field",
                        label { class: "sheet-label", "Title" }
                        input {
                            class: "sheet-input",
                            placeholder: "What needs to be done?",
                            value: "{new_title}",
                            oninput: move |e| new_title.set(e.value()),
                        }
                    }
                    div { class: "sheet-field",
                        label { class: "sheet-label", "Category" }
                        div { class: "sheet-select-wrap",
                            select {
                                class: "sheet-select",
                                onchange: move |e| new_cat.set(e.value()),
                                option { value: "personal", "@personal" }
                                option { value: "work", "@work" }
                                option { value: "errand", "@errand" }
                            }
                        }
                    }
                    div { class: "sheet-field",
                        label { class: "sheet-label", "Subtasks" }
                        for (i, sub) in new_subtasks.read().clone().into_iter().enumerate() {
                            div { class: "sheet-subtask-row",
                                span { class: "sheet-subtask-title", "{sub}" }
                                button {
                                    r#type: "button",
                                    class: "sheet-subtask-rm",
                                    onclick: move |_| { new_subtasks.write().remove(i); },
                                    "×"
                                }
                            }
                        }
                        div { class: "sheet-subtask-add",
                            input {
                                class: "sheet-input",
                                placeholder: "Add a subtask…",
                                value: "{new_subtask_input}",
                                oninput: move |e| new_subtask_input.set(e.value()),
                                onkeydown: move |e| {
                                    if e.key() == Key::Enter {
                                        e.prevent_default();
                                        let val = new_subtask_input.read().trim().to_string();
                                        if !val.is_empty() {
                                            new_subtasks.write().push(val);
                                            new_subtask_input.set(String::new());
                                        }
                                    }
                                },
                            }
                            button {
                                r#type: "button",
                                class: "sheet-subtask-add-btn",
                                onclick: move |_| {
                                    let val = new_subtask_input.read().trim().to_string();
                                    if !val.is_empty() {
                                        new_subtasks.write().push(val);
                                        new_subtask_input.set(String::new());
                                    }
                                },
                                svg { view_box: "0 0 16 16",
                                    line { x1: "8", y1: "3", x2: "8", y2: "13", stroke: "currentColor", stroke_width: "1.8", stroke_linecap: "round" }
                                    line { x1: "3", y1: "8", x2: "13", y2: "8", stroke: "currentColor", stroke_width: "1.8", stroke_linecap: "round" }
                                }
                            }
                        }
                    }
                    div { class: "sheet-actions",
                        button {
                            r#type: "button",
                            class: "sheet-btn-cancel",
                            onclick: move |_| show_modal.set(false),
                            "Cancel"
                        }
                        button {
                            r#type: "submit",
                            class: "sheet-btn-submit",
                            "Add task"
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct TaskSectionProps {
    label: &'static str,
    modifier: &'static str,
    tasks: Vec<Task>,
    on_toggle: EventHandler<String>,
}

#[component]
fn TaskSection(props: TaskSectionProps) -> Element {
    let count = props.tasks.len();
    let label_class = format!("lbl {}", props.modifier);
    let word = if count == 1 { "task" } else { "tasks" };
    rsx! {
        div {
            div { class: "section-head",
                span { class: "{label_class}", "{props.label}" }
                span { class: "count", "{count} {word}" }
            }
            for task in props.tasks.iter() {
                TaskRow { task: task.clone(), on_toggle: props.on_toggle }
            }
        }
    }
}
