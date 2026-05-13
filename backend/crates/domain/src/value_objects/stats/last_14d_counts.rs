use chrono::{Days, NaiveDate, Utc};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Last14dCountsError {
    #[error("day is not within the last 14 days")]
    DayNotWithinLast14Days,
    #[error("day already exists")]
    DayAlreadyExists,
}

pub struct DayCounts {
    day: NaiveDate,
    count: usize,
}

impl DayCounts {
    pub fn new(day: NaiveDate, count: usize) -> Self {
        Self { day, count }
    }

    pub fn day(&self) -> NaiveDate {
        self.day
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

pub struct Last14dCounts {
    counts: Vec<DayCounts>,
}

impl Last14dCounts {
    pub fn new() -> Self {
        Self { counts: Vec::new() }
    }

    pub fn push(&mut self, day: NaiveDate, count: usize) -> Result<(), Last14dCountsError> {
        let current_date = Utc::now().date_naive();
        let fourteen_days_ago = current_date
            .checked_sub_days(Days::new(13))
            .unwrap_or(NaiveDate::MIN);
        if day < fourteen_days_ago || day > current_date {
            return Err(Last14dCountsError::DayNotWithinLast14Days);
        }

        if let Some(index) = self.counts.iter().position(|d| d.day == day) {
            let existing_day_counts = &mut self.counts[index];
            self.counts[index] = DayCounts::new(day, existing_day_counts.count() + count);
            return Ok(());
        }

        self.counts.push(DayCounts::new(day, count));
        Ok(())
    }

    pub fn counts(&self) -> &[DayCounts] {
        &self.counts
    }

    pub fn total(&self) -> usize {
        self.counts.iter().map(|d| d.count).sum()
    }

    pub fn max(&self) -> usize {
        self.counts.iter().map(|d| d.count).max().unwrap_or(0)
    }

    pub fn find_by_day(&self, day: NaiveDate) -> Option<&DayCounts> {
        self.counts.iter().find(|d| d.day == day)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Days, NaiveDate, Utc};

    fn today() -> NaiveDate {
        Utc::now().date_naive()
    }

    fn days_ago(n: u64) -> NaiveDate {
        today().checked_sub_days(Days::new(n)).unwrap()
    }

    #[test]
    fn test_day_counts_getters() {
        let dc = DayCounts::new(today(), 5);
        assert_eq!(dc.day(), today());
        assert_eq!(dc.count(), 5);
    }

    #[test]
    fn test_push_today() {
        let mut c = Last14dCounts::new();
        assert!(c.push(today(), 3).is_ok());
        assert_eq!(c.counts().len(), 1);
        assert_eq!(c.total(), 3);
    }

    #[test]
    fn test_push_13_days_ago() {
        let mut c = Last14dCounts::new();
        assert!(c.push(days_ago(13), 2).is_ok());
    }

    #[test]
    fn test_push_too_old_rejected() {
        let mut c = Last14dCounts::new();
        let too_old = days_ago(14);
        assert!(matches!(
            c.push(too_old, 1),
            Err(Last14dCountsError::DayNotWithinLast14Days)
        ));
    }

    #[test]
    fn test_push_future_rejected() {
        let mut c = Last14dCounts::new();
        let future = today().checked_add_days(Days::new(1)).unwrap();
        assert!(matches!(
            c.push(future, 1),
            Err(Last14dCountsError::DayNotWithinLast14Days)
        ));
    }

    #[test]
    fn test_push_duplicate_accumulates() {
        let mut c = Last14dCounts::new();
        c.push(today(), 3).unwrap();
        c.push(today(), 5).unwrap();
        assert_eq!(c.counts().len(), 1);
        assert_eq!(c.find_by_day(today()).unwrap().count(), 8);
    }

    #[test]
    fn test_total() {
        let mut c = Last14dCounts::new();
        c.push(today(), 3).unwrap();
        c.push(days_ago(1), 7).unwrap();
        assert_eq!(c.total(), 10);
    }

    #[test]
    fn test_total_empty() {
        assert_eq!(Last14dCounts::new().total(), 0);
    }

    #[test]
    fn test_max() {
        let mut c = Last14dCounts::new();
        c.push(today(), 2).unwrap();
        c.push(days_ago(1), 9).unwrap();
        c.push(days_ago(2), 4).unwrap();
        assert_eq!(c.max(), 9);
    }

    #[test]
    fn test_max_empty() {
        assert_eq!(Last14dCounts::new().max(), 0);
    }

    #[test]
    fn test_find_by_day_found() {
        let mut c = Last14dCounts::new();
        c.push(today(), 7).unwrap();
        assert_eq!(c.find_by_day(today()).unwrap().count(), 7);
    }

    #[test]
    fn test_find_by_day_not_found() {
        let c = Last14dCounts::new();
        assert!(c.find_by_day(today()).is_none());
    }
}
