use chrono::{Datelike, Duration, Local, Months, NaiveDate, Timelike, Weekday};
use dioxus::prelude::*;
use shared::task::TaskPriority;

use crate::use_cases::tasks::task_list_uc::{task_list_uc, TaskSchedule, TodoTask};

// ── constants ─────────────────────────────────────────────────────────────────

const HOUR_PX: f64 = 64.0;
const TIMELINE_START: u32 = 6;
const TIMELINE_END: u32 = 22;

// ── helpers ───────────────────────────────────────────────────────────────────

fn task_color(task: &TodoTask) -> String {
    match task.priority {
        Some(TaskPriority::Urgent) => "#7c3aed".to_string(),
        Some(TaskPriority::High) => "#ef4444".to_string(),
        Some(TaskPriority::Medium) => "#d97706".to_string(),
        Some(TaskPriority::Low) => "#6b7280".to_string(),
        None => task.cat_color.clone().unwrap_or_else(|| "#888888".to_string()),
    }
}

fn week_monday(date: NaiveDate) -> NaiveDate {
    let dow = date.weekday().num_days_from_monday();
    date - Duration::days(dow as i64)
}

fn days_in_month(year: i32, month: u32) -> u32 {
    let next = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    };
    next.map(|d| (d - Duration::days(1)).day()).unwrap_or(30)
}

fn month_name(m: u32) -> &'static str {
    match m {
        1 => "January", 2 => "February", 3 => "March", 4 => "April",
        5 => "May", 6 => "June", 7 => "July", 8 => "August",
        9 => "September", 10 => "October", 11 => "November", _ => "December",
    }
}

fn weekday_name(w: Weekday) -> &'static str {
    match w {
        Weekday::Mon => "Monday", Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday", Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday", Weekday::Sat => "Saturday", Weekday::Sun => "Sunday",
    }
}

fn weekday_short(w: Weekday) -> &'static str {
    match w {
        Weekday::Mon => "Mon", Weekday::Tue => "Tue", Weekday::Wed => "Wed",
        Weekday::Thu => "Thu", Weekday::Fri => "Fri", Weekday::Sat => "Sat",
        Weekday::Sun => "Sun",
    }
}

// ── data structs ──────────────────────────────────────────────────────────────

struct DayCell {
    date: NaiveDate,
    chips: Vec<(String, String)>,
}

#[derive(Clone)]
struct TimedItem {
    title: String,
    color: String,
    top_px: f64,
    height_px: f64,
}

// ── Calendar ──────────────────────────────────────────────────────────────────

#[component]
pub fn Calendar() -> Element {
    let mut cal_mode: Signal<&'static str> = use_signal(|| "month");
    let mut current_date: Signal<NaiveDate> = use_signal(|| Local::now().date_naive());

    let tasks_res = use_resource(move || async move { task_list_uc(None).await });

    let tasks: Vec<TodoTask> = match &*tasks_res.read() {
        Some(Ok(list)) => list.tasks.clone(),
        _ => vec![],
    };

    let cur = *current_date.read();
    let mode = *cal_mode.read();
    let month_val = format!("{}-{:02}", cur.year(), cur.month());

    let month_label = if mode == "week" {
        let mon = week_monday(cur);
        let sun = mon + Duration::days(6);
        if mon.month() == sun.month() {
            format!("{} {} – {}, {}", month_name(mon.month()), mon.day(), sun.day(), mon.year())
        } else {
            format!("{} {} – {} {}, {}", month_name(mon.month()), mon.day(), month_name(sun.month()), sun.day(), sun.year())
        }
    } else {
        format!("{} {}", month_name(cur.month()), cur.year())
    };

    rsx! {
        div { class: "scroll",

            div { class: "cal-nav",
                div { class: "cal-toggle",
                    button {
                        class: if mode == "month" { "active" } else { "" },
                        onclick: move |_| cal_mode.set("month"),
                        "Month"
                    }
                    button {
                        class: if mode == "week" { "active" } else { "" },
                        onclick: move |_| cal_mode.set("week"),
                        "Week"
                    }
                }

                div { class: "cal-nav-btns",
                    button {
                        onclick: move |_| {
                            let d = *current_date.read();
                            if *cal_mode.read() == "month" {
                                if let Some(prev) = d.checked_sub_months(Months::new(1)) {
                                    current_date.set(prev);
                                }
                            } else {
                                current_date.set(d - Duration::weeks(1));
                            }
                        },
                        svg { view_box: "0 0 16 16",
                            path { d: "M10 4L6 8l4 4", stroke: "currentColor", fill: "none" }
                        }
                    }

                    div { class: "cal-month-pick",
                        span { class: "cal-month", "{month_label}" }
                        input {
                            r#type: "month",
                            class: "cal-month-input",
                            value: "{month_val}",
                            oninput: move |e| {
                                let v = e.value();
                                let parts: Vec<&str> = v.split('-').collect();
                                if parts.len() == 2 {
                                    if let (Ok(y), Ok(m)) = (parts[0].parse::<i32>(), parts[1].parse::<u32>()) {
                                        if let Some(nd) = NaiveDate::from_ymd_opt(y, m, 1) {
                                            current_date.set(nd);
                                        }
                                    }
                                }
                            },
                        }
                    }

                    button {
                        onclick: move |_| {
                            let d = *current_date.read();
                            if *cal_mode.read() == "month" {
                                if let Some(next) = d.checked_add_months(Months::new(1)) {
                                    current_date.set(next);
                                }
                            } else {
                                current_date.set(d + Duration::weeks(1));
                            }
                        },
                        svg { view_box: "0 0 16 16",
                            path { d: "M6 4l4 4-4 4", stroke: "currentColor", fill: "none" }
                        }
                    }

                    button {
                        class: "today-btn",
                        onclick: move |_| current_date.set(Local::now().date_naive()),
                        "Today"
                    }
                }
            }

            if mode == "month" {
                MonthView { tasks: tasks.clone(), current_date: cur }
            } else {
                WeekView { tasks, current_date: cur }
            }
        }
    }
}

// ── MonthView ─────────────────────────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
struct MonthViewProps {
    tasks: Vec<TodoTask>,
    current_date: NaiveDate,
}

#[component]
fn MonthView(props: MonthViewProps) -> Element {
    let today = Local::now().date_naive();
    let mut selected: Signal<NaiveDate> = use_signal(|| today);

    let year = props.current_date.year();
    let month = props.current_date.month();
    let dim = days_in_month(year, month);
    let first = NaiveDate::from_ymd_opt(year, month, 1).unwrap_or(props.current_date);
    let start_offset = first.weekday().num_days_from_monday() as usize;

    let cells: Vec<DayCell> = (1..=dim)
        .map(|d| {
            let date = NaiveDate::from_ymd_opt(year, month, d).unwrap_or(first);
            let chips = props.tasks.iter()
                .filter(|t| t.schedule.naive_date() == Some(date))
                .map(|t| (t.title.clone(), task_color(t)))
                .collect();
            DayCell { date, chips }
        })
        .collect();

    let sel = *selected.read();
    let sel_tasks: Vec<(String, String, Option<String>)> = props.tasks.iter()
        .filter(|t| t.schedule.naive_date() == Some(sel))
        .map(|t| (t.title.clone(), task_color(t), t.cat.clone()))
        .collect();
    let sel_count = sel_tasks.len();
    let sel_count_str = format!("{} task{}", sel_count, if sel_count == 1 { "" } else { "s" });
    let sel_head = format!("{}, {} {}{}",
        weekday_name(sel.weekday()), month_name(sel.month()), sel.day(),
        if sel == today { " · today" } else { "" }
    );

    let dows = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

    rsx! {
        div { class: "cal-grid",
            for dow in dows {
                div { class: "cal-dow", "{dow}" }
            }
            for _ in 0..start_offset {
                div { class: "cal-cell dim" }
            }
            for cell in cells {
                {
                    let date = cell.date;
                    let chip_count = cell.chips.len();
                    let visible: Vec<(String, String)> = cell.chips.into_iter().take(2).collect();
                    let is_today = date == today;
                    let is_sel = date == sel;
                    let mut cls = "cal-cell".to_string();
                    if is_today { cls.push_str(" is-today"); }
                    else if is_sel { cls.push_str(" selected"); }
                    rsx! {
                        div {
                            class: "{cls}",
                            onclick: move |_| selected.set(date),
                            span { class: "cell-num", "{date.day()}" }
                            div { class: "cell-chips",
                                for (title, color) in visible {
                                    div { class: "cell-chip", style: "background:{color};", "{title}" }
                                }
                                if chip_count > 2 {
                                    div { class: "cell-chip-more", "+{chip_count - 2}" }
                                }
                            }
                        }
                    }
                }
            }
        }

        div { class: "day-detail",
            div { class: "day-detail-head",
                span { class: "date", "{sel_head}" }
                span { class: "meta", "{sel_count_str}" }
            }
            if sel_tasks.is_empty() {
                div { class: "cal-empty", "// quiet day · breathe" }
            } else {
                div { class: "day-task-list",
                    for (title, color, cat) in sel_tasks {
                        div { class: "day-task-row",
                            div { class: "day-task-stripe", style: "background:{color};" }
                            div { class: "day-task-body",
                                div { class: "day-task-title", "{title}" }
                                if let Some(c) = cat {
                                    span {
                                        class: "todo-cat",
                                        style: "color:{color};background:color-mix(in srgb,{color} 15%,transparent);border-color:color-mix(in srgb,{color} 30%,transparent);",
                                        "@{c}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── WeekView ──────────────────────────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
struct WeekViewProps {
    tasks: Vec<TodoTask>,
    current_date: NaiveDate,
}

#[component]
fn WeekView(props: WeekViewProps) -> Element {
    let today = Local::now().date_naive();
    let now = Local::now();
    let monday = week_monday(props.current_date);

    let now_top: Option<f64> = {
        let h = now.hour() as f64;
        let m = now.minute() as f64;
        if h >= TIMELINE_START as f64 && h < TIMELINE_END as f64 {
            Some((h - TIMELINE_START as f64 + m / 60.0) * HOUR_PX)
        } else {
            None
        }
    };

    let total_height = (TIMELINE_END - TIMELINE_START) as f64 * HOUR_PX;

    let header_data: Vec<(NaiveDate, bool)> = (0..7i64)
        .map(|i| { let d = monday + Duration::days(i); (d, d == today) })
        .collect();

    let mut allday_data: Vec<Vec<(String, String)>> = vec![];
    let mut timeline_data: Vec<(bool, Vec<TimedItem>)> = vec![];

    for i in 0..7i64 {
        let date = monday + Duration::days(i);
        let is_today = date == today;
        let mut allday: Vec<(String, String)> = vec![];
        let mut timed: Vec<TimedItem> = vec![];

        for task in props.tasks.iter().filter(|t| t.schedule.naive_date() == Some(date)) {
            let color = task_color(task);
            match &task.schedule {
                TaskSchedule::AllDay { .. } => {
                    allday.push((task.title.clone(), color));
                }
                TaskSchedule::At { starts_at } => {
                    let h = starts_at.hour() as f64;
                    let m = starts_at.minute() as f64;
                    if h >= TIMELINE_START as f64 && h < TIMELINE_END as f64 {
                        timed.push(TimedItem {
                            title: task.title.clone(), color,
                            top_px: (h - TIMELINE_START as f64 + m / 60.0) * HOUR_PX,
                            height_px: 28.0,
                        });
                    }
                }
                TaskSchedule::Span { starts_at, duration_secs } => {
                    let h = starts_at.hour() as f64;
                    let m = starts_at.minute() as f64;
                    if h >= TIMELINE_START as f64 && h < TIMELINE_END as f64 {
                        timed.push(TimedItem {
                            title: task.title.clone(), color,
                            top_px: (h - TIMELINE_START as f64 + m / 60.0) * HOUR_PX,
                            height_px: (*duration_secs as f64 / 3600.0 * HOUR_PX).max(28.0),
                        });
                    }
                }
                TaskSchedule::Unscheduled => {}
            }
        }

        allday_data.push(allday);
        timeline_data.push((is_today, timed));
    }

    let has_allday = allday_data.iter().any(|a| !a.is_empty());

    rsx! {
        div { class: "week-view",

            div { class: "week-header",
                div { class: "week-gutter" }
                for (date, is_today) in header_data {
                    div {
                        class: if is_today { "week-col-head is-today" } else { "week-col-head" },
                        span { class: "week-dow", "{weekday_short(date.weekday())}" }
                        span { class: "week-day-num", "{date.day()}" }
                    }
                }
            }

            if has_allday {
                div { class: "week-allday-strip",
                    div { class: "week-gutter",
                        span { class: "week-allday-label", "all day" }
                    }
                    for chips in allday_data {
                        div { class: "week-allday-col",
                            for (title, color) in chips {
                                div { class: "week-chip", style: "background:{color};", "{title}" }
                            }
                        }
                    }
                }
            }

            div { class: "week-scroll",
                div { class: "week-timeline-inner", style: "height:{total_height}px;",

                    div { class: "week-gutter week-gutter-abs",
                        for h in TIMELINE_START..TIMELINE_END {
                            span {
                                class: "week-hour-label",
                                style: "top:{(h - TIMELINE_START) as f64 * HOUR_PX}px;",
                                "{h:02}:00"
                            }
                        }
                    }

                    div { class: "week-cols",
                        for (is_today, timed) in timeline_data {
                            div {
                                class: if is_today { "week-col is-today" } else { "week-col" },
                                for h in TIMELINE_START..TIMELINE_END {
                                    div {
                                        class: "week-gridline",
                                        style: "top:{(h - TIMELINE_START) as f64 * HOUR_PX}px;",
                                    }
                                }
                                if is_today {
                                    if let Some(top) = now_top {
                                        div { class: "week-now", style: "top:{top}px;" }
                                    }
                                }
                                for item in timed {
                                    div {
                                        class: "week-task",
                                        style: "top:{item.top_px}px;height:{item.height_px}px;background:{item.color};",
                                        span { class: "week-task-title", "{item.title}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
