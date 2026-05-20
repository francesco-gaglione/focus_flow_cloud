use crate::components::icon::Icon;
use dioxus::prelude::*;
use dioxus_primitives::calendar::{
    self, CalendarDayProps, CalendarGridProps, CalendarHeaderProps, CalendarMonthTitleProps,
    CalendarNavigationProps, CalendarProps, CalendarSelectMonthProps, CalendarSelectYearProps,
    RangeCalendarProps,
};

#[component]
pub fn Calendar(props: CalendarProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        calendar::Calendar {
            class: "dx-calendar",
            selected_date: props.selected_date,
            on_date_change: props.on_date_change,
            on_format_weekday: props.on_format_weekday,
            on_format_month: props.on_format_month,
            view_date: props.view_date,
            today: props.today,
            on_view_change: props.on_view_change,
            disabled: props.disabled,
            first_day_of_week: props.first_day_of_week,
            min_date: props.min_date,
            max_date: props.max_date,
            disabled_ranges: props.disabled_ranges,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn RangeCalendar(props: RangeCalendarProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        calendar::RangeCalendar {
            class: "dx-calendar",
            selected_range: props.selected_range,
            on_range_change: props.on_range_change,
            on_format_weekday: props.on_format_weekday,
            on_format_month: props.on_format_month,
            view_date: props.view_date,
            today: props.today,
            on_view_change: props.on_view_change,
            disabled: props.disabled,
            first_day_of_week: props.first_day_of_week,
            min_date: props.min_date,
            max_date: props.max_date,
            disabled_ranges: props.disabled_ranges,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn CalendarView(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        calendar::CalendarView {
            attributes,
            {children}
        }
    }
}

#[component]
pub fn CalendarHeader(props: CalendarHeaderProps) -> Element {
    rsx! {
        calendar::CalendarHeader { id: props.id, attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn CalendarNavigation(props: CalendarNavigationProps) -> Element {
    rsx! {
        calendar::CalendarNavigation { class: "dx-calendar-navigation", {props.children} }
    }
}

#[component]
pub fn CalendarPreviousMonthButton(
    #[props(extends = GlobalAttributes)] _attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        calendar::CalendarPreviousMonthButton { class: "dx-calendar-nav-prev",
            Icon {
                class: "dx-calendar-previous-month-icon",
                width: "20px",
                height: "20px",
                path { d: "m15 18-6-6 6-6" }
            }
        }
    }
}

#[component]
pub fn CalendarNextMonthButton(
    #[props(extends = GlobalAttributes)] _attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        calendar::CalendarNextMonthButton { class: "dx-calendar-nav-next",
            Icon {
                class: "dx-calendar-next-month-icon",
                width: "20px",
                height: "20px",
                path { d: "m9 18 6-6-6-6" }
            }
        }
    }
}

#[component]
pub fn CalendarSelectMonth(props: CalendarSelectMonthProps) -> Element {
    rsx! {
        calendar::CalendarSelectMonth { class: "dx-calendar-month-select-container", attributes: props.attributes,
            calendar::CalendarSelectMonthValue { class: "dx-calendar-month-select-value",
                DropDownIcon {}
            }
            calendar::CalendarSelectMonthSelect { class: "dx-calendar-month-select" }
        }
    }
}

#[component]
pub fn CalendarSelectYear(props: CalendarSelectYearProps) -> Element {
    rsx! {
        calendar::CalendarSelectYear { class: "dx-calendar-year-select-container", attributes: props.attributes,
            calendar::CalendarSelectYearValue { class: "dx-calendar-year-select-value",
                DropDownIcon {}
            }
            calendar::CalendarSelectYearSelect { class: "dx-calendar-year-select" }
        }
    }
}

#[component]
pub fn CalendarGrid(props: CalendarGridProps) -> Element {
    rsx! {
        calendar::CalendarGrid {
            class: "dx-calendar-grid",
            id: props.id,
            attributes: props.attributes,
        }
    }
}

#[component]
pub fn CalendarMonthTitle(props: CalendarMonthTitleProps) -> Element {
    calendar::CalendarMonthTitle(props)
}

#[component]
pub fn CalendarDay(props: CalendarDayProps) -> Element {
    calendar::CalendarDay(props)
}

#[component]
fn DropDownIcon() -> Element {
    rsx! {
        Icon {
            width: "14px",
            height: "14px",
            stroke: "currentColor",
            path { d: "m6 9 6 6 6-6" }
        }
    }
}
