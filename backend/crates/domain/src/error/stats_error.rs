use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum StatsError {
    #[error("No data available")]
    NoData,
}

pub type StatsResult<T> = Result<T, StatsError>;
