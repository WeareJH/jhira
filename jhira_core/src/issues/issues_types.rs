use std::str::FromStr;

#[derive(Deserialize, Debug)]
pub struct JiraIssues {
    pub issues: Vec<JiraIssue>,
}

impl FromStr for JiraIssues {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let output = serde_json::from_str(s)?;
        Ok(output)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct JiraIssue {
    // pub fields: JiraField,
    pub key: String,
}
