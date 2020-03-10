use std::str::FromStr;

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

#[derive(Deserialize, Debug, Clone)]
pub struct JiraIssue {
    pub fields: JiraFields,
    pub key: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JiraFields {
    pub issuetype: IssueType,
    pub status: IssueType,
    pub summary: Option<String>,
    pub subtasks: Option<Vec<JiraIssue>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct IssueType {
    pub name: String,
    pub subtask: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct IssueStatus {
    name: String,
}
