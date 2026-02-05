use crate::entities::focus_session::FocusSession;
use chrono::NaiveDate;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DailyActivityItem {
    pub date: NaiveDate,
    pub category_distribution: Vec<DailyActivityDistributionItem>,
}

impl DailyActivityItem {
    pub fn new(date: NaiveDate, category_distribution: Vec<DailyActivityDistributionItem>) -> Self {
        Self {
            date,
            category_distribution,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DailyActivityDistributionItem {
    pub category_name: String,
    pub category_id: Uuid,
    pub total_focus_time: i64,
}

impl DailyActivityDistributionItem {
    pub fn new(category_name: String, category_id: Uuid, total_focus_time: i64) -> Self {
        Self {
            category_name,
            category_id,
            total_focus_time,
        }
    }
}

pub struct DailyActivityCalculator;

impl DailyActivityCalculator {
    pub fn calculate(
        sessions: &[FocusSession],
        category_names: &HashMap<Uuid, String>,
    ) -> Vec<DailyActivityItem> {
        let mut daily_data: HashMap<NaiveDate, HashMap<Uuid, i64>> = HashMap::new();

        for session in sessions {
            if let (Some(category_id), Some(duration)) =
                (session.category_id(), session.actual_duration())
            {
                let date = session.started_at().date_naive();
                daily_data
                    .entry(date)
                    .or_default()
                    .entry(category_id)
                    .and_modify(|time| *time += duration)
                    .or_insert(duration);
            }
        }

        let mut daily_activity: Vec<DailyActivityItem> = daily_data
            .into_iter()
            .map(|(date, categories)| {
                let mut category_distribution: Vec<DailyActivityDistributionItem> = categories
                    .into_iter()
                    .filter_map(|(category_id, total_focus_time)| {
                        category_names.get(&category_id).map(|name| {
                            DailyActivityDistributionItem::new(
                                name.clone(),
                                category_id,
                                total_focus_time,
                            )
                        })
                    })
                    .collect();

                category_distribution.sort_by_key(|b| std::cmp::Reverse(b.total_focus_time));

                DailyActivityItem::new(date, category_distribution)
            })
            .collect::<Vec<DailyActivityItem>>();

        daily_activity.sort_by_key(|a| a.date);

        daily_activity
    }
}
