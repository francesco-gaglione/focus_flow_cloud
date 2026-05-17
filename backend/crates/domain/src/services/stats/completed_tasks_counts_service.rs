use chrono::Utc;

use crate::{
    entities::{
        focus_session::{FocusSession, TerminatedSession},
        tasks::task::Task,
    },
    value_objects::stats::completed_tasks_counts::CompletedTasksCounts,
};

pub struct CompletedTasksCountsService {}

impl Default for CompletedTasksCountsService {
    fn default() -> Self {
        Self::new()
    }
}

impl CompletedTasksCountsService {
    pub fn new() -> Self {
        Self {}
    }

    /// Calculates the completed tasks counts for the given tasks.
    ///
    /// # Arguments
    ///
    /// * `tasks` - A slice of tasks to calculate the counts from (should contain at least 1 month of data)
    /// * `focus_sessions` - A slice of focus sessions to calculate the counts from
    ///
    /// # Returns
    ///
    /// A `CompletedTasksCounts` struct containing the calculated counts.
    pub fn calculate(
        tasks: &[Task],
        focus_sessions: &[FocusSession<TerminatedSession>],
    ) -> CompletedTasksCounts {
        let completed_today = tasks
            .iter()
            .filter(|t| {
                t.completed_at().is_some()
                    && t.completed_at().unwrap().date_naive() == Utc::now().date_naive()
            })
            .count() as i64;

        let completed_this_week = tasks
            .iter()
            .filter(|t| {
                t.completed_at().is_some()
                    && t.completed_at().unwrap().date_naive()
                        >= (Utc::now() - chrono::Duration::weeks(1)).date_naive()
            })
            .count() as i64;

        let completed_last_week = tasks
            .iter()
            .filter(|t| {
                t.completed_at().is_some()
                    && t.completed_at().unwrap().date_naive()
                        >= (Utc::now() - chrono::Duration::weeks(2)).date_naive()
                    && t.completed_at().unwrap().date_naive()
                        < (Utc::now() - chrono::Duration::weeks(1)).date_naive()
            })
            .count() as i64;

        let completed_this_month = tasks
            .iter()
            .filter(|t| {
                t.completed_at().is_some()
                    && t.completed_at().unwrap().date_naive()
                        >= (Utc::now() - chrono::Duration::days(30)).date_naive()
            })
            .count() as i64;

        let day_avg = completed_this_month as f64 / 30.0;

        let focus_sessions = focus_sessions.len() as i64;

        CompletedTasksCounts::new(
            completed_today,
            completed_this_week,
            completed_this_week - completed_last_week,
            completed_this_month,
            day_avg,
            focus_sessions,
        )
    }
}

#[cfg(test)]
mod test_completed_tasks_counts_service {
    use uuid::Uuid;

    use crate::entities::{
        focus_session::{FocusSession, TerminatedSession},
        tasks::task_schedule::TaskSchedule,
    };

    use super::*;

    #[test]
    fn test_calculate_empty() {
        let tasks: Vec<Task> = vec![];
        let sessions: Vec<FocusSession<TerminatedSession>> = vec![];
        let result = CompletedTasksCountsService::calculate(&tasks, &sessions);
        assert_eq!(result.completed_tasks(), 0);
        assert_eq!(result.week_completed_tasks(), 0);
        assert_eq!(result.current_week_delta(), 0);
        assert_eq!(result.month_completed_tasks(), 0);
        assert_eq!(result.day_avg(), 0.0);
        assert_eq!(result.focus_sessions(), 0);
    }

    #[test]
    fn test_calculate() {
        let user_id = Uuid::new_v4();
        let sessions: Vec<FocusSession<TerminatedSession>> = vec![];
        let mut tasks: Vec<Task> = vec![];
        for i in 0..10 {
            let mut task = Task::new(
                user_id,
                format!("Task {}", i),
                TaskSchedule::Unscheduled,
                None,
            );
            task.complete().unwrap();
            tasks.push(task);
        }

        for i in 0..10 {
            let task = Task::new(
                user_id,
                format!("Task {}", i),
                TaskSchedule::Unscheduled,
                None,
            );
            tasks.push(task);
        }

        let result = CompletedTasksCountsService::calculate(&tasks, &sessions);
        assert_eq!(result.completed_tasks(), 10);
        assert_eq!(result.week_completed_tasks(), 10);
        assert_eq!(result.current_week_delta(), 10);
        assert_eq!(result.month_completed_tasks(), 10);
        assert!((result.day_avg() - 10.0 / 30.0).abs() < f64::EPSILON);
        assert_eq!(result.focus_sessions(), 0);
    }
}
