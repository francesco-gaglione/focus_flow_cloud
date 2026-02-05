use crate::entities::stats::calculators::concentration_calculator::ConcentrationStats;
use crate::entities::stats::calculators::daily_activity_calculator::DailyActivityItem;
use crate::entities::stats::calculators::period_summary_calculator::PeriodSummary;
use crate::entities::stats::category_distribution::CategoryDistributionItem;
use crate::entities::stats::concentration_period::ConcentrationPeriod;

#[derive(Debug, Clone)]
pub struct Stats {
    total_sessions: usize,
    total_breaks: usize,
    total_focus_time: i64,
    total_break_time: i64,
    most_concentrated_period: ConcentrationPeriod,
    less_concentrated_period: ConcentrationPeriod,
    concentration_distribution: [u32; 5],
    category_distribution: Vec<CategoryDistributionItem>,
    daily_activity: Vec<DailyActivityItem>,
}

impl Stats {
    pub fn new(
        period_summary: PeriodSummary,
        concentration_stats: ConcentrationStats,
        category_distribution: Vec<CategoryDistributionItem>,
        daily_activity: Vec<DailyActivityItem>,
    ) -> Self {
        Self {
            total_sessions: period_summary.total_sessions,
            total_breaks: period_summary.total_breaks,
            total_focus_time: period_summary.total_focus_time,
            total_break_time: period_summary.total_break_time,
            most_concentrated_period: concentration_stats.most_concentrated_period,
            less_concentrated_period: concentration_stats.less_concentrated_period,
            concentration_distribution: concentration_stats.concentration_distribution,
            category_distribution,
            daily_activity,
        }
    }

    pub fn total_sessions(&self) -> usize {
        self.total_sessions
    }

    pub fn total_breaks(&self) -> usize {
        self.total_breaks
    }

    pub fn total_focus_time(&self) -> i64 {
        self.total_focus_time
    }

    pub fn total_break_time(&self) -> i64 {
        self.total_break_time
    }

    pub fn most_concentrated_period(&self) -> &ConcentrationPeriod {
        &self.most_concentrated_period
    }

    pub fn less_concentrated_period(&self) -> &ConcentrationPeriod {
        &self.less_concentrated_period
    }

    pub fn concentration_distribution(&self) -> &[u32; 5] {
        &self.concentration_distribution
    }

    pub fn category_distribution(&self) -> &[CategoryDistributionItem] {
        &self.category_distribution
    }

    pub fn daily_activity(&self) -> &[DailyActivityItem] {
        &self.daily_activity
    }
}
