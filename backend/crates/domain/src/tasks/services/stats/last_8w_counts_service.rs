use chrono::{Datelike, Days, Utc};

use crate::tasks::entities::task::Task;
use crate::tasks::value_objects::stats::last_8w_counts::Last8wCounts;

pub struct Last8wCountsService {}

impl Default for Last8wCountsService {
    fn default() -> Self {
        Self::new()
    }
}

impl Last8wCountsService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn calculate(tasks: &[Task]) -> Last8wCounts {
        let mut counts = Last8wCounts::new();
        let this_week = week_start_of(Utc::now().date_naive());

        for i in 0..8u64 {
            let week = this_week.checked_sub_days(Days::new(7 * (7 - i))).unwrap();
            let _ = counts.push(week, 0);
        }

        for task in tasks {
            if let Some(completed_at) = task.completed_at() {
                let day = completed_at.date_naive();
                let week_start = week_start_of(day);
                let _ = counts.push(week_start, 1);
            }
        }

        counts
    }
}

fn week_start_of(date: chrono::NaiveDate) -> chrono::NaiveDate {
    let days_from_monday = date.weekday().num_days_from_monday();
    date.checked_sub_days(Days::new(days_from_monday as u64))
        .unwrap_or(date)
}

#[cfg(test)]
mod tests {
    use chrono::{Days, NaiveDate, Utc};
    use uuid::Uuid;

    use crate::tasks::entities::{task::Task, task_schedule::TaskSchedule};

    use super::*;

    fn this_week_start() -> NaiveDate {
        week_start_of(Utc::now().date_naive())
    }

    fn weeks_ago_start(n: u64) -> NaiveDate {
        this_week_start()
            .checked_sub_days(Days::new(7 * n))
            .unwrap()
    }

    fn completed_on(date: NaiveDate) -> Task {
        let completed_at = date.and_hms_opt(12, 0, 0).unwrap().and_utc();
        Task::reconstitute(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "task".to_string(),
            None,
            None,
            None,
            vec![],
            TaskSchedule::Unscheduled,
            Some(completed_at),
            vec![],
        )
    }

    fn pending() -> Task {
        Task::new(
            Uuid::new_v4(),
            "task".to_string(),
            TaskSchedule::Unscheduled,
            None,
        )
    }

    #[test]
    fn test_empty() {
        let result = Last8wCountsService::calculate(&[]);
        assert_eq!(result.total(), 0);
        assert_eq!(result.counts().len(), 8);
    }

    #[test]
    fn test_task_this_week() {
        let tasks = vec![completed_on(this_week_start())];
        let result = Last8wCountsService::calculate(&tasks);
        assert_eq!(result.total(), 1);
        assert_eq!(result.find_by_week(this_week_start()).unwrap().count(), 1);
    }

    #[test]
    fn test_multiple_tasks_same_week_accumulate() {
        let tasks = vec![
            completed_on(this_week_start()),
            completed_on(this_week_start()),
        ];
        let result = Last8wCountsService::calculate(&tasks);
        assert_eq!(result.counts().len(), 8);
        assert_eq!(result.find_by_week(this_week_start()).unwrap().count(), 2);
    }

    #[test]
    fn test_tasks_across_weeks() {
        let tasks = vec![
            completed_on(this_week_start()),
            completed_on(weeks_ago_start(1)),
            completed_on(weeks_ago_start(4)),
            completed_on(weeks_ago_start(7)),
        ];
        let result = Last8wCountsService::calculate(&tasks);
        assert_eq!(result.total(), 4);
        assert_eq!(result.counts().len(), 8);
    }

    #[test]
    fn test_task_older_than_8w_ignored() {
        let old = vec![completed_on(weeks_ago_start(8))];
        let result = Last8wCountsService::calculate(&old);
        assert_eq!(result.total(), 0);
    }

    #[test]
    fn test_pending_tasks_ignored() {
        let tasks = vec![pending(), pending()];
        let result = Last8wCountsService::calculate(&tasks);
        assert_eq!(result.total(), 0);
    }

    #[test]
    fn test_max() {
        let tasks = vec![
            completed_on(this_week_start()),
            completed_on(this_week_start()),
            completed_on(weeks_ago_start(1)),
        ];
        let result = Last8wCountsService::calculate(&tasks);
        assert_eq!(result.max(), 2);
    }
}
