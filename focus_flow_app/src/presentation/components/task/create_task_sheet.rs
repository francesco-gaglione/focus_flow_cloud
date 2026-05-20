use chrono::{Datelike, Local, NaiveDate, NaiveTime, TimeZone, Timelike};
use dioxus::{logger::tracing::debug, prelude::*};
use shared::task::{TaskPriority, TaskScheduleDto};
use time::Date as TimeDate;

use crate::{
    components::{
        button::{Button, ButtonVariant},
        calendar::{
            Calendar, CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation,
            CalendarNextMonthButton, CalendarPreviousMonthButton, CalendarView,
        },
        input::Input,
        select::{Select, SelectList, SelectOption, SelectTrigger, SelectValue},
    },
    presentation::components::common_components::bottom_sheet::BottomSheet,
    use_cases::tasks::{
        create_task_uc::{CreateSubtask, CreateTaskCommand},
        task_list_uc::TodoCategory,
    },
};

const FIELD_LABEL: &str = "font-mono text-xs font-medium tracking-[0.02em] uppercase text-subtle";
const FIELD_HINT: &str = "font-mono text-[11px] text-subtle mt-0.5";

fn compute_duration_mins(start_str: &str, end_str: &str) -> i64 {
    let parse = |s: &str| NaiveTime::parse_from_str(s.trim(), "%H:%M").ok();
    if let (Some(start), Some(end)) = (parse(start_str), parse(end_str)) {
        let s = start.hour() as i64 * 60 + start.minute() as i64;
        let e = end.hour() as i64 * 60 + end.minute() as i64;
        if e > s {
            e - s
        } else {
            0
        }
    } else {
        0
    }
}

fn build_schedule(
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

#[derive(Props, Clone, PartialEq)]
pub struct CreateTaskSheetProps {
    pub show: bool,
    pub categories: Vec<TodoCategory>,
    pub on_submit: EventHandler<CreateTaskCommand>,
    pub on_close: EventHandler<()>,
}

#[component]
pub fn CreateTaskSheet(props: CreateTaskSheetProps) -> Element {
    let mut title = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut selected_cat_id = use_signal(String::new);
    let mut selected_date: Signal<Option<TimeDate>> = use_signal(|| None);
    let mut show_calendar = use_signal(|| false);
    let mut cal_view: Signal<TimeDate> = use_signal(|| {
        let now = Local::now().date_naive();
        TimeDate::from_calendar_date(
            now.year(),
            time::Month::try_from(now.month() as u8).unwrap_or(time::Month::January),
            now.day() as u8,
        )
        .unwrap_or(time::macros::date!(2026-01-01))
    });
    let mut due_time_str = use_signal(String::new);
    let mut due_end_time_str = use_signal(String::new);
    let mut is_all_day = use_signal(|| true);
    let mut selected_priority: Signal<Option<TaskPriority>> = use_signal(|| None);
    let mut subtask_input = use_signal(String::new);
    let mut subtasks: Signal<Vec<String>> = use_signal(Vec::new);

    let mut add_subtask = move || {
        let val = subtask_input.read().trim().to_string();
        if !val.is_empty() {
            subtasks.write().push(val);
            subtask_input.set(String::new());
        }
    };

    let on_close = props.on_close;
    let mut close = move || {
        title.set(String::new());
        description.set(String::new());
        selected_cat_id.set(String::new());
        selected_date.set(None);
        show_calendar.set(false);
        due_time_str.set(String::new());
        due_end_time_str.set(String::new());
        is_all_day.set(true);
        selected_priority.set(None);
        subtask_input.set(String::new());
        subtasks.write().clear();
        on_close.call(());
    };

    rsx! {
        BottomSheet {
            show: props.show,
            title: "New task".to_string(),
            on_close: move |_| close(),

            form {
                class: "flex flex-col gap-[18px] px-5 pt-5 pb-1",
                onsubmit: move |e| {
                    e.prevent_default();
                    let val = title.read().trim().to_string();
                    if val.is_empty() { return; }

                    let cat_id = selected_cat_id.read().clone();
                    let category_id = if cat_id.is_empty() { None } else { Some(cat_id) };

                    let schedule = (*selected_date.read()).map(|d| {
                        build_schedule(d, due_time_str.read().trim(), *is_all_day.read(), due_end_time_str.read().trim())
                    });

                    let command = CreateTaskCommand {
                        title: val,
                        description: {
                            let d = description.read().trim().to_string();
                            if d.is_empty() { None } else { Some(d) }
                        },
                        schedule,
                        category_id,
                        priority: *selected_priority.read(),
                        subtasks: subtasks
                            .read()
                            .iter()
                            .map(|t| CreateSubtask { title: t.clone(), description: None })
                            .collect(),
                    };

                    props.on_submit.call(command);
                    title.set(String::new());
                    description.set(String::new());
                    selected_cat_id.set(String::new());
                    selected_date.set(None);
                    show_calendar.set(false);
                    due_time_str.set(String::new());
                    due_end_time_str.set(String::new());
                    is_all_day.set(true);
                    selected_priority.set(None);
                    subtask_input.set(String::new());
                    subtasks.write().clear();
                },

                div { class: "flex flex-col gap-1.5",
                    label { class: FIELD_LABEL, "Task name" }
                    Input {
                        placeholder: "What needs to be done?",
                        value: "{title}",
                        oninput: move |e: FormEvent| title.set(e.value()),
                    }
                }

                div { class: "flex flex-col gap-1.5",
                    label { class: FIELD_LABEL, "Notes" }
                    Input {
                        placeholder: "Add extra details or context…",
                        value: "{description}",
                        oninput: move |e: FormEvent| description.set(e.value()),
                    }
                }

                div { class: "flex flex-col gap-1.5",
                    label { class: FIELD_LABEL, "Schedule" }
                    div { class: "flex gap-2 items-center flex-wrap",
                        button {
                            r#type: "button",
                            class: "h-9 px-3 bg-surface-card border border-border rounded-md text-sm font-sans cursor-pointer text-foreground transition-[border-color] duration-fast ease-tech hover:border-accent",
                            onclick: move |_| {
                                let cur = *show_calendar.read();
                                show_calendar.set(!cur);
                            },
                            if let Some(d) = *selected_date.read() {
                                "{d.day()} {d.month()} {d.year()}"
                            } else {
                                "Pick a date"
                            }
                        }
                        if selected_date.read().is_some() {
                            button {
                                r#type: "button",
                                class: "size-9 grid place-items-center bg-surface-card border border-border rounded-md text-subtle cursor-pointer text-base hover:text-foreground transition-colors duration-fast",
                                onclick: move |_| {
                                    selected_date.set(None);
                                    show_calendar.set(false);
                                },
                                "×"
                            }
                            label { class: "flex items-center gap-1.5 shrink-0 cursor-pointer",
                                input {
                                    r#type: "checkbox",
                                    class: "accent-accent w-[14px] h-[14px] cursor-pointer",
                                    checked: *is_all_day.read(),
                                    oninput: move |e: FormEvent| {
                                        is_all_day.set(e.value() == "true");
                                        if e.value() == "true" {
                                            due_time_str.set(String::new());
                                            due_end_time_str.set(String::new());
                                        }
                                    },
                                }
                                span { class: "font-mono text-[11px] text-subtle uppercase tracking-[0.04em]", "All day" }
                            }
                        }
                    }
                    if *show_calendar.read() {
                        Calendar {
                            selected_date: ReadSignal::new(selected_date),
                            on_date_change: move |d: Option<TimeDate>| {
                                selected_date.set(d);
                                show_calendar.set(false);
                            },
                            view_date: ReadSignal::new(cal_view),
                            on_view_change: move |d: TimeDate| cal_view.set(d),
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
                    if selected_date.read().is_some() && !*is_all_day.read() {
                        div { class: "flex gap-3 items-end mt-1",
                            div { class: "flex flex-col gap-1",
                                span { class: "font-mono text-[10px] text-subtle uppercase tracking-[0.04em]", "From" }
                                input {
                                    class: "w-[110px] h-10 px-[14px] bg-surface-card border border-border rounded-md text-foreground font-mono text-sm outline-none transition-[border-color,box-shadow] duration-fast ease-tech focus:border-accent focus:[box-shadow:var(--shadow-focus)] [color-scheme:dark]",
                                    r#type: "time",
                                    value: "{due_time_str}",
                                    oninput: move |e| due_time_str.set(e.value()),
                                }
                            }
                            div { class: "flex flex-col gap-1",
                                span { class: "font-mono text-[10px] text-subtle uppercase tracking-[0.04em]", "To" }
                                input {
                                    class: "w-[110px] h-10 px-[14px] bg-surface-card border border-border rounded-md text-foreground font-mono text-sm outline-none transition-[border-color,box-shadow] duration-fast ease-tech focus:border-accent focus:[box-shadow:var(--shadow-focus)] [color-scheme:dark]",
                                    r#type: "time",
                                    placeholder: "optional",
                                    value: "{due_end_time_str}",
                                    oninput: move |e| due_end_time_str.set(e.value()),
                                }
                            }
                        }
                        p { class: FIELD_HINT, "Leave \"To\" empty to mark a point in time with no duration." }
                    }
                }

                div { class: "flex gap-3",
                    div { class: "flex-1 min-w-0 flex flex-col gap-1.5",
                        label { class: FIELD_LABEL, "Category" }
                        Select::<String> {
                            default_value: None,
                            on_value_change: move |v: Option<String>| {
                                debug!("selected_cat_id: {:?}", v);
                                selected_cat_id.set(v.unwrap_or_default());
                            },
                            SelectTrigger {
                                SelectValue { placeholder: "No category" }
                            }
                            SelectList {
                                for (i, cat) in props.categories.iter().enumerate() {
                                    SelectOption::<String> {
                                        index: i,
                                        value: cat.id.clone(),
                                        text_value: cat.name.clone(),
                                        "@{cat.name}"
                                    }
                                }
                            }
                        }
                    }

                    div { class: "flex-1 min-w-0 flex flex-col gap-1.5",
                        label { class: FIELD_LABEL, "Priority" }
                        Select::<String> {
                            default_value: None,
                            on_value_change: move |v: Option<String>| {
                                debug!("priority on_value_change: {:?}", v);
                                selected_priority.set(v.as_deref().and_then(|s| match s {
                                    "low" => Some(TaskPriority::Low),
                                    "medium" => Some(TaskPriority::Medium),
                                    "high" => Some(TaskPriority::High),
                                    "urgent" => Some(TaskPriority::Urgent),
                                    _ => None,
                                }));
                            },
                            SelectTrigger {
                                SelectValue { placeholder: "None" }
                            }
                            SelectList {
                                SelectOption::<String> { index: 0_usize, value: "low".to_string(), text_value: "Low", "Low" }
                                SelectOption::<String> { index: 1_usize, value: "medium".to_string(), text_value: "Medium", "Medium" }
                                SelectOption::<String> { index: 2_usize, value: "high".to_string(), text_value: "High", "High" }
                                SelectOption::<String> { index: 3_usize, value: "urgent".to_string(), text_value: "Urgent", "Urgent" }
                            }
                        }
                    }
                }

                div { class: "flex flex-col gap-1.5",
                    label { class: FIELD_LABEL, "Subtasks" }
                    for (i, sub) in subtasks.read().clone().into_iter().enumerate() {
                        div { class: "flex items-center gap-2 px-3 py-2 bg-surface-card border border-border rounded-md mb-1",
                            span { class: "flex-1 text-sm text-muted leading-[1.4]", "{sub}" }
                            button {
                                r#type: "button",
                                class: "size-6 bg-transparent border-0 text-subtle cursor-pointer text-lg leading-none rounded-sm grid place-items-center shrink-0 transition-colors duration-fast ease-tech hover:text-danger",
                                onclick: move |_| { subtasks.write().remove(i); },
                                "×"
                            }
                        }
                    }
                    div { class: "flex gap-2 items-center mt-1",
                        input {
                            class: "flex-1 h-[38px] px-[14px] bg-surface-card border border-border rounded-md text-foreground font-sans text-sm outline-none transition-[border-color,box-shadow] duration-fast ease-tech placeholder:text-subtle focus:border-accent focus:[box-shadow:var(--shadow-focus)]",
                            placeholder: "Add a subtask…",
                            value: "{subtask_input}",
                            oninput: move |e| subtask_input.set(e.value()),
                            onkeydown: move |e| {
                                if e.key() == Key::Enter {
                                    e.prevent_default();
                                    add_subtask();
                                }
                            },
                        }
                        button {
                            r#type: "button",
                            class: "size-[38px] shrink-0 bg-surface-card border border-border rounded-md text-accent cursor-pointer grid place-items-center transition-[background,border-color] duration-fast ease-tech hover:bg-accent-soft hover:border-accent",
                            onclick: move |_| add_subtask(),
                            svg { view_box: "0 0 16 16", width: "14", height: "14", stroke: "currentColor", fill: "none", stroke_width: "1.8", stroke_linecap: "round",
                                line { x1: "8", y1: "3", x2: "8", y2: "13" }
                                line { x1: "3", y1: "8", x2: "13", y2: "8" }
                            }
                        }
                    }
                }

                div { class: "flex gap-2 pt-1",
                    Button {
                        variant: ButtonVariant::Outline,
                        r#type: "button",
                        style: "flex: 1;",
                        onclick: move |_| close(),
                        "Cancel"
                    }
                    Button {
                        variant: ButtonVariant::Primary,
                        r#type: "submit",
                        style: "flex: 2; justify-content: center;",
                        "Add task"
                    }
                }
            }
        }
    }
}
