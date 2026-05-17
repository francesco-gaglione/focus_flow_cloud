use chrono::{Days, Utc};

use crate::{
    entities::tasks::{task::Task, task_schedule::TaskSchedule},
    value_objects::stats::overdue_trend::{OverdueTrend, OverdueTrendType},
};

pub struct OverdueTrendService {}

impl Default for OverdueTrendService {
    fn default() -> Self {
        Self::new()
    }
}

impl OverdueTrendService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn calculate(tasks: &[Task]) -> OverdueTrend {
        let today = Utc::now().date_naive();
        let week_ago = today.checked_sub_days(Days::new(7)).unwrap_or(today);
        let two_weeks_ago = today.checked_sub_days(Days::new(14)).unwrap_or(today);

        let this_week_overdue = tasks
            .iter()
            .filter(|t| {
                t.completed_at().is_none()
                    && schedule_date(t).is_some_and(|d| d >= week_ago && d <= today)
            })
            .count() as i64;

        let last_week_overdue = tasks
            .iter()
            .filter(|t| {
                t.completed_at().is_none()
                    && schedule_date(t).is_some_and(|d| d >= two_weeks_ago && d < week_ago)
            })
            .count() as i64;

        let trend_value = if last_week_overdue == 0 {
            this_week_overdue as f64
        } else {
            (this_week_overdue - last_week_overdue) as f64 / last_week_overdue as f64
        };

        let trend_type = if this_week_overdue > last_week_overdue {
            OverdueTrendType::Increasing
        } else if this_week_overdue < last_week_overdue {
            OverdueTrendType::Decreasing
        } else {
            OverdueTrendType::Stable
        };

        OverdueTrend {
            trend_type,
            trend_value,
        }
    }
}

fn schedule_date(task: &Task) -> Option<chrono::NaiveDate> {
    match task.schedule() {
        TaskSchedule::AllDay { date } => Some(date),
        TaskSchedule::At { starts_at } => Some(starts_at.date_naive()),
        TaskSchedule::Span { starts_at, .. } => Some(starts_at.date_naive()),
        TaskSchedule::Unscheduled => None,
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Days, Duration, NaiveDate, Utc};
    use uuid::Uuid;

    use crate::entities::tasks::{task::Task, task_schedule::TaskSchedule};

    use super::*;

    fn today() -> NaiveDate {
        Utc::now().date_naive()
    }

    fn days_ago(n: u64) -> NaiveDate {
        today().checked_sub_days(Days::new(n)).unwrap()
    }

    fn overdue_task_scheduled_on(date: NaiveDate) -> Task {
        let starts_at = date.and_hms_opt(9, 0, 0).unwrap().and_utc();
        Task::reconstitute(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "task".to_string(),
            None,
            None,
            None,
            vec![],
            TaskSchedule::At { starts_at },
            None,
            vec![],
        )
    }

    fn completed_task_scheduled_on(date: NaiveDate) -> Task {
        let starts_at = date.and_hms_opt(9, 0, 0).unwrap().and_utc();
        let completed_at = Utc::now() - Duration::hours(1);
        Task::reconstitute(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "task".to_string(),
            None,
            None,
            None,
            vec![],
            TaskSchedule::At { starts_at },
            Some(completed_at),
            vec![],
        )
    }

    #[test]
    fn test_empty_is_stable() {
        let result = OverdueTrendService::calculate(&[]);
        assert!(matches!(result.trend_type, OverdueTrendType::Stable));
        assert_eq!(result.trend_value, 0.0);
    }

    #[test]
    fn test_increasing_trend() {
        let tasks = vec![
            overdue_task_scheduled_on(days_ago(1)),
            overdue_task_scheduled_on(days_ago(2)),
            overdue_task_scheduled_on(days_ago(8)),
        ];
        let result = OverdueTrendService::calculate(&tasks);
        assert!(matches!(result.trend_type, OverdueTrendType::Increasing));
    }

    #[test]
    fn test_decreasing_trend() {
        let tasks = vec![
            overdue_task_scheduled_on(days_ago(1)),
            overdue_task_scheduled_on(days_ago(8)),
            overdue_task_scheduled_on(days_ago(9)),
            overdue_task_scheduled_on(days_ago(10)),
        ];
        let result = OverdueTrendService::calculate(&tasks);
        assert!(matches!(result.trend_type, OverdueTrendType::Decreasing));
    }

    #[test]
    fn test_completed_tasks_not_counted() {
        let tasks = vec![
            completed_task_scheduled_on(days_ago(1)),
            completed_task_scheduled_on(days_ago(2)),
        ];
        let result = OverdueTrendService::calculate(&tasks);
        assert!(matches!(result.trend_type, OverdueTrendType::Stable));
    }

    #[test]
    fn test_unscheduled_not_counted() {
        let task = Task::new(
            Uuid::new_v4(),
            "task".to_string(),
            TaskSchedule::Unscheduled,
            None,
        );
        let result = OverdueTrendService::calculate(&[task]);
        assert!(matches!(result.trend_type, OverdueTrendType::Stable));
    }
}
