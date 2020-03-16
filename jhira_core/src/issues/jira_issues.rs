use std::str::FromStr;

use crate::issues::jira_issue::JiraIssue;

#[derive(Deserialize, Debug)]
pub struct JiraIssues {
    pub issues: Vec<JiraIssue>,
    pub total: u16,
}

impl FromStr for JiraIssues {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let output = serde_json::from_str(s)?;
        Ok(output)
    }
}
