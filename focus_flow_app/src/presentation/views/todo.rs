use std::time::Duration;

use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};

use crate::{
    components::select::{Select, SelectList, SelectOption, SelectTrigger, SelectValue},
    presentation::components::task::{create_task_sheet::CreateTaskSheet, task_row::TaskRow},
    use_cases::tasks::{
        create_subtask_uc::create_subtask_uc,
        create_task_uc::{create_task_uc, CreateTaskCommand},
        delete_task_uc::delete_task_uc,
        task_list_uc::{task_list_uc, TaskDue, TodoCategory, TodoTask},
        update_subtask_completition_uc::update_subtask_completition_uc,
        update_task_completition_uc::update_task_completition_uc,
    },
    Route,
};

#[component]
pub fn Todo() -> Element {
    let mut tasks = use_signal(Vec::<TodoTask>::new);
    let mut categories = use_signal(Vec::<TodoCategory>::new);
    let mut is_loading = use_signal(|| true);
    let mut load_error: Signal<Option<String>> = use_signal(|| None);
    let mut period_filter = use_signal(|| "all".to_string());
    let mut cat_filter = use_signal(|| "all".to_string());
    let mut show_modal = use_signal(|| false);
    let toast_api = use_toast();

    let mut fetch_task_list = use_resource(move || async move {
        //TODO review the get all task api and consider to add some filters so the app
        // doesn't load all tasks at once
        match task_list_uc(None).await {
            Ok(res) => {
                tasks.set(res.tasks);
                categories.set(res.categories);
                is_loading.set(false);
            }
            Err(e) => {
                load_error.set(Some(e.to_string()));
                is_loading.set(false);
            }
        }
    });

    let filtered: Vec<TodoTask> = tasks
        .read()
        .iter()
        .filter(|t| {
            let period_ok = match period_filter.read().as_str() {
                "today" => !t.done && matches!(t.due, TaskDue::Today(_)),
                "upcoming" => {
                    !t.done && matches!(t.due, TaskDue::Upcoming(_) | TaskDue::Tomorrow(_))
                }
                "done" => t.done,
                _ => true,
            };
            let cat_ok = {
                let filter = cat_filter.read();
                filter.as_str() == "all" || t.cat.as_deref() == Some(filter.as_str())
            };
            period_ok && cat_ok
        })
        .cloned()
        .collect();

    let overdue: Vec<TodoTask> = filtered
        .iter()
        .filter(|t| !t.done && matches!(t.due, TaskDue::Overdue(_)))
        .cloned()
        .collect();
    let today_tasks: Vec<TodoTask> = filtered
        .iter()
        .filter(|t| !t.done && matches!(t.due, TaskDue::Today(_)))
        .cloned()
        .collect();
    let upcoming_tasks: Vec<TodoTask> = filtered
        .iter()
        .filter(|t| !t.done && matches!(t.due, TaskDue::Upcoming(_) | TaskDue::Tomorrow(_)))
        .cloned()
        .collect();
    let done_tasks: Vec<TodoTask> = filtered.iter().filter(|t| t.done).cloned().collect();

    let complete_task_toggle = move |(id, completed): (String, bool)| {
        spawn(async move {
            match update_task_completition_uc(&id, Some(completed)).await {
                Ok(_) => {
                    info!("Task completed");
                    fetch_task_list.restart();
                }
                Err(e) => {
                    let task = tasks.iter().find(|t| t.id == id);
                    if let Some(task) = task {
                        if task.subtasks.iter().filter(|s| !s.is_completed).count() > 0 {
                            toast_api.info(
                                "Uncompleted subtasks".to_string(),
                                ToastOptions::new()
                                    .description(
                                        "Complete all subtasks before completing this task",
                                    )
                                    .duration(Duration::from_secs(15))
                                    .permanent(false),
                            );
                        }
                    }
                    error!("Error completing a task: {}", e.to_string());
                }
            }
        });
    };

    let complete_subtask_handler =
        move |(task_id, subtask_id, completed): (String, String, bool)| {
            spawn(async move {
                match update_subtask_completition_uc(task_id, subtask_id, Some(completed)).await {
                    Ok(_) => {
                        info!("Subtask completed");
                        fetch_task_list.restart();
                    }
                    Err(e) => {
                        error!("Error completing subtask: {}", e.to_string());
                    }
                }
            });
        };

    let delete_task_handler = move |id: String| {
        spawn(async move {
            match delete_task_uc(id).await {
                Ok(_) => {
                    info!("Task deleted");
                    fetch_task_list.restart();
                }
                Err(e) => {
                    error!("Error deleting a task: {}", e.to_string());
                }
            }
        });
    };

    let mut selected_task = use_context::<Signal<Option<(String, String)>>>();
    let navigator = use_navigator();
    let start_timer_handler = move |(task_id, task_title): (String, String)| {
        selected_task.set(Some((task_id, task_title)));
        navigator.push(Route::Pomodoro {});
    };

    let add_subtask_handler = move |(task_id, title): (String, String)| {
        spawn(async move {
            match create_subtask_uc(task_id, title, None).await {
                Ok(_) => {
                    info!("Subtask created");
                    fetch_task_list.restart();
                }
                Err(e) => {
                    error!("Error creating subtask: {}", e);
                }
            }
        });
    };

    let show_sections = *period_filter.read() == "all";

    rsx! {
        div { class: "filter-bar",
            div { class: "filter-select-wrap",
                Select::<String> {
                    default_value: Some("all".to_string()),
                    on_value_change: move |v: Option<String>| {
                        if let Some(v) = v { period_filter.set(v); }
                    },
                    SelectTrigger {
                        SelectValue { placeholder: "All periods" }
                    }
                    SelectList {
                        SelectOption::<String> { index: 0_usize, value: "all".to_string(), text_value: "All periods", "All periods" }
                        SelectOption::<String> { index: 1_usize, value: "today".to_string(), text_value: "Today", "Today" }
                        SelectOption::<String> { index: 2_usize, value: "upcoming".to_string(), text_value: "Upcoming", "Upcoming" }
                        SelectOption::<String> { index: 3_usize, value: "done".to_string(), text_value: "Done", "Done" }
                    }
                }
            }
            div { class: "filter-select-wrap",
                Select::<String> {
                    default_value: Some("all".to_string()),
                    on_value_change: move |v: Option<String>| {
                        if let Some(v) = v { cat_filter.set(v); }
                    },
                    SelectTrigger {
                        SelectValue { placeholder: "All categories" }
                    }
                    SelectList {
                        SelectOption::<String> { index: 0_usize, value: "all".to_string(), text_value: "All categories", "All categories" }
                        for (i, cat) in categories.read().iter().enumerate() {
                            SelectOption::<String> { index: i + 1, value: cat.name.clone(), text_value: cat.name.clone(), "@{cat.name}" }
                        }
                    }
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
                    TaskSection { label: "Overdue", modifier: "danger", tasks: overdue, on_toggle: complete_task_toggle, on_subtask_toggle: complete_subtask_handler, on_delete: delete_task_handler, on_start_timer: start_timer_handler, on_add_subtask: add_subtask_handler }
                }
                if !today_tasks.is_empty() {
                    TaskSection { label: "Today", modifier: "today", tasks: today_tasks, on_toggle: complete_task_toggle, on_subtask_toggle: complete_subtask_handler, on_delete: delete_task_handler, on_start_timer: start_timer_handler, on_add_subtask: add_subtask_handler }
                }
                if !upcoming_tasks.is_empty() {
                    TaskSection { label: "Upcoming", modifier: "", tasks: upcoming_tasks, on_toggle: complete_task_toggle, on_subtask_toggle: complete_subtask_handler, on_delete: delete_task_handler, on_start_timer: start_timer_handler, on_add_subtask: add_subtask_handler }
                }
                if !done_tasks.is_empty() {
                    TaskSection { label: "Done", modifier: "", tasks: done_tasks, on_toggle: complete_task_toggle, on_subtask_toggle: complete_subtask_handler, on_delete: delete_task_handler, on_start_timer: start_timer_handler, on_add_subtask: add_subtask_handler }
                }
            } else {
                for task in filtered.iter() {
                    TaskRow { task: task.clone(), on_toggle: complete_task_toggle, on_subtask_toggle: complete_subtask_handler, on_delete: delete_task_handler, on_start_timer: start_timer_handler, on_add_subtask: add_subtask_handler }
                }
            }
        }

        button {
            class: "fab",
            onclick: move |_| show_modal.set(true),
            svg { class: "fab-icon", view_box: "0 0 16 16",
                line { x1: "8", y1: "3", x2: "8", y2: "13", stroke: "currentColor", stroke_width: "1.8", stroke_linecap: "round" }
                line { x1: "3", y1: "8", x2: "13", y2: "8", stroke: "currentColor", stroke_width: "1.8", stroke_linecap: "round" }
            }
            span { "New task" }
        }

        CreateTaskSheet {
            show: *show_modal.read(),
            categories: categories.read().clone(),
            on_submit: move |dto: CreateTaskCommand| {
                spawn(async move {
                    match create_task_uc(dto).await {
                        Ok(_) => {
                            show_modal.set(false);
                            fetch_task_list.restart();
                        }
                        Err(e) => {
                            error!("Error creating task: {}", e);
                        }
                    }
                });
            },
            on_close: move |_| show_modal.set(false),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct TaskSectionProps {
    label: &'static str,
    modifier: &'static str,
    tasks: Vec<TodoTask>,
    on_toggle: EventHandler<(String, bool)>,
    on_subtask_toggle: EventHandler<(String, String, bool)>,
    on_delete: EventHandler<String>,
    on_start_timer: EventHandler<(String, String)>,
    on_add_subtask: EventHandler<(String, String)>,
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
                TaskRow { task: task.clone(), on_toggle: props.on_toggle, on_subtask_toggle: props.on_subtask_toggle, on_delete: props.on_delete, on_start_timer: props.on_start_timer, on_add_subtask: props.on_add_subtask }
            }
        }
    }
}
