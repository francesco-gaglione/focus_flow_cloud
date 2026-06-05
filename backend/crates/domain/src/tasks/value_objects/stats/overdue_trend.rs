pub enum OverdueTrendType {
    Increasing,
    Decreasing,
    Stable,
}

pub struct OverdueTrend {
    pub trend_type: OverdueTrendType,
    pub trend_value: f64,
}
