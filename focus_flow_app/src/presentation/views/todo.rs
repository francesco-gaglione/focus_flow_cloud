use std::time::Duration;

use chrono::{Datelike, Local, NaiveDate, NaiveTime, TimeZone};
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};
use shared::task::{TaskPriority, TaskScheduleDto};
use time::Date as TimeDate;

use crate::{
    components::{
        button::{Button, ButtonVariant},
        calendar::{
            Calendar, CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation,
            CalendarNextMonthButton, CalendarPreviousMonthButton, CalendarView,
        },
        select::{Select, SelectList, SelectOption, SelectTrigger, SelectValue},
    },
    i18n::use_i18n,
    presentation::components::{
        common_components::bottom_sheet::BottomSheet,
        task::{create_task_sheet::CreateTaskSheet, task_row::TaskRow},
    },
    use_cases::tasks::{
        create_subtask_uc::create_subtask_uc,
        create_task_uc::{create_task_uc, CreateTaskCommand},
        delete_task_uc::delete_task_uc,
        task_list_uc::{task_list_uc, TaskDue, TaskSchedule, TodoCategory, TodoTask},
        update_subtask_completition_uc::update_subtask_completition_uc,
        update_task_completition_uc::update_task_completition_uc,
        update_task_due_date_uc::update_task_due_date_uc,
        update_task_priority_uc::update_task_priority_uc,
    },
    Route,
};

fn schedule_to_time_date(schedule: &TaskSchedule) -> Option<TimeDate> {
    let naive = schedule.naive_date()?;
    time::Month::try_from(naive.month() as u8)
        .ok()
        .and_then(|m| TimeDate::from_calendar_date(naive.year(), m, naive.day() as u8).ok())
}

fn compute_duration_mins(start_str: &str, end_str: &str) -> i64 {
    let parse = |s: &str| NaiveTime::parse_from_str(s.trim(), "%H:%M").ok();
    if let (Some(start), Some(end)) = (parse(start_str), parse(end_str)) {
        use chrono::Timelike;
        let s = start.hour() as i64 * 60 + start.minute() as i64;
        let e = end.hour() as i64 * 60 + end.minute() as i64;
        if e > s { e - s } else { 0 }
    } else {
        0
    }
}

fn build_schedule_dto(
    date: TimeDate,
    start_str: &str,
    is_all_day: bool,
    end_str: &str,
) -> TaskScheduleDto {
    let naive_date = NaiveDate::from_ymd_opt(date.year(), date.month() as u32, date.day() as u32)
        .unwrap_or_else(|| Local::now().date_naive());

    if is_all_day || start_str.trim().is_empty() {
        let ts = naive_date
            .and_hms_opt(0, 0, 0)
            .map(|ndt| {
                chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(ndt, chrono::Utc)
                    .timestamp()
            })
            .unwrap_or(0);
        TaskScheduleDto::AllDay { date: ts }
    } else {
        let naive_time = NaiveTime::parse_from_str(start_str.trim(), "%H:%M")
            .unwrap_or_else(|_| NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        let ts = Local
            .from_local_datetime(&naive_date.and_time(naive_time))
            .single()
            .map(|dt| dt.timestamp())
            .unwrap_or_else(|| chrono::Utc::now().timestamp());
        let duration_mins = compute_duration_mins(start_str, end_str);
        if duration_mins > 0 {
            TaskScheduleDto::Span {
                starts_at: ts,
                duration: duration_mins * 60,
            }
        } else {
            TaskScheduleDto::At { starts_at: ts }
        }
    }
}

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
    let i18n = use_i18n();

    let mut prio_sheet: Signal<Option<(String, Option<TaskPriority>)>> =
        use_context_provider(|| Signal::new(None));
    let mut date_sheet: Signal<Option<(String, TaskSchedule)>> =
        use_context_provider(|| Signal::new(None));

    let mut picker_date: Signal<Option<TimeDate>> = use_signal(|| None);
    let mut picker_time: Signal<String> = use_signal(String::new);
    let mut picker_end_time: Signal<String> = use_signal(String::new);
    let mut picker_is_all_day: Signal<bool> = use_signal(|| true);
    let mut picker_show_calendar = use_signal(|| false);
    let mut picker_cal_view: Signal<TimeDate> = use_signal(|| {
        let now = Local::now().date_naive();
        TimeDate::from_calendar_date(
            now.year(),
            time::Month::try_from(now.month() as u8).unwrap_or(time::Month::January),
            now.day() as u8,
        )
        .unwrap_or(time::macros::date!(2026-01-01))
    });

    use_effect(move || {
        picker_show_calendar.set(false);
        if let Some((_, schedule)) = date_sheet.read().as_ref() {
            let date = schedule_to_time_date(schedule);
            picker_date.set(date);
            picker_is_all_day.set(schedule.is_all_day() || !schedule.is_scheduled());
            picker_time.set(schedule.time_str().unwrap_or_default());
            picker_end_time.set(schedule.end_time_str().unwrap_or_default());
            if let Some(d) = date {
                picker_cal_view.set(d);
            }
        }
    });

    let mut fetch_task_list = use_resource(move || async move {
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
                "today"    => !t.done && matches!(t.due, TaskDue::Today(_)),
                "upcoming" => !t.done && matches!(t.due, TaskDue::Upcoming(_) | TaskDue::Tomorrow(_)),
                "done"     => t.done,
                _          => true,
            };
            let cat_ok = {
                let filter = cat_filter.read();
                filter.as_str() == "all" || t.cat.as_deref() == Some(filter.as_str())
            };
            period_ok && cat_ok
        })
        .cloned()
        .collect();

    let overdue: Vec<TodoTask> = filtered.iter().filter(|t| !t.done && matches!(t.due, TaskDue::Overdue(_))).cloned().collect();
    let today_tasks: Vec<TodoTask> = filtered.iter().filter(|t| !t.done && matches!(t.due, TaskDue::Today(_))).cloned().collect();
    let upcoming_tasks: Vec<TodoTask> = filtered.iter().filter(|t| !t.done && matches!(t.due, TaskDue::Upcoming(_) | TaskDue::Tomorrow(_))).cloned().collect();
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
                                i18n.read().t("todo.toast_uncompleted_subtasks"),
                                ToastOptions::new()
                                    .description(i18n.read().t("todo.toast_complete_subtasks_first"))
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

    let complete_subtask_handler = move |(task_id, subtask_id, completed): (String, String, bool)| {
        spawn(async move {
            match update_subtask_completition_uc(task_id, subtask_id, Some(completed)).await {
                Ok(_) => { info!("Subtask completed"); fetch_task_list.restart(); }
                Err(e) => { error!("Error completing subtask: {}", e.to_string()); }
            }
        });
    };

    let delete_task_handler = move |id: String| {
        spawn(async move {
            match delete_task_uc(id).await {
                Ok(_) => { info!("Task deleted"); fetch_task_list.restart(); }
                Err(e) => { error!("Error deleting a task: {}", e.to_string()); }
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
                Ok(_) => { info!("Subtask created"); fetch_task_list.restart(); }
                Err(e) => { error!("Error creating subtask: {}", e); }
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
                        SelectValue { placeholder: i18n.read().t("todo.filter_all_periods") }
                    }
                    SelectList {
                        SelectOption::<String> { index: 0_usize, value: "all".to_string(),      text_value: i18n.read().t("todo.filter_all_periods"), "{i18n.read().t(\"todo.filter_all_periods\")}" }
                        SelectOption::<String> { index: 1_usize, value: "today".to_string(),    text_value: i18n.read().t("todo.filter_today"),       "{i18n.read().t(\"todo.filter_today\")}" }
                        SelectOption::<String> { index: 2_usize, value: "upcoming".to_string(), text_value: i18n.read().t("todo.filter_upcoming"),    "{i18n.read().t(\"todo.filter_upcoming\")}" }
                        SelectOption::<String> { index: 3_usize, value: "done".to_string(),     text_value: i18n.read().t("todo.filter_done"),        "{i18n.read().t(\"todo.filter_done\")}" }
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
                        SelectValue { placeholder: i18n.read().t("todo.filter_all_categories") }
                    }
                    SelectList {
                        SelectOption::<String> { index: 0_usize, value: "all".to_string(), text_value: i18n.read().t("todo.filter_all_categories"), "{i18n.read().t(\"todo.filter_all_categories\")}" }
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
                    p { "{i18n.read().t(\"todo.loading\")}" }
                }
            } else if let Some(err) = load_error.read().as_ref() {
                div { class: "empty-state",
                    p { "{i18n.read().tf(\"todo.failed_load\", &[err])}" }
                }
            } else if filtered.is_empty() {
                div { class: "empty-state",
                    div { class: "ico",
                        svg { view_box: "0 0 16 16",
                            path { d: "M3 8l3 3 7-7", stroke: "currentColor", stroke_width: "1.5", fill: "none" }
                        }
                    }
                    h3 { "{i18n.read().t(\"todo.empty_title\")}" }
                    p { "{i18n.read().t(\"todo.empty_desc\")}" }
                }
            } else if show_sections {
                if !overdue.is_empty() {
                    TaskSection { label: i18n.read().t("todo.section_overdue"), modifier: "danger", tasks: overdue, on_toggle: complete_task_toggle, on_subtask_toggle: complete_subtask_handler, on_delete: delete_task_handler, on_start_timer: start_timer_handler, on_add_subtask: add_subtask_handler }
                }
                if !today_tasks.is_empty() {
                    TaskSection { label: i18n.read().t("todo.section_today"), modifier: "today", tasks: today_tasks, on_toggle: complete_task_toggle, on_subtask_toggle: complete_subtask_handler, on_delete: delete_task_handler, on_start_timer: start_timer_handler, on_add_subtask: add_subtask_handler }
                }
                if !upcoming_tasks.is_empty() {
                    TaskSection { label: i18n.read().t("todo.section_upcoming"), modifier: "", tasks: upcoming_tasks, on_toggle: complete_task_toggle, on_subtask_toggle: complete_subtask_handler, on_delete: delete_task_handler, on_start_timer: start_timer_handler, on_add_subtask: add_subtask_handler }
                }
                if !done_tasks.is_empty() {
                    TaskSection {
                        label: i18n.read().t("todo.section_done"),
                        modifier: "",
                        tasks: done_tasks.clone(),
                        on_toggle: complete_task_toggle,
                        on_subtask_toggle: complete_subtask_handler,
                        on_delete: delete_task_handler,
                        on_start_timer: start_timer_handler,
                        on_add_subtask: add_subtask_handler,
                        on_delete_all: move |_| {
                            let ids: Vec<String> = done_tasks.iter().map(|t| t.id.clone()).collect();
                            spawn(async move {
                                for id in ids {
                                    if let Err(e) = delete_task_uc(id).await {
                                        error!("Error deleting done task: {}", e);
                                    }
                                }
                                fetch_task_list.restart();
                            });
                        },
                    }
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
            span { "{i18n.read().t(\"todo.new_task_btn\")}" }
        }

        CreateTaskSheet {
            show: *show_modal.read(),
            categories: categories.read().clone(),
            on_submit: move |dto: CreateTaskCommand| {
                spawn(async move {
                    match create_task_uc(dto).await {
                        Ok(_) => { show_modal.set(false); fetch_task_list.restart(); }
                        Err(e) => { error!("Error creating task: {}", e); }
                    }
                });
            },
            on_close: move |_| show_modal.set(false),
        }

        // ── Date picker sheet ─────────────────────────────────────────────
        {
            let sheet = date_sheet.read().clone();
            let tid = sheet.as_ref().map(|(id, _)| id.clone()).unwrap_or_default();
            let tid_clear = tid.clone();
            rsx! {
                BottomSheet {
                    show: sheet.is_some(),
                    title: i18n.read().t("todo.sheet_schedule_title"),
                    on_close: move |_| date_sheet.set(None),
                    div { class: "date-sheet-body",
                        div { class: "date-sheet-section",
                            span { class: "date-sheet-section-label", "{i18n.read().t(\"todo.date_label\")}" }
                            div { class: "flex gap-2 items-center",
                                button {
                                    r#type: "button",
                                    class: "h-9 px-3 bg-surface-card border border-border rounded-md text-sm font-sans cursor-pointer text-foreground transition-[border-color] duration-fast ease-tech hover:border-accent",
                                    onclick: move |_| {
                                        let cur = *picker_show_calendar.read();
                                        picker_show_calendar.set(!cur);
                                    },
                                    if let Some(d) = *picker_date.read() {
                                        "{d.day()} {d.month()} {d.year()}"
                                    } else {
                                        "{i18n.read().t(\"todo.pick_date\")}"
                                    }
                                }
                                if picker_date.read().is_some() {
                                    button {
                                        r#type: "button",
                                        class: "size-9 grid place-items-center bg-surface-card border border-border rounded-md text-subtle cursor-pointer text-base hover:text-foreground transition-colors duration-fast",
                                        onclick: move |_| {
                                            picker_date.set(None);
                                            picker_show_calendar.set(false);
                                        },
                                        "×"
                                    }
                                }
                            }
                            if *picker_show_calendar.read() {
                                Calendar {
                                    selected_date: ReadSignal::new(picker_date),
                                    on_date_change: move |d: Option<TimeDate>| {
                                        picker_date.set(d);
                                        picker_show_calendar.set(false);
                                    },
                                    view_date: ReadSignal::new(picker_cal_view),
                                    on_view_change: move |d: TimeDate| picker_cal_view.set(d),
                                    CalendarView {
                                        CalendarHeader {
                                            CalendarNavigation {
                                                CalendarPreviousMonthButton {}
                                                CalendarMonthTitle { class: "dx-calendar-month-title" }
                                                CalendarNextMonthButton {}
                                            }
                                        }
                                        CalendarGrid {}
                                    }
                                }
                            }
                        }
                        if picker_date.read().is_some() {
                            div { class: "date-sheet-section",
                                label { class: "flex items-center gap-2 cursor-pointer",
                                    input {
                                        r#type: "checkbox",
                                        class: "accent-accent w-4 h-4 cursor-pointer",
                                        checked: *picker_is_all_day.read(),
                                        oninput: move |e: FormEvent| {
                                            picker_is_all_day.set(e.value() == "true");
                                            if e.value() == "true" {
                                                picker_time.set(String::new());
                                                picker_end_time.set(String::new());
                                            }
                                        },
                                    }
                                    span { class: "date-sheet-section-label", "{i18n.read().t(\"todo.all_day\")}" }
                                }
                            }
                            div { class: "flex flex-row",
                                if !*picker_is_all_day.read() {
                                    div { class: "date-sheet-section",
                                        span { class: "date-sheet-section-label", "{i18n.read().t(\"todo.from_label\")}" }
                                        input {
                                            class: "date-sheet-time-input",
                                            r#type: "time",
                                            value: "{picker_time}",
                                            oninput: move |e| picker_time.set(e.value()),
                                        }
                                    }
                                    div { class: "date-sheet-section",
                                        span { class: "date-sheet-section-label", "{i18n.read().t(\"todo.to_label\")}" }
                                        input {
                                            class: "date-sheet-time-input",
                                            r#type: "time",
                                            placeholder: i18n.read().t("todo.time_optional"),
                                            value: "{picker_end_time}",
                                            oninput: move |e| picker_end_time.set(e.value()),
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "date-sheet-actions",
                        Button {
                            variant: ButtonVariant::Outline,
                            r#type: "button",
                            onclick: move |_| {
                                let t = tid_clear.clone();
                                spawn(async move {
                                    match update_task_due_date_uc(&t, Some(TaskScheduleDto::Unscheduled)).await {
                                        Ok(_) => { fetch_task_list.restart(); date_sheet.set(None); }
                                        Err(e) => error!("Error clearing schedule: {}", e),
                                    }
                                });
                            },
                            "{i18n.read().t(\"todo.clear\")}"
                        }
                        Button {
                            variant: ButtonVariant::Primary,
                            r#type: "button",
                            disabled: picker_date.read().is_none(),
                            onclick: move |_| {
                                if let Some(d) = *picker_date.read() {
                                    let tid2 = tid.clone();
                                    let time_str = picker_time.read().clone();
                                    let end_str = picker_end_time.read().clone();
                                    let all_day = *picker_is_all_day.read();
                                    let schedule = build_schedule_dto(d, &time_str, all_day, &end_str);
                                    spawn(async move {
                                        match update_task_due_date_uc(&tid2, Some(schedule)).await {
                                            Ok(_) => { fetch_task_list.restart(); date_sheet.set(None); }
                                            Err(e) => error!("Error updating schedule: {}", e),
                                        }
                                    });
                                }
                            },
                            "{i18n.read().t(\"todo.confirm\")}"
                        }
                    }
                }
            }
        }

        // ── Priority picker sheet ─────────────────────────────────────────
        {
            let sheet_state = prio_sheet.read().clone();
            let task_id = sheet_state.as_ref().map(|(id, _)| id.clone()).unwrap_or_default();
            let current = sheet_state.as_ref().map(|(_, p)| *p).unwrap_or(None);
            rsx! {
                BottomSheet {
                    show: sheet_state.is_some(),
                    title: i18n.read().t("todo.sheet_priority_title"),
                    on_close: move |_| prio_sheet.set(None),
                    div { class: "prio-sheet-options p-5",
                        for (variant, key, class_mod) in [
                            (None,                        "todo.priority_none",   "none"),
                            (Some(TaskPriority::Low),     "todo.priority_low",    "low"),
                            (Some(TaskPriority::Medium),  "todo.priority_medium", "medium"),
                            (Some(TaskPriority::High),    "todo.priority_high",   "high"),
                            (Some(TaskPriority::Urgent),  "todo.priority_urgent", "urgent"),
                        ] {
                            {
                                let tid = task_id.clone();
                                let label = i18n.read().t(key);
                                rsx! {
                                    button {
                                        class: "prio-sheet-btn prio-sheet-{class_mod}",
                                        r#type: "button",
                                        onclick: move |_| {
                                            let tid2 = tid.clone();
                                            spawn(async move {
                                                match update_task_priority_uc(&tid2, variant).await {
                                                    Ok(_) => { info!("Priority updated"); fetch_task_list.restart(); }
                                                    Err(e) => { error!("Error updating task priority: {}", e.to_string()); }
                                                }
                                            });
                                            prio_sheet.set(None);
                                        },
                                        span { class: "prio-sheet-dot" }
                                        "{label}"
                                        if current == variant {
                                            span { class: "prio-sheet-check", "✓" }
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
}

#[derive(Props, Clone, PartialEq)]
struct TaskSectionProps {
    label: String,
    modifier: &'static str,
    tasks: Vec<TodoTask>,
    on_toggle: EventHandler<(String, bool)>,
    on_subtask_toggle: EventHandler<(String, String, bool)>,
    on_delete: EventHandler<String>,
    on_start_timer: EventHandler<(String, String)>,
    on_add_subtask: EventHandler<(String, String)>,
    #[props(default)]
    on_delete_all: Option<EventHandler<()>>,
}

#[component]
fn TaskSection(props: TaskSectionProps) -> Element {
    let i18n = use_i18n();
    let count = props.tasks.len();
    let label_class = format!("lbl {}", props.modifier);
    let word = if count == 1 {
        i18n.read().t("todo.task_singular")
    } else {
        i18n.read().t("todo.task_plural")
    };
    rsx! {
        div {
            div { class: "section-head",
                span { class: "{label_class}", "{props.label}" }
                div { class: "section-head-right",
                    span { class: "count", "{count} {word}" }
                    if let Some(on_delete_all) = props.on_delete_all {
                        Button {
                            variant: ButtonVariant::Destructive,
                            r#type: "button",
                            style: "padding: 4px 10px; font-size: 0.75rem;",
                            onclick: move |_| on_delete_all.call(()),
                            "{i18n.read().t(\"todo.delete_all\")}"
                        }
                    }
                }
            }
            for task in props.tasks.iter() {
                TaskRow { task: task.clone(), on_toggle: props.on_toggle, on_subtask_toggle: props.on_subtask_toggle, on_delete: props.on_delete, on_start_timer: props.on_start_timer, on_add_subtask: props.on_add_subtask }
            }
        }
    }
}
