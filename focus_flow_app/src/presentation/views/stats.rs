use dioxus::prelude::*;

// ── sample data ───────────────────────────────────────────────────────────────

const DONE_TODAY: u32 = 5;
const DONE_WEEK: u32 = 23;
const DONE_WEEK_DELTA: i32 = 4;

const DONE_30D: u32 = 47;
const AVG_PER_DAY: &str = "3.2";

const FOCUS_WEEK: u32 = 12;
const FOCUS_AVG_MINS: u32 = 24;
const FOCUS_BEST_DAY: &str = "Tue";
const FOCUS_BEST_COUNT: u32 = 5;

const PEAK_HOURS: [(&str, u32); 8] = [
    ("08", 8), ("10", 12), ("12", 4), ("14", 6),
    ("16", 2), ("18", 4), ("20", 1), ("22", 0),
];
const PEAK_MAX: u32 = 12;
const PEAK_LABEL: &str = "10:00 – 12:00";

const PRIORITY_DATA: [(&str, &str, u32, u32); 4] = [
    ("Urgent",  "#7c3aed", 8,  17),
    ("High",    "#ef4444", 15, 32),
    ("Medium",  "#d97706", 18, 38),
    ("Low / —", "#6b7280", 6,  13),
];

const OVERDUE_TREND: [u32; 5] = [12, 9, 7, 5, 4];
const OVERDUE_DELTA: i32 = -3;
const OVERDUE_CURRENT: u32 = 4;

const BAR_HEIGHTS: [u32; 14] = [40, 20, 70, 55, 30, 80, 60, 45, 90, 35, 65, 75, 50, 85];
const BAR_LABELS: [&str; 14] = [
    "14", "13", "12", "11", "10", "9", "8", "7", "6", "5", "4", "3", "2", "TDY",
];
const HEATMAP: [u8; 56] = [
    0, 1, 2, 1, 0, 1, 3, 1, 2, 3, 2, 1, 0, 2, 2, 3, 4, 3, 2, 1, 3, 1, 2, 3, 2, 1, 2, 2, 0, 1, 2, 3,
    1, 0, 1, 2, 3, 4, 3, 2, 1, 0, 1, 2, 3, 2, 3, 4, 2, 0, 1, 2, 1, 0, 0, 0,
];

// ── Stats ─────────────────────────────────────────────────────────────────────

#[component]
pub fn Stats() -> Element {
    let peak_data: Vec<(&str, u32, u32, bool)> = PEAK_HOURS.iter()
        .map(|(lbl, cnt)| {
            let pct = if PEAK_MAX > 0 { cnt * 100 / PEAK_MAX } else { 0 };
            (*lbl, *cnt, pct, *cnt == PEAK_MAX)
        })
        .collect();

    let ov_max = OVERDUE_TREND.iter().max().copied().unwrap_or(1);
    let overdue_bars: Vec<(u32, bool)> = OVERDUE_TREND.iter().enumerate()
        .map(|(i, &v)| (v * 32 / ov_max, i == OVERDUE_TREND.len() - 1))
        .collect();

    let (delta_sym, delta_cls, delta_txt) = if OVERDUE_DELTA <= 0 {
        ("↓", "overdue-delta good", format!("{} fewer than last week", OVERDUE_DELTA.unsigned_abs()))
    } else {
        ("↑", "overdue-delta bad", format!("{} more than last week", OVERDUE_DELTA))
    };
    let overdue_display = format!("{}{}", delta_sym, OVERDUE_CURRENT);

    let week_delta_str = if DONE_WEEK_DELTA >= 0 {
        format!("↑ +{} from last week", DONE_WEEK_DELTA)
    } else {
        format!("↓ {} from last week", DONE_WEEK_DELTA.unsigned_abs())
    };
    let week_badge_cls = if DONE_WEEK_DELTA >= 0 { "stats-hero-badge positive" } else { "stats-hero-badge negative" };

    rsx! {
        div { class: "scroll",

            // ── Hero (no hint — self-explanatory) ────────────────────────
            div { class: "stats-hero",
                div { class: "stats-card",
                    div { class: "stats-title", "Done today" }
                    div { class: "stats-big",
                        em { style: "color:var(--color-success,#46a758);", "{DONE_TODAY}" }
                        span { class: "unit", " tasks" }
                    }
                    div { class: "stats-hero-badge positive", "keep going ✦" }
                }
                div { class: "stats-card",
                    div { class: "stats-title", "This week" }
                    div { class: "stats-big", em { "{DONE_WEEK}" } span { class: "unit", " tasks" } }
                    div { class: "{week_badge_cls}", "{week_delta_str}" }
                }
            }

            // ── Trio ─────────────────────────────────────────────────────
            div { class: "stats-trio",
                HintCard {
                    title: "Done · 30d",
                    hint: "Total tasks completed in the last 30 days. Rolling window.",
                    div { class: "stats-big", em { "{DONE_30D}" } }
                }
                HintCard {
                    title: "Avg / day",
                    hint: "Average tasks completed per day over 30 days. Use as a baseline, not a target.",
                    div { class: "stats-big", "{AVG_PER_DAY}" }
                }
                HintCard {
                    title: "Focus · 7d",
                    hint: "Pomodoro sessions completed this week.",
                    div { class: "stats-big", em { "{FOCUS_WEEK}" } }
                }
            }

            // ── Peak window ───────────────────────────────────────────────
            HintCard {
                title: "Peak window",
                subtitle: "// when you flow",
                hint: "Hours when you complete the most tasks. Schedule demanding work here.",
                div { class: "peak-chart",
                    for (lbl, cnt, pct, is_peak) in peak_data {
                        div { class: "peak-row",
                            span { class: "peak-label", "{lbl}:00" }
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
                div { class: "peak-note", "⚡ Your peak: {PEAK_LABEL}" }
            }

            // ── Priority mix + Focus sessions ─────────────────────────────
            div { class: "stats-pair",
                HintCard {
                    title: "Priority mix",
                    hint: "Breakdown of completed tasks by priority. Useful to spot if you tend to complete easier tasks over more important ones.",
                    div { class: "breakdown",
                        for (lbl, color, cnt, pct) in PRIORITY_DATA {
                            PriorityRow { label: lbl, color, count: cnt, pct }
                        }
                    }
                }
                HintCard {
                    title: "Focus sessions",
                    hint: "Pomodoro session stats. Average duration reflects your actual focus window.",
                    div { class: "stats-big",
                        em { "{FOCUS_WEEK}" }
                        span { class: "unit", " this wk" }
                    }
                    div { class: "focus-details",
                        div { class: "focus-detail-row",
                            span { class: "focus-detail-label", "Avg" }
                            span { class: "focus-detail-val", "{FOCUS_AVG_MINS} min" }
                        }
                        div { class: "focus-detail-row",
                            span { class: "focus-detail-label", "Best" }
                            span { class: "focus-detail-val", "{FOCUS_BEST_DAY} · {FOCUS_BEST_COUNT}" }
                        }
                    }
                }
            }

            // ── Overdue trend ─────────────────────────────────────────────
            HintCard {
                title: "Overdue trend",
                subtitle: "// direction matters",
                hint: "Overdue task count week over week. Direction matters more than the absolute number.",
                div { class: "overdue-trend",
                    span { class: "{delta_cls}", "{overdue_display}" }
                    div { class: "overdue-info",
                        span { class: "overdue-label", "{delta_txt}" }
                        span { class: "overdue-sub", "{OVERDUE_CURRENT} overdue right now" }
                    }
                    div { class: "overdue-mini-bars",
                        for (h_px, is_last) in overdue_bars {
                            div {
                                class: if is_last { "overdue-mini-bar last" } else { "overdue-mini-bar" },
                                style: "height:{h_px}px;",
                            }
                        }
                    }
                }
            }

            // ── Category balance ──────────────────────────────────────────
            HintCard {
                title: "Category balance",
                subtitle: "// neglected?",
                hint: "Tasks completed per category. Low numbers in a category may signal it is getting less attention.",
                div { class: "breakdown",
                    BreakdownRow { label: "@work",     color: "#0070f3", pct: 42 }
                    BreakdownRow { label: "@personal", color: "#12a594", pct: 38 }
                    BreakdownRow { label: "@errand",   color: "#ffb224", pct: 20 }
                }
            }

            // ── 14-day rhythm ─────────────────────────────────────────────
            HintCard {
                title: "Last 14 days",
                subtitle: "// rhythm",
                hint: "Task completions over the last 14 days. Look at the pattern, not individual days.",
                div { class: "barchart",
                    for (i, h) in BAR_HEIGHTS.iter().enumerate() {
                        {
                            let is_today = i == 13;
                            let is_muted = *h == 0;
                            let bar_class = if is_muted { "bar muted" } else if is_today { "bar today" } else { "bar" };
                            let height_pct = format!("{}%", h);
                            rsx! {
                                div { class: "{bar_class}",
                                    span { class: "col", style: "height: {height_pct}" }
                                }
                            }
                        }
                    }
                }
                div { class: "barchart-labels",
                    for (i, lbl) in BAR_LABELS.iter().enumerate() {
                        {
                            let cls = if i == 13 { "today" } else { "" };
                            rsx! { span { class: "{cls}", "{lbl}" } }
                        }
                    }
                }
            }

            // ── 8-week heatmap ────────────────────────────────────────────
            HintCard {
                title: "Last 8 weeks",
                subtitle: "// no shame",
                hint: "8 weeks of activity at a glance. A reference for patterns, not a streak tracker.",
                div { class: "heatmap",
                    for level in HEATMAP.iter() {
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

// ── HintCard ──────────────────────────────────────────────────────────────────

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

// ── sub-components ────────────────────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
struct PriorityRowProps {
    label: &'static str,
    color: &'static str,
    count: u32,
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
    label: &'static str,
    color: &'static str,
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
            span { class: "breakdown-pct", "{props.pct}%" }
        }
    }
}
