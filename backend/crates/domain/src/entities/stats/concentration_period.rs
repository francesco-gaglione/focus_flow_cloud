use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ConcentrationPeriodError {
    #[error("Invalid concentration period: {0}")]
    InvalidConcentrationPeriod(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConcentrationPeriod {
    Morning,
    Afternoon,
}

impl ConcentrationPeriod {
    pub fn from_string(value: impl AsRef<str>) -> Result<Self, ConcentrationPeriodError> {
        match value.as_ref() {
            "MORNING" => Ok(ConcentrationPeriod::Morning),
            "AFTERNOON" => Ok(ConcentrationPeriod::Afternoon),
            _ => Err(ConcentrationPeriodError::InvalidConcentrationPeriod(
                value.as_ref().to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::entities::stats::{
        concentration_period::ConcentrationPeriodError, ConcentrationPeriod,
    };

    #[test]
    fn parse_valid_concentration_period() {
        let res = ConcentrationPeriod::from_string("AFTERNOON");
        assert_eq!(res, Ok(ConcentrationPeriod::Afternoon));

        let res = ConcentrationPeriod::from_string("MORNING");
        assert_eq!(res, Ok(ConcentrationPeriod::Morning))
    }

    #[test]
    fn parse_invalid_concentration_period() {
        let res = ConcentrationPeriod::from_string("INVALID");
        assert!(
            res == Err(ConcentrationPeriodError::InvalidConcentrationPeriod(
                "INVALID".to_string()
            ))
        )
    }
}
