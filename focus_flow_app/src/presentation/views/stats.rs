use dioxus::prelude::*;

const BAR_HEIGHTS: [u32; 14] = [40, 20, 70, 55, 30, 80, 60, 45, 90, 35, 65, 75, 50, 85];
const BAR_LABELS: [&str; 14] = [
    "14", "13", "12", "11", "10", "9", "8", "7", "6", "5", "4", "3", "2", "TDY",
];
const HEATMAP: [u8; 56] = [
    0, 1, 2, 1, 0, 1, 3, 1, 2, 3, 2, 1, 0, 2, 2, 3, 4, 3, 2, 1, 3, 1, 2, 3, 2, 1, 2, 2, 0, 1, 2, 3,
    1, 0, 1, 2, 3, 4, 3, 2, 1, 0, 1, 2, 3, 2, 3, 4, 2, 0, 1, 2, 1, 0, 0, 0,
];

#[component]
pub fn Stats() -> Element {
    rsx! {
        div { class: "scroll",

           p {
              "Mock page, work in progress"
           } ,

            // Trio
            div { class: "stats-trio",
                div { class: "stats-card",
                    div { class: "stats-title", "Streak" }
                    div { class: "stats-big", em { "12" } span { class: "unit", "d" } }
                }
                div { class: "stats-card",
                    div { class: "stats-title", "Done · 30d" }
                    div { class: "stats-big", em { "47" } }
                }
                div { class: "stats-card",
                    div { class: "stats-title", "Avg/day" }
                    div { class: "stats-big", "3", span { class: "unit", ".2" } }
                }
            }

            // Bar chart
            div { class: "stats-card",
                div { class: "stats-title",
                    "Last 14 days "
                    span { class: "delta", "// rhythm" }
                }
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

            // By category
            div { class: "stats-card",
                div { class: "stats-title", "By category" }
                div { class: "breakdown",
                    BreakdownRow { label: "@work", color: "var(--cat-magenta)", pct: 42 }
                    BreakdownRow { label: "@personal", color: "var(--cat-cyan)", pct: 38 }
                    BreakdownRow { label: "@errand", color: "var(--cat-amber)", pct: 20 }
                }
            }

            // Heatmap
            div { class: "stats-card",
                div { class: "stats-title",
                    "Last 8 weeks "
                    span { class: "delta", "// no shame" }
                }
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
            span { class: "breakdown-lbl", style: "color: {props.color}", "{props.label}" }
            div { class: "breakdown-track",
                div { class: "breakdown-fill", style: "--c: {props.color}; width: {width}" }
            }
            span { class: "breakdown-pct", "{props.pct}%" }
        }
    }
}
