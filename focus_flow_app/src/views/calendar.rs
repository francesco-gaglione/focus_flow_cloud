use dioxus::prelude::*;

use crate::components::models::{Task, TaskDue};

fn sample_tasks() -> Vec<Task> {
    vec![
        Task { id: "t1".into(), title: "Write design system intro".into(), cat: "work".into(), cat_color: "var(--cat-magenta)".into(), due: TaskDue::Today, done: false },
        Task { id: "t4".into(), title: "Reply to Marco".into(), cat: "work".into(), cat_color: "var(--cat-magenta)".into(), due: TaskDue::Today, done: false },
        Task { id: "t2".into(), title: "Pharmacy run".into(), cat: "errand".into(), cat_color: "var(--cat-amber)".into(), due: TaskDue::Tomorrow, done: false },
        Task { id: "t5".into(), title: "Sketch chapter 3 cover".into(), cat: "work".into(), cat_color: "var(--cat-magenta)".into(), due: TaskDue::Upcoming("Thu".into()), done: false },
    ]
}

#[component]
pub fn Calendar() -> Element {
    let mut cal_mode = use_signal(|| "month");

    rsx! {
        div { class: "scroll",
            // Mode toggle
            div { class: "cal-toggle",
                button {
                    class: if *cal_mode.read() == "month" { "active" } else { "" },
                    onclick: move |_| cal_mode.set("month"),
                    "Month"
                }
                button {
                    class: if *cal_mode.read() == "week" { "active" } else { "" },
                    onclick: move |_| cal_mode.set("week"),
                    "Week"
                }
            }

            // Nav
            div { class: "cal-nav",
                div { class: "cal-month",
                    "April ",
                    em { "2026" }
                }
                div { class: "cal-nav-btns",
                    button { class: "today-btn", "Today" }
                    button {
                        svg { view_box: "0 0 16 16",
                            path { d: "M10 4L6 8l4 4", stroke: "currentColor", fill: "none" }
                        }
                    }
                    button {
                        svg { view_box: "0 0 16 16",
                            path { d: "M6 4l4 4-4 4", stroke: "currentColor", fill: "none" }
                        }
                    }
                }
            }

            if *cal_mode.read() == "month" {
                MonthView {}
            } else {
                WeekView {}
            }
        }
    }
}

#[component]
fn MonthView() -> Element {
    let selected_day = use_signal(|| 27u32);
    let tasks = sample_tasks();

    let dows = ["M", "T", "W", "T", "F", "S", "S"];
    // April 2026 starts on Wednesday (index 2 in Mon-based)
    let start_dow = 2usize;
    let days_in_month = 30u32;

    rsx! {
        div { class: "cal-grid",
            // Day of week headers
            for dow in dows.iter() {
                div { class: "cal-dow", "{dow}" }
            }
            // Empty cells before start
            for _ in 0..start_dow {
                div { class: "cal-cell dim",
                    span { class: "cell-num" }
                }
            }
            // Day cells
            for day in 1..=days_in_month {
                {
                    let is_today = day == 27;
                    let is_selected = *selected_day.read() == day;
                    let has_tasks = matches!(day, 27 | 28 | 30);
                    let mut cell_class = "cal-cell".to_string();
                    if is_today { cell_class.push_str(" is-today"); }
                    if is_selected && !is_today { cell_class.push_str(" selected"); }
                    let mut sd = selected_day.clone();
                    rsx! {
                        div {
                            class: "{cell_class}",
                            onclick: move |_| sd.set(day),
                            span { class: "cell-num", "{day}" }
                            if has_tasks {
                                div { class: "cell-dots",
                                    span { class: "cell-dot", style: "--c: var(--cat-magenta)" }
                                    if day == 27 {
                                        span { class: "cell-dot", style: "--c: var(--cat-cyan)" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Day detail
        div { class: "day-detail",
            div { class: "day-detail-head",
                div { class: "date",
                    "Sunday, Apr ",
                    {
                        let day = *selected_day.read();
                        rsx! {
                            if day == 27 {
                                em { "27" }
                                " · today"
                            } else {
                                "{day}"
                            }
                        }
                    }
                }
                div { class: "meta",
                    {
                        let day = *selected_day.read();
                        if day == 27 { "2 tasks" } else { "0 tasks" }
                    }
                }
            }
            {
                let day = *selected_day.read();
                if day == 27 {
                    rsx! {
                        for task in tasks.iter().filter(|t| matches!(t.due, TaskDue::Today)) {
                            div { class: "todo-row",
                                style: "--cat: {task.cat_color}",
                                div { class: "todo-check" }
                                div { class: "todo-body",
                                    div { class: "todo-title", "{task.title}" }
                                    div { class: "todo-sub",
                                        span { class: "todo-cat", "@{task.cat}" }
                                        span { "·" }
                                        span { class: "todo-due today", "TODAY" }
                                    }
                                }
                                button { class: "todo-more", "⋯" }
                            }
                        }
                    }
                } else {
                    rsx! {
                        div { class: "cal-empty", "// quiet day · breathe" }
                    }
                }
            }
        }
    }
}

#[component]
fn WeekView() -> Element {
    let selected_day = use_signal(|| 27u32);
    let tasks = sample_tasks();

    let days = [
        ("M", 21u32, false), ("T", 22, false), ("W", 23, false),
        ("T", 24, false), ("F", 25, false), ("S", 26, false), ("S", 27, true),
    ];

    rsx! {
        div { class: "week-strip",
            for (dow, num, is_today) in days.iter() {
                {
                    let is_selected = *selected_day.read() == *num;
                    let mut cell_class = "week-day".to_string();
                    if *is_today { cell_class.push_str(" is-today"); }
                    if is_selected { cell_class.push_str(" selected"); }
                    let n = *num;
                    let mut sd = selected_day.clone();
                    rsx! {
                        div {
                            class: "{cell_class}",
                            onclick: move |_| sd.set(n),
                            span { class: "dow", "{dow}" }
                            span { class: "wnum", "{num}" }
                            span { class: "count-pill",
                                if *is_today { "2" } else { "·" }
                            }
                        }
                    }
                }
            }
        }

        // Agenda
        for (_, num, is_today) in days.iter() {
            {
                let n = *num;
                let it = *is_today;
                let day_tasks: Vec<&Task> = if it {
                    tasks.iter().filter(|t| matches!(t.due, TaskDue::Today)).collect()
                } else {
                    vec![]
                };
                rsx! {
                    div { class: "agenda-day",
                        div { class: "agenda-day-head",
                            span { class: "agenda-day-num",
                                if it { em { "{n}" } } else { "{n}" }
                            }
                            span {
                                class: if it { "agenda-day-name today" } else { "agenda-day-name" },
                                {
                                    let names = ["MONDAY", "TUESDAY", "WEDNESDAY", "THURSDAY", "FRIDAY", "SATURDAY", "SUNDAY"];
                                    let idx = (n as usize + 6) % 7; // rough
                                    let suffix = if it { " · TODAY" } else { "" };
                                    format!("{}{}", names[idx % 7], suffix)
                                }
                            }
                        }
                        if day_tasks.is_empty() {
                            div { class: "agenda-empty", "// quiet day" }
                        } else {
                            for task in day_tasks.iter() {
                                div { class: "todo-row", style: "--cat: {task.cat_color}",
                                    div { class: "todo-check" }
                                    div { class: "todo-body",
                                        div { class: "todo-title", "{task.title}" }
                                        div { class: "todo-sub",
                                            span { class: "todo-cat", "@{task.cat}" }
                                        }
                                    }
                                    button { class: "todo-more", "⋯" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
