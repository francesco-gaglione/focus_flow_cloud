use crate::entities::focus_session::FocusSession;
use crate::entities::stats::category_distribution::{
    CategoryDistributionItem, TaskDistributionItem,
};
use std::collections::HashMap;
use uuid::Uuid;

pub struct CategoryAnalysisCalculator;

impl CategoryAnalysisCalculator {
    pub fn calculate(
        sessions: &[FocusSession],
        category_names: &HashMap<Uuid, String>,
        task_details: &HashMap<Uuid, String>,
    ) -> Vec<CategoryDistributionItem> {
        let mut category_times: HashMap<Uuid, i64> = HashMap::new();
        let mut total_time: i64 = 0;

        for session in sessions {
            if let (Some(category_id), Some(duration)) =
                (session.category_id(), session.actual_duration())
            {
                *category_times.entry(category_id).or_insert(0) += duration;
                total_time += duration;
            }
        }

        let mut distribution: Vec<CategoryDistributionItem> = category_times
            .into_iter()
            .filter_map(|(category_id, total_focus_time)| {
                category_names.get(&category_id).and_then(|name| {
                    let percentage = if total_time > 0 {
                        (total_focus_time as f32 / total_time as f32) * 100.0
                    } else {
                        0.0
                    };

                    let category_sessions: Vec<FocusSession> = sessions
                        .iter()
                        .filter(|s| s.category_id() == Some(category_id))
                        .cloned()
                        .collect();

                    let task_distribution = Self::calculate_task_distribution(
                        &category_sessions,
                        task_details,
                        total_time,
                    );

                    CategoryDistributionItem::new(
                        name.to_string(),
                        category_id,
                        total_focus_time,
                        percentage,
                        task_distribution,
                    )
                    .ok()
                })
            })
            .collect();

        distribution.sort_by_key(|b| std::cmp::Reverse(b.total_focus_time()));

        distribution
    }

    fn calculate_task_distribution(
        sessions: &[FocusSession],
        task_details: &HashMap<Uuid, String>,
        total_time: i64,
    ) -> Vec<TaskDistributionItem> {
        let mut task_times: HashMap<Uuid, i64> = HashMap::new();

        for session in sessions {
            if let (Some(task_id), Some(duration)) = (session.task_id(), session.actual_duration())
            {
                *task_times.entry(task_id).or_insert(0) += duration;
            }
        }

        let mut distribution: Vec<TaskDistributionItem> = task_times
            .into_iter()
            .filter_map(|(task_id, total_focus_time)| {
                task_details.get(&task_id).and_then(|task_name| {
                    let percentage = if total_time > 0 {
                        (total_focus_time as f32 / total_time as f32) * 100.0
                    } else {
                        0.0
                    };

                    TaskDistributionItem::new(task_name.clone(), total_focus_time, percentage).ok()
                })
            })
            .collect();

        distribution.sort_by_key(|b| std::cmp::Reverse(b.total_focus_time()));

        distribution
    }
}
