use crate::entities::focus_session::FocusSession;
use crate::entities::stats::concentration_period::ConcentrationPeriod;

use chrono::Timelike;

#[derive(Debug, Clone)]
pub struct ConcentrationStats {
    pub most_concentrated_period: ConcentrationPeriod,
    pub less_concentrated_period: ConcentrationPeriod,
    pub concentration_distribution: [u32; 5],
}

pub struct ConcentrationCalculator;

impl ConcentrationCalculator {
    pub fn calculate(sessions: &[FocusSession]) -> ConcentrationStats {
        ConcentrationStats {
            most_concentrated_period: Self::calculate_most_concentrated_period(sessions),
            less_concentrated_period: Self::calculate_least_concentrated_period(sessions),
            concentration_distribution: Self::calculate_concentration_distribution(sessions),
        }
    }

    fn calculate_concentration_distribution(sessions: &[FocusSession]) -> [u32; 5] {
        let mut distribution = [0u32; 5];

        for session in sessions {
            if let Some(score) = session.concentration_score() {
                if score > 0 {
                    let index = (score - 1) as usize;
                    if let Some(count) = distribution.get_mut(index) {
                        *count += 1;
                    }
                }
            }
        }

        distribution
    }

    fn calculate_most_concentrated_period(sessions: &[FocusSession]) -> ConcentrationPeriod {
        let (morning_total, morning_count, afternoon_total, afternoon_count) =
            sessions.iter().fold((0, 0, 0, 0), |acc, session| {
                if let Some(score) = session.concentration_score() {
                    let hour = session.started_at().hour();
                    if hour < 12 {
                        (acc.0 + score, acc.1 + 1, acc.2, acc.3)
                    } else {
                        (acc.0, acc.1, acc.2 + score, acc.3 + 1)
                    }
                } else {
                    acc
                }
            });

        let morning_avg = if morning_count > 0 {
            morning_total as f64 / morning_count as f64
        } else {
            0.0
        };

        let afternoon_avg = if afternoon_count > 0 {
            afternoon_total as f64 / afternoon_count as f64
        } else {
            0.0
        };

        if morning_avg >= afternoon_avg {
            ConcentrationPeriod::Morning
        } else {
            ConcentrationPeriod::Afternoon
        }
    }

    fn calculate_least_concentrated_period(sessions: &[FocusSession]) -> ConcentrationPeriod {
        let (morning_total, morning_count, afternoon_total, afternoon_count) =
            sessions.iter().fold((0, 0, 0, 0), |acc, session| {
                if let Some(score) = session.concentration_score() {
                    let hour = session.started_at().hour();
                    if hour < 12 {
                        (acc.0 + score, acc.1 + 1, acc.2, acc.3)
                    } else {
                        (acc.0, acc.1, acc.2 + score, acc.3 + 1)
                    }
                } else {
                    acc
                }
            });

        let morning_avg = if morning_count > 0 {
            morning_total as f64 / morning_count as f64
        } else {
            f64::MAX
        };

        let afternoon_avg = if afternoon_count > 0 {
            afternoon_total as f64 / afternoon_count as f64
        } else {
            f64::MAX
        };

        if morning_avg <= afternoon_avg {
            ConcentrationPeriod::Morning
        } else {
            ConcentrationPeriod::Afternoon
        }
    }
}
