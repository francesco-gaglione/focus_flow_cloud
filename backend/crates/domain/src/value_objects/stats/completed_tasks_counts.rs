pub struct CompletedTasksCounts {
    completed_tasks_today: i64,
    week_completed_tasks: i64,
    current_week_delta: i64,
    month_completed_tasks: i64,
    day_avg: f64,
    focus_sessions: i64,
}

impl CompletedTasksCounts {
    pub fn new(
        completed_tasks_today: i64,
        week_completed_tasks: i64,
        current_week_delta: i64,
        month_completed_tasks: i64,
        day_avg: f64,
        focus_sessions: i64,
    ) -> Self {
        Self {
            completed_tasks_today,
            week_completed_tasks,
            current_week_delta,
            month_completed_tasks,
            day_avg,
            focus_sessions,
        }
    }

    pub fn completed_tasks(&self) -> i64 {
        self.completed_tasks_today
    }

    pub fn week_completed_tasks(&self) -> i64 {
        self.week_completed_tasks
    }

    pub fn current_week_delta(&self) -> i64 {
        self.current_week_delta
    }

    pub fn month_completed_tasks(&self) -> i64 {
        self.month_completed_tasks
    }

    pub fn day_avg(&self) -> f64 {
        self.day_avg
    }

    pub fn focus_sessions(&self) -> i64 {
        self.focus_sessions
    }
}

#[cfg(test)]
mod tests {
    use crate::value_objects::stats::completed_tasks_counts::CompletedTasksCounts;

    #[test]
    fn new_completed_task_counts() {
        let stat = CompletedTasksCounts::new(10, 20, -4, 47, 3.2, 12);
        assert_eq!(stat.completed_tasks(), 10);
        assert_eq!(stat.week_completed_tasks(), 10);
        assert_eq!(stat.current_week_delta(), -4);
        assert_eq!(stat.month_completed_tasks(), 47);
        assert_eq!(stat.day_avg(), 3.2);
        assert_eq!(stat.focus_sessions(), 12);
    }
}
