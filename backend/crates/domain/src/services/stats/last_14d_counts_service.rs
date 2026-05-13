use crate::{entities::tasks::task::Task, value_objects::stats::last_14d_counts::Last14dCounts};

pub struct Last14dCountsService {}

impl Last14dCountsService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn calculate(tasks: &[Task]) -> Last14dCounts {
        let mut counts = Last14dCounts::new();

        for task in tasks {
            if let Some(completed_at) = task.completed_at() {
                let day = completed_at.date_naive();
                let _ = counts.push(day, 1);
            }
        }

        counts
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
        Task::new(Uuid::new_v4(), "task".to_string(), TaskSchedule::Unscheduled, None)
    }

    #[test]
    fn test_empty() {
        let result = Last14dCountsService::calculate(&[]);
        assert_eq!(result.total(), 0);
        assert_eq!(result.counts().len(), 0);
    }

    #[test]
    fn test_task_completed_today() {
        let tasks = vec![completed_on(today())];
        let result = Last14dCountsService::calculate(&tasks);
        assert_eq!(result.total(), 1);
        assert_eq!(result.find_by_day(today()).unwrap().count(), 1);
    }

    #[test]
    fn test_multiple_tasks_same_day_accumulate() {
        let tasks = vec![completed_on(today()), completed_on(today()), completed_on(today())];
        let result = Last14dCountsService::calculate(&tasks);
        assert_eq!(result.counts().len(), 1);
        assert_eq!(result.find_by_day(today()).unwrap().count(), 3);
    }

    #[test]
    fn test_tasks_across_days() {
        let tasks = vec![
            completed_on(today()),
            completed_on(days_ago(1)),
            completed_on(days_ago(7)),
            completed_on(days_ago(13)),
        ];
        let result = Last14dCountsService::calculate(&tasks);
        assert_eq!(result.total(), 4);
        assert_eq!(result.counts().len(), 4);
    }

    #[test]
    fn test_task_older_than_14d_ignored() {
        let old = vec![completed_on(days_ago(14))];
        let result = Last14dCountsService::calculate(&old);
        assert_eq!(result.total(), 0);
    }

    #[test]
    fn test_pending_tasks_ignored() {
        let tasks = vec![pending(), pending()];
        let result = Last14dCountsService::calculate(&tasks);
        assert_eq!(result.total(), 0);
    }

    #[test]
    fn test_max() {
        let tasks = vec![
            completed_on(today()),
            completed_on(today()),
            completed_on(days_ago(1)),
        ];
        let result = Last14dCountsService::calculate(&tasks);
        assert_eq!(result.max(), 2);
    }
}
