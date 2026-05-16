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
        None => task
            .cat_color
            .clone()
            .unwrap_or_else(|| "#888888".to_string()),
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
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        _ => "December",
    }
}

fn weekday_name(w: Weekday) -> &'static str {
    match w {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    }
}

fn weekday_short(w: Weekday) -> &'static str {
    match w {
        Weekday::Mon => "Mon",
        Weekday::Tue => "Tue",
        Weekday::Wed => "Wed",
        Weekday::Thu => "Thu",
        Weekday::Fri => "Fri",
        Weekday::Sat => "Sat",
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
        let m1 = &month_name(mon.month())[..3];
        let m2 = &month_name(sun.month())[..3];
        if mon.month() == sun.month() {
            format!("{} {}–{}", m1, mon.day(), sun.day())
        } else {
            format!("{} {}–{} {}", m1, mon.day(), m2, sun.day())
        }
    } else {
        format!("{} {}", month_name(cur.month()), cur.year())
    };

    rsx! {
        div { class: "scroll",

            div { class: "flex items-center justify-between mb-[14px]",
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

                div { class: "cal-nav-btns flex gap-[6px] items-center",
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

                    div { class: "relative cursor-pointer flex items-center overflow-hidden",
                        span {
                            class: "cal-month",
                            style: if mode == "week" { "font-size:15px;" } else { "" },
                            "{month_label}"
                        }
                        input {
                            r#type: "month",
                            class: "absolute inset-0 opacity-0 cursor-pointer",
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
            let chips = props
                .tasks
                .iter()
                .filter(|t| t.schedule.naive_date() == Some(date))
                .map(|t| (t.title.clone(), task_color(t)))
                .collect();
            DayCell { date, chips }
        })
        .collect();

    let sel = *selected.read();
    let sel_tasks: Vec<(String, String, Option<String>)> = props
        .tasks
        .iter()
        .filter(|t| t.schedule.naive_date() == Some(sel))
        .map(|t| (t.title.clone(), task_color(t), t.cat.clone()))
        .collect();
    let sel_count = sel_tasks.len();
    let sel_count_str = format!(
        "{} task{}",
        sel_count,
        if sel_count == 1 { "" } else { "s" }
    );
    let sel_head = format!(
        "{}, {} {}{}",
        weekday_name(sel.weekday()),
        month_name(sel.month()),
        sel.day(),
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

        div { class: "bg-surface-raised border border-border rounded-md p-[14px]",
            div { class: "flex items-baseline justify-between mb-[10px] pb-2 border-b border-border",
                span { class: "text-[16px] font-semibold text-foreground tracking-tight", "{sel_head}" }
                span { class: "font-mono text-xs text-subtle tracking-[var(--tracking-data)] uppercase", "{sel_count_str}" }
            }
            if sel_tasks.is_empty() {
                div { class: "text-center py-6 px-3 font-mono text-xs text-subtle tracking-[var(--tracking-data)] uppercase", "// quiet day · breathe" }
            } else {
                div { class: "flex flex-col",
                    for (title, color, cat) in sel_tasks {
                        div { class: "flex gap-[10px] items-start py-2 border-b border-border last:border-b-0",
                            div { class: "w-[3px] rounded-[99px] self-stretch flex-shrink-0 min-h-[18px]", style: "background:{color};" }
                            div { class: "flex-1 min-w-0",
                                div { class: "text-sm text-foreground leading-[1.4] mb-[2px]", "{title}" }
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

    // Compute timeline bounds from tasks in this week; fall back to defaults.
    let mut dyn_start = TIMELINE_START;
    let mut dyn_end = TIMELINE_END;
    for i in 0..7i64 {
        let date = monday + Duration::days(i);
        for task in props
            .tasks
            .iter()
            .filter(|t| t.schedule.naive_date() == Some(date))
        {
            match &task.schedule {
                TaskSchedule::At { starts_at } => {
                    let h = starts_at.hour();
                    if h < dyn_start {
                        dyn_start = h;
                    }
                    if h + 1 > dyn_end {
                        dyn_end = (h + 1).min(24);
                    }
                }
                TaskSchedule::Span {
                    starts_at,
                    duration_secs,
                } => {
                    let h = starts_at.hour();
                    let end_h = {
                        let total_mins = h * 60 + starts_at.minute() + (*duration_secs as u32) / 60;
                        total_mins.div_ceil(60).min(24)
                    };
                    if h < dyn_start {
                        dyn_start = h;
                    }
                    if end_h > dyn_end {
                        dyn_end = end_h;
                    }
                }
                _ => {}
            }
        }
    }

    let now_top: Option<f64> = {
        let h = now.hour() as f64;
        let m = now.minute() as f64;
        if h >= dyn_start as f64 && h < dyn_end as f64 {
            Some((h - dyn_start as f64 + m / 60.0) * HOUR_PX)
        } else {
            None
        }
    };

    let total_height = (dyn_end - dyn_start) as f64 * HOUR_PX;

    let header_data: Vec<(NaiveDate, bool)> = (0..7i64)
        .map(|i| {
            let d = monday + Duration::days(i);
            (d, d == today)
        })
        .collect();

    let mut allday_data: Vec<Vec<(String, String)>> = vec![];
    let mut timeline_data: Vec<(bool, Vec<TimedItem>)> = vec![];

    for i in 0..7i64 {
        let date = monday + Duration::days(i);
        let is_today = date == today;
        let mut allday: Vec<(String, String)> = vec![];
        let mut timed: Vec<TimedItem> = vec![];

        for task in props
            .tasks
            .iter()
            .filter(|t| t.schedule.naive_date() == Some(date))
        {
            let color = task_color(task);
            match &task.schedule {
                TaskSchedule::AllDay { .. } => {
                    allday.push((task.title.clone(), color));
                }
                TaskSchedule::At { starts_at } => {
                    let h = starts_at.hour() as f64;
                    let m = starts_at.minute() as f64;
                    if h >= dyn_start as f64 && h < dyn_end as f64 {
                        timed.push(TimedItem {
                            title: task.title.clone(),
                            color,
                            top_px: (h - dyn_start as f64 + m / 60.0) * HOUR_PX,
                            height_px: 28.0,
                        });
                    }
                }
                TaskSchedule::Span {
                    starts_at,
                    duration_secs,
                } => {
                    let h = starts_at.hour() as f64;
                    let m = starts_at.minute() as f64;
                    if h >= dyn_start as f64 && h < dyn_end as f64 {
                        timed.push(TimedItem {
                            title: task.title.clone(),
                            color,
                            top_px: (h - dyn_start as f64 + m / 60.0) * HOUR_PX,
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
        div { class: "flex flex-col min-h-0",

            div { class: "flex items-stretch border-b border-border",
                div { class: "w-[52px] flex-shrink-0" }
                for (date, is_today) in header_data {
                    div {
                        class: if is_today { "flex-1 flex flex-col items-center py-2 px-1 gap-[2px] border-l border-border bg-accent-soft" } else { "flex-1 flex flex-col items-center py-2 px-1 gap-[2px] border-l border-border" },
                        span { class: "font-mono text-[9px] text-subtle uppercase tracking-[var(--tracking-data)]", "{weekday_short(date.weekday())}" }
                        span { class: if is_today { "text-lg font-bold tracking-tight text-accent leading-none" } else { "text-lg font-bold tracking-tight text-foreground leading-none" }, "{date.day()}" }
                    }
                }
            }

            if has_allday {
                div { class: "flex border-b border-border min-h-[28px]",
                    div { class: "w-[52px] flex-shrink-0 flex items-center justify-end pr-2",
                        span { class: "font-mono text-[9px] text-subtle uppercase tracking-[var(--tracking-data)] leading-none text-right", "all day" }
                    }
                    for chips in allday_data {
                        div { class: "flex-1 py-1 px-[2px] flex flex-col gap-[2px] border-l border-border",
                            for (title, color) in chips {
                                div { class: "text-[10px] py-[1px] px-1 rounded-[3px] text-white truncate leading-[16px]", style: "background:{color};", "{title}" }
                            }
                        }
                    }
                }
            }

            div { class: "overflow-y-auto flex-1",
                div { class: "flex relative", style: "height:{total_height}px;",

                    div { class: "w-[52px] flex-shrink-0 flex items-start justify-end pr-2 relative",
                        for h in dyn_start..dyn_end {
                            span {
                                class: "week-hour-label",
                                style: "top:{(h - dyn_start) as f64 * HOUR_PX}px;",
                                "{h:02}:00"
                            }
                        }
                    }

                    div { class: "flex-1 flex",
                        for (is_today, timed) in timeline_data {
                            div {
                                class: if is_today { "flex-1 relative border-l border-border bg-[color-mix(in_srgb,var(--accent)_4%,transparent)]" } else { "flex-1 relative border-l border-border" },
                                for h in dyn_start..dyn_end {
                                    div {
                                        class: "week-gridline",
                                        style: "top:{(h - dyn_start) as f64 * HOUR_PX}px;",
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
