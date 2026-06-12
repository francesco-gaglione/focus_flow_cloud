use chrono::Duration;

pub struct CompletedFocusSessions {
    count: usize,
    avg_duration: Duration,
}

impl CompletedFocusSessions {
    pub fn new(count: usize, avg_duration: Duration) -> Self {
        Self {
            count,
            avg_duration,
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn avg_duration(&self) -> Duration {
        self.avg_duration
    }
}
