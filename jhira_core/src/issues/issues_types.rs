use crate::issues::jira_issue::JiraIssue;

#[derive(Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct JiraFields {
    pub issuetype: IssueType,
    pub status: IssueType,
    pub summary: Option<String>,
    pub subtasks: Option<Vec<JiraIssue>>,
    pub assignee: Option<Assignee>,
}

#[derive(Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct IssueType {
    pub name: String,
    pub subtask: Option<bool>,
}

#[derive(Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct IssueStatus {
    pub name: String,
}

#[derive(Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Assignee {
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
}
