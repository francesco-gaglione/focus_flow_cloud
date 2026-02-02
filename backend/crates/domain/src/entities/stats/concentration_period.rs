#[derive(Debug, Clone, PartialEq)]
pub enum ConcentrationPeriod {
    Morning,
    Afternoon,
}

impl ConcentrationPeriod {
    pub fn from_string(value: String) -> Self {
        match value.as_str() {
            "MORNING" => ConcentrationPeriod::Morning,
            "AFTERNOON" => ConcentrationPeriod::Afternoon,
            _ => throw_invalid_arg(value),
        }
    }
}

fn throw_invalid_arg(value: String) -> ConcentrationPeriod {
    panic!("Invalid ConcentrationPeriod: {}", value);
}
