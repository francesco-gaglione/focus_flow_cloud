use dioxus::prelude::*;

use crate::components::common_components::{FilterChip, TaskRow};
use crate::components::models::{Task, TaskDue};

fn mock_tasks() -> Vec<Task> {
    vec![
        Task { id: "t1".into(), title: "Write design system intro for FocusFlow".into(), cat: "work".into(), cat_color: "var(--cat-magenta)".into(), due: TaskDue::Today, done: false },
        Task { id: "t2".into(), title: "Pharmacy: pick up prescription".into(), cat: "errand".into(), cat_color: "var(--cat-amber)".into(), due: TaskDue::Tomorrow, done: false },
        Task { id: "t3".into(), title: "Morning walk · 20 min".into(), cat: "personal".into(), cat_color: "var(--cat-cyan)".into(), due: TaskDue::Today, done: true },
        Task { id: "t4".into(), title: "Reply to Marco about the proposal".into(), cat: "work".into(), cat_color: "var(--cat-magenta)".into(), due: TaskDue::Today, done: false },
        Task { id: "t5".into(), title: "Sketch ideas for chapter 3 cover".into(), cat: "work".into(), cat_color: "var(--cat-magenta)".into(), due: TaskDue::Upcoming("Thu".into()), done: false },
        Task { id: "t6".into(), title: "Call dentist to reschedule".into(), cat: "errand".into(), cat_color: "var(--cat-amber)".into(), due: TaskDue::Today, done: false },
        Task { id: "t7".into(), title: "Drink water · 2L today".into(), cat: "personal".into(), cat_color: "var(--cat-cyan)".into(), due: TaskDue::Today, done: true },
        Task { id: "t8".into(), title: "Review Q2 roadmap doc".into(), cat: "work".into(), cat_color: "var(--cat-magenta)".into(), due: TaskDue::Upcoming("Wed".into()), done: false },
        Task { id: "t9".into(), title: "Buy groceries for the week".into(), cat: "errand".into(), cat_color: "var(--cat-amber)".into(), due: TaskDue::Tomorrow, done: false },
        Task { id: "t10".into(), title: "Take meds @ 20:00".into(), cat: "personal".into(), cat_color: "var(--cat-cyan)".into(), due: TaskDue::Today, done: false },
        Task { id: "t11".into(), title: "Read 20 pages".into(), cat: "personal".into(), cat_color: "var(--cat-cyan)".into(), due: TaskDue::Today, done: true },
        Task { id: "t12".into(), title: "Plan weekend trip".into(), cat: "personal".into(), cat_color: "var(--cat-cyan)".into(), due: TaskDue::Upcoming("Sat".into()), done: false },
    ]
}

#[component]
pub fn Todo() -> Element {
    let mut tasks = use_signal(mock_tasks);
    let mut filter = use_signal(|| "all".to_string());
    let mut composer_value = use_signal(String::new);

    let all_count = tasks.read().len();
    let today_count = tasks.read().iter().filter(|t| !t.done && matches!(t.due, TaskDue::Today)).count();
    let upcoming_count = tasks.read().iter().filter(|t| !t.done && matches!(t.due, TaskDue::Upcoming(_) | TaskDue::Tomorrow)).count();
    let done_count = tasks.read().iter().filter(|t| t.done).count();

    let filtered: Vec<Task> = tasks.read().iter().filter(|t| {
        match filter.read().as_str() {
            "today" => !t.done && matches!(t.due, TaskDue::Today),
            "upcoming" => !t.done && matches!(t.due, TaskDue::Upcoming(_) | TaskDue::Tomorrow),
            "done" => t.done,
            "work" => t.cat == "work",
            "personal" => t.cat == "personal",
            "errand" => t.cat == "errand",
            _ => true,
        }
    }).cloned().collect();

    let overdue: Vec<Task> = filtered.iter().filter(|t| !t.done && matches!(t.due, TaskDue::Overdue(_))).cloned().collect();
    let today_tasks: Vec<Task> = filtered.iter().filter(|t| !t.done && matches!(t.due, TaskDue::Today)).cloned().collect();
    let upcoming_tasks: Vec<Task> = filtered.iter().filter(|t| !t.done && matches!(t.due, TaskDue::Upcoming(_) | TaskDue::Tomorrow)).cloned().collect();
    let done_tasks: Vec<Task> = filtered.iter().filter(|t| t.done).cloned().collect();

    let toggle_task = move |id: String| {
        let mut t = tasks.write();
        if let Some(task) = t.iter_mut().find(|t| t.id == id) {
            task.done = !task.done;
        }
    };

    rsx! {
        // Composer
        div { class: "composer-bar",
            form {
                class: "composer",
                onsubmit: move |e| {
                    e.prevent_default();
                    let val = composer_value.read().trim().to_string();
                    if !val.is_empty() {
                        let id = format!("t{}", tasks.read().len() + 1);
                        tasks.write().insert(0, Task {
                            id,
                            title: val,
                            cat: "personal".into(),
                            cat_color: "var(--cat-cyan)".into(),
                            due: TaskDue::Today,
                            done: false,
                        });
                        composer_value.set(String::new());
                    }
                },
                svg { class: "add-icon", view_box: "0 0 16 16",
                    line { x1: "8", y1: "3", x2: "8", y2: "13" }
                    line { x1: "3", y1: "8", x2: "13", y2: "8" }
                }
                input {
                    value: "{composer_value}",
                    placeholder: "Add a thought… @work !today",
                    oninput: move |e| composer_value.set(e.value()),
                }
                span { class: "hint", kbd { "↵" } }
            }
        }

        div { class: "scroll",
            // Filter strip
            div { class: "filter-strip",
                FilterChip { label: "All", count: Some(all_count), active: *filter.read() == "all", onclick: move |_: ()| filter.set("all".into()) }
                FilterChip { label: "Today", count: Some(today_count), active: *filter.read() == "today", onclick: move |_: ()| filter.set("today".into()) }
                FilterChip { label: "Upcoming", count: Some(upcoming_count), active: *filter.read() == "upcoming", onclick: move |_: ()| filter.set("upcoming".into()) }
                FilterChip { label: "Done", count: Some(done_count), active: *filter.read() == "done", onclick: move |_: ()| filter.set("done".into()) }
                FilterChip { label: "@work", count: None, active: *filter.read() == "work", onclick: move |_: ()| filter.set("work".into()) }
                FilterChip { label: "@personal", count: None, active: *filter.read() == "personal", onclick: move |_: ()| filter.set("personal".into()) }
                FilterChip { label: "@errand", count: None, active: *filter.read() == "errand", onclick: move |_: ()| filter.set("errand".into()) }
            }

            if filtered.is_empty() {
                div { class: "empty-state",
                    div { class: "ico",
                        svg { view_box: "0 0 16 16",
                            path { d: "M3 8l3 3 7-7", stroke: "currentColor", stroke_width: "1.5", fill: "none" }
                        }
                    }
                    h3 { "Nothing ", em { "here" }, "." }
                    p { "Either you're done, or this filter is too strict. Both are fine." }
                }
            } else if *filter.read() == "all" {
                // Grouped sections
                if !overdue.is_empty() {
                    TaskSection { label: "Overdue", modifier: "danger", tasks: overdue.clone(), on_toggle: toggle_task.clone() }
                }
                if !today_tasks.is_empty() {
                    TaskSection { label: "Today", modifier: "today", tasks: today_tasks.clone(), on_toggle: toggle_task.clone() }
                }
                if !upcoming_tasks.is_empty() {
                    TaskSection { label: "Upcoming", modifier: "", tasks: upcoming_tasks.clone(), on_toggle: toggle_task.clone() }
                }
                if !done_tasks.is_empty() {
                    TaskSection { label: "Done", modifier: "", tasks: done_tasks.clone(), on_toggle: toggle_task.clone() }
                }
            } else {
                for task in filtered.iter() {
                    TaskRow { task: task.clone(), on_toggle: toggle_task.clone() }
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
                TaskRow { task: task.clone(), on_toggle: props.on_toggle.clone() }
            }
        }
    }
}
