use chrono::{Local, NaiveDateTime, NaiveTime, TimeZone, Utc};
use dioxus::prelude::*;
use time::Date as TimeDate;

use crate::{
    components::{
        date_picker::{DatePicker, DatePickerInput},
        select::{Select, SelectList, SelectOption, SelectTrigger, SelectValue},
    },
    presentation::components::common_components::bottom_sheet::BottomSheet,
    use_cases::tasks::{
        create_task_uc::{CreateSubtask, CreateTaskCommand},
        task_list_uc::TodoCategory,
    },
};

#[derive(Props, Clone, PartialEq)]
pub struct CreateTaskSheetProps {
    pub show: bool,
    pub categories: Vec<TodoCategory>,
    pub on_submit: EventHandler<CreateTaskCommand>,
    pub on_close: EventHandler<()>,
}

fn time_date_to_utc(date: TimeDate, time_str: &str) -> chrono::DateTime<Utc> {
    let naive_date =
        chrono::NaiveDate::from_ymd_opt(date.year(), date.month() as u32, date.day() as u32)
            .unwrap_or_else(|| Local::now().date_naive());

    let naive_time = if time_str.is_empty() {
        NaiveTime::from_hms_opt(0, 0, 0).unwrap()
    } else {
        NaiveTime::parse_from_str(time_str, "%H:%M")
            .unwrap_or_else(|_| NaiveTime::from_hms_opt(0, 0, 0).unwrap())
    };

    Local
        .from_local_datetime(&NaiveDateTime::new(naive_date, naive_time))
        .single()
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|| Utc::now())
}

#[component]
pub fn CreateTaskSheet(props: CreateTaskSheetProps) -> Element {
    let mut title = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut selected_cat_id = use_signal(String::new);
    let mut selected_date: Signal<Option<TimeDate>> = use_signal(|| None);
    let mut due_time_str = use_signal(String::new);
    let mut subtask_input = use_signal(String::new);
    let mut subtasks: Signal<Vec<String>> = use_signal(Vec::new);

    let mut add_subtask = move || {
        let val = subtask_input.read().trim().to_string();
        if !val.is_empty() {
            subtasks.write().push(val);
            subtask_input.set(String::new());
        }
    };

    let on_close = props.on_close.clone();
    let mut close = move || {
        title.set(String::new());
        description.set(String::new());
        selected_cat_id.set(String::new());
        selected_date.set(None);
        due_time_str.set(String::new());
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
                class: "sheet-form",
                onsubmit: move |e| {
                    e.prevent_default();
                    let val = title.read().trim().to_string();
                    if val.is_empty() { return; }

                    let cat_id = selected_cat_id.read().clone();
                    let category_id = if cat_id.is_empty() { None } else { Some(cat_id) };

                    let due_date = (*selected_date.read()).map(|d| {
                        time_date_to_utc(d, due_time_str.read().trim())
                    });

                    let command = CreateTaskCommand {
                        title: val,
                        description: {
                            let d = description.read().trim().to_string();
                            if d.is_empty() { None } else { Some(d) }
                        },
                        due_date,
                        category_id,
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
                    due_time_str.set(String::new());
                    subtask_input.set(String::new());
                    subtasks.write().clear();
                },

                div { class: "sheet-field",
                    label { class: "sheet-label", "Title" }
                    input {
                        class: "sheet-input",
                        placeholder: "What needs to be done?",
                        value: "{title}",
                        oninput: move |e| title.set(e.value()),
                    }
                }

                div { class: "sheet-field",
                    label { class: "sheet-label", "Description" }
                    input {
                        class: "sheet-input",
                        placeholder: "Optional details…",
                        value: "{description}",
                        oninput: move |e| description.set(e.value()),
                    }
                }

                div { class: "sheet-field",
                    label { class: "sheet-label", "Due date" }
                    div { class: "sheet-datetime-row",
                        DatePicker {
                            selected_date: ReadSignal::new(selected_date),
                            on_value_change: move |d: Option<TimeDate>| selected_date.set(d),
                            DatePickerInput {}
                        }
                        input {
                            class: "sheet-input sheet-time",
                            r#type: "time",
                            value: "{due_time_str}",
                            oninput: move |e| due_time_str.set(e.value()),
                        }
                    }
                }

                div { class: "sheet-field",
                    label { class: "sheet-label", "Category" }
                    Select::<String> {
                        default_value: None,
                        on_value_change: move |v: Option<String>| {
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

                div { class: "sheet-field",
                    label { class: "sheet-label", "Subtasks" }
                    for (i, sub) in subtasks.read().clone().into_iter().enumerate() {
                        div { class: "sheet-subtask-row",
                            span { class: "sheet-subtask-title", "{sub}" }
                            button {
                                r#type: "button",
                                class: "sheet-subtask-rm",
                                onclick: move |_| { subtasks.write().remove(i); },
                                "×"
                            }
                        }
                    }
                    div { class: "sheet-subtask-add",
                        input {
                            class: "sheet-input",
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
                            class: "sheet-subtask-add-btn",
                            onclick: move |_| add_subtask(),
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
                        onclick: move |_| close(),
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
