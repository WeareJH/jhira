use std::str::FromStr;

#[derive(Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum PriorityName {
    Low,
    Medium,
    High,
    Critical,
}

impl FromStr for PriorityName {
    type Err = PriorityError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Low" => Ok(PriorityName::Low),
            "Medium" => Ok(PriorityName::Medium),
            "High" => Ok(PriorityName::High),
            "Critical" => Ok(PriorityName::Critical),
            _ => {
                let err = PriorityError::Invalid {
                    name: String::from(s),
                };
                Err(err)
            }
        }
    }
}

impl ToString for PriorityName {
    fn to_string(&self) -> String {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::Critical => "Critical",
        }
        .to_string()
    }
}

#[derive(Fail, Debug)]
pub enum PriorityError {
    #[fail(display = "Priority not support: {}", name)]
    Invalid { name: String },
}
