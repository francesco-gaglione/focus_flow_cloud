use chrono::Datelike;
use dioxus::prelude::*;
use shared::stats::OverdueTrendTypeDto;

use crate::use_cases::stats::get_stats_uc::{get_stats_uc, StatsData};

const CAT_COLORS: [&str; 7] = [
    "#0070f3", "#12a594", "#ffb224", "#7c3aed", "#ef4444", "#d97706", "#6b7280",
];

#[component]
pub fn Stats() -> Element {
    let mut stats_data = use_signal(StatsData::default);
    let mut is_loading = use_signal(|| true);
    let mut load_error: Signal<Option<String>> = use_signal(|| None);

    let _resource = use_resource(move || async move {
        match get_stats_uc().await {
            Ok(res) => {
                is_loading.set(false);
                stats_data.set(res);
            }
            Err(e) => {
                load_error.set(Some(e.to_string()));
                is_loading.set(false);
            }
        }
    });

    let stats = stats_data.read().stats.clone();
    let category_colors = stats_data.read().category_colors.clone();

    let counts = stats.completed_tasks_counts.clone();
    let peak_window = stats.peak_window.clone();
    let priority = stats.completed_by_priority.clone();
    let focus = stats.completed_focus_sessions.clone();
    let overdue_dto = stats.overdue_trend.clone();
    let cats = stats.count_by_category.clone();
    let days14 = stats.last_14d.clone();
    let weeks8 = stats.last_8w.clone();

    let peak_max = peak_window.iter().map(|p| p.count).max().unwrap_or(0);
    let peak_data: Vec<(String, usize, u32, bool)> = peak_window
        .iter()
        .map(|p| {
            let lbl = p.start.get(..5).unwrap_or(&p.start).to_string();
            let pct = (p.count * 100).checked_div(peak_max).unwrap_or(0) as u32;
            (lbl, p.count, pct, p.count == peak_max && peak_max > 0)
        })
        .collect();

    let priority_total = (priority.low + priority.medium + priority.high + priority.urgent).max(1);
    let p_urgent_pct = (priority.urgent * 100 / priority_total) as u32;
    let p_high_pct = (priority.high * 100 / priority_total) as u32;
    let p_medium_pct = (priority.medium * 100 / priority_total) as u32;
    let p_low_pct = (priority.low * 100 / priority_total) as u32;

    let focus_count = focus.count;
    let focus_avg_mins = if focus.avg_duration_secs > 0 {
        focus.avg_duration_secs / 60
    } else {
        0
    };

    let trend_val = overdue_dto.trend_value.abs();
    let (delta_cls, delta_txt, overdue_display) = match overdue_dto.trend_type {
        OverdueTrendTypeDto::Decreasing => (
            "overdue-delta good",
            format!("{:.0}% fewer overdue vs last week", trend_val),
            format!("↓{:.0}%", trend_val),
        ),
        OverdueTrendTypeDto::Increasing => (
            "overdue-delta bad",
            format!("{:.0}% more overdue vs last week", trend_val),
            format!("↑{:.0}%", trend_val),
        ),
        OverdueTrendTypeDto::Stable => (
            "overdue-delta",
            "No change from last week".to_string(),
            "→ 0%".to_string(),
        ),
    };

    let cat_total: u64 = cats.iter().map(|c| c.count).sum::<u64>().max(1);

    let max14 = days14.iter().map(|d| d.count).max().unwrap_or(0).max(1);
    let bar14: Vec<(u32, bool, bool, String)> = days14
        .iter()
        .enumerate()
        .map(|(i, d)| {
            let h = (d.count * 100 / max14) as u32;
            let is_today = i == days14.len().saturating_sub(1);
            let is_muted = d.count == 0;
            let lbl = if is_today {
                "TDY".to_string()
            } else {
                d.day.day().to_string()
            };
            (h, is_today, is_muted, lbl)
        })
        .collect();

    let max_week = weeks8.iter().map(|w| w.count).max().unwrap_or(0);
    let heatmap_levels: Vec<u8> = weeks8
        .iter()
        .flat_map(|w| {
            let level = ((w.count * 4).checked_div(max_week).unwrap_or(0).min(4)) as u8;
            vec![level; 7]
        })
        .collect();

    let week_delta = counts.week_delta;
    let week_delta_str = if week_delta >= 0 {
        format!("↑ +{} from last week", week_delta)
    } else {
        format!("↓ {} from last week", week_delta.abs())
    };
    let week_badge_cls = if week_delta >= 0 {
        "font-mono text-[10px] tracking-[var(--tracking-data)] uppercase mt-1 inline-block px-[7px] py-[2px] rounded-full text-success bg-[color-mix(in_srgb,#46a758_12%,transparent)]"
    } else {
        "font-mono text-[10px] tracking-[var(--tracking-data)] uppercase mt-1 inline-block px-[7px] py-[2px] rounded-full text-[#ef4444] bg-[color-mix(in_srgb,#ef4444_12%,transparent)]"
    };
    let day_avg = format!("{:.1}", counts.day_avg);

    rsx! {
        div { class: "scroll",

            if *is_loading.read() {
                div { class: "font-mono text-xs text-subtle text-center py-8 tracking-[var(--tracking-data)] uppercase", "Loading…" }
            } else if let Some(err) = load_error.read().as_ref() {
                div { class: "font-mono text-xs text-[#ef4444] text-center py-8", "{err}" }
            } else {

                div { class: "stats-hero grid grid-cols-2 gap-2 mb-3",
                    div { class: "stats-card",
                        div { class: "stats-title", "Done today" }
                        div { class: "stats-big",
                            em { style: "color:var(--color-success,#46a758);", "{counts.completed_today}" }
                            span { class: "unit", " tasks" }
                        }
                        div { class: "font-mono text-[10px] tracking-[var(--tracking-data)] uppercase mt-1 inline-block px-[7px] py-[2px] rounded-full text-success bg-[color-mix(in_srgb,#46a758_12%,transparent)]", "keep going ✦" }
                    }
                    div { class: "stats-card",
                        div { class: "stats-title", "This week" }
                        div { class: "stats-big",
                            em { "{counts.completed_this_week}" }
                            span { class: "unit", " tasks" }
                        }
                        div { class: "{week_badge_cls}", "{week_delta_str}" }
                    }
                }

                div { class: "stats-trio grid grid-cols-3 gap-2 mb-3",
                    HintCard {
                        title: "Done · 30d",
                        hint: "Total tasks completed in the last 30 days. Rolling window.",
                        div { class: "stats-big", em { "{counts.completed_this_month}" } }
                    }
                    HintCard {
                        title: "Avg / day",
                        hint: "Average tasks completed per day over 30 days. Use as a baseline, not a target.",
                        div { class: "stats-big", "{day_avg}" }
                    }
                    HintCard {
                        title: "Focus · 7d",
                        hint: "Pomodoro sessions completed this week.",
                        div { class: "stats-big", em { "{counts.focus_sessions}" } }
                    }
                }

                HintCard {
                    title: "Peak window",
                    subtitle: "// when you flow",
                    hint: "Hours when you complete the most tasks. Schedule demanding work here.",
                    if peak_data.is_empty() {
                        div { class: "font-mono text-xs text-subtle py-2", "No data yet" }
                    } else {
                        div { class: "peak-chart",
                            for (lbl, cnt, pct, is_peak) in peak_data {
                                div { class: "peak-row",
                                    span { class: "peak-label", "{lbl}" }
                                    div { class: "peak-track",
                                        div {
                                            class: if is_peak { "peak-fill peak-top" } else { "peak-fill" },
                                            style: "width:{pct}%;",
                                        }
                                    }
                                    span { class: "peak-count", "{cnt}" }
                                }
                            }
                        }
                    }
                }

                div { class: "stats-pair grid grid-cols-2 gap-2 mb-3",
                    HintCard {
                        title: "Priority mix",
                        hint: "Breakdown of completed tasks by priority. Useful to spot if you tend to complete easier tasks over more important ones.",
                        div { class: "breakdown",
                            PriorityRow { label: "Urgent", color: "#7c3aed", count: priority.urgent, pct: p_urgent_pct }
                            PriorityRow { label: "High",   color: "#ef4444", count: priority.high,   pct: p_high_pct   }
                            PriorityRow { label: "Medium", color: "#d97706", count: priority.medium,  pct: p_medium_pct }
                            PriorityRow { label: "Low / —",color: "#6b7280", count: priority.low,    pct: p_low_pct    }
                        }
                    }
                    HintCard {
                        title: "Focus sessions",
                        hint: "Pomodoro session stats. Average duration reflects your actual focus window.",
                        div { class: "stats-big",
                            em { "{focus_count}" }
                            span { class: "unit", " this wk" }
                        }
                        div { class: "focus-details",
                            div { class: "focus-detail-row",
                                span { class: "focus-detail-label", "Avg" }
                                span { class: "focus-detail-val", "{focus_avg_mins} min" }
                            }
                        }
                    }
                }

                HintCard {
                    title: "Overdue trend",
                    subtitle: "// direction matters",
                    hint: "Overdue task count week over week. Direction matters more than the absolute number.",
                    div { class: "overdue-trend",
                        span { class: "{delta_cls}", "{overdue_display}" }
                        div { class: "overdue-info",
                            span { class: "overdue-label", "{delta_txt}" }
                        }
                    }
                }

                HintCard {
                    title: "Category balance",
                    subtitle: "// neglected?",
                    hint: "Tasks completed per category. Low numbers in a category may signal it is getting less attention.",
                    if cats.is_empty() {
                        div { class: "font-mono text-xs text-subtle py-2", "No data yet" }
                    } else {
                        div { class: "breakdown",
                            for (i, cat) in cats.iter().enumerate() {
                                {
                                    let color = category_colors
                                        .get(&cat.category_id.to_string())
                                        .cloned()
                                        .unwrap_or_else(|| CAT_COLORS[i % CAT_COLORS.len()].to_string());
                                    let pct = (cat.count * 100 / cat_total) as u32;
                                    rsx! {
                                        BreakdownRow {
                                            label: cat.category_name.clone(),
                                            color,
                                            count: cat.count,
                                            pct,
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                HintCard {
                    title: "Last 14 days",
                    subtitle: "// rhythm",
                    hint: "Task completions over the last 14 days. Look at the pattern, not individual days.",
                    if bar14.is_empty() {
                        div { class: "font-mono text-xs text-subtle py-2", "No data yet" }
                    } else {
                        div { class: "barchart",
                            for (h, is_today, is_muted, _lbl) in bar14.iter() {
                                {
                                    let bar_cls = if *is_muted { "bar muted" } else if *is_today { "bar today" } else { "bar" };
                                    let height = format!("{}%", h);
                                    rsx! {
                                        div { class: "{bar_cls}",
                                            span { class: "col", style: "height:{height};" }
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "barchart-labels",
                            for (_h, is_today, _is_muted, lbl) in bar14.iter() {
                                {
                                    let cls = if *is_today { "today" } else { "" };
                                    rsx! { span { class: "{cls}", "{lbl}" } }
                                }
                            }
                        }
                    }
                }

                HintCard {
                    title: "Last 8 weeks",
                    subtitle: "// no shame",
                    hint: "8 weeks of activity at a glance. A reference for patterns, not a streak tracker.",
                    if heatmap_levels.is_empty() {
                        div { class: "font-mono text-xs text-subtle py-2", "No data yet" }
                    } else {
                        div { class: "heatmap",
                            for level in heatmap_levels.iter() {
                                {
                                    let cls = match level {
                                        1 => "cell hl-1",
                                        2 => "cell hl-2",
                                        3 => "cell hl-3",
                                        4 => "cell hl-4",
                                        _ => "cell",
                                    };
                                    rsx! { div { class: "{cls}" } }
                                }
                            }
                        }
                        div { class: "heatmap-legend",
                            span { "less" }
                            div { class: "scale",
                                span {}
                                span { class: "hl-1" }
                                span { class: "hl-2" }
                                span { class: "hl-3" }
                                span { class: "hl-4" }
                            }
                            span { "more" }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct HintCardProps {
    title: &'static str,
    hint: &'static str,
    #[props(optional)]
    subtitle: Option<&'static str>,
    children: Element,
}

#[component]
fn HintCard(props: HintCardProps) -> Element {
    let mut open = use_signal(|| false);
    let is_open = *open.read();
    rsx! {
        div { class: "stats-card",
            div { class: "stats-title",
                span { class: "stats-title-left",
                    "{props.title}"
                    if let Some(sub) = props.subtitle {
                        span { class: "delta", " {sub}" }
                    }
                }
                button {
                    class: if is_open { "stats-hint-btn active" } else { "stats-hint-btn" },
                    r#type: "button",
                    title: "What does this mean?",
                    onclick: move |e| {
                        e.stop_propagation();
                        let cur = *open.read();
                        open.set(!cur);
                    },
                    "?"
                }
            }
            if is_open {
                div { class: "stats-hint-text", "{props.hint}" }
            }
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct PriorityRowProps {
    label: &'static str,
    color: &'static str,
    count: usize,
    pct: u32,
}

#[component]
fn PriorityRow(props: PriorityRowProps) -> Element {
    let width = format!("{}%", props.pct);
    rsx! {
        div { class: "breakdown-row",
            span { class: "breakdown-lbl", style: "color:{props.color};", "{props.label}" }
            div { class: "breakdown-track",
                div { class: "breakdown-fill", style: "--c:{props.color};width:{width};" }
            }
            span { class: "breakdown-pct", "{props.count}" }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct BreakdownRowProps {
    label: String,
    color: String,
    count: u64,
    pct: u32,
}

#[component]
fn BreakdownRow(props: BreakdownRowProps) -> Element {
    let width = format!("{}%", props.pct);
    rsx! {
        div { class: "breakdown-row",
            span { class: "breakdown-lbl", style: "color:{props.color};", "{props.label}" }
            div { class: "breakdown-track",
                div { class: "breakdown-fill", style: "--c:{props.color};width:{width};" }
            }
            span { class: "breakdown-pct", "{props.count}" }
        }
    }
}
