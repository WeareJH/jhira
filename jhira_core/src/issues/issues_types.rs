use crate::issues::jira_issue::JiraIssue;
use crate::issues::priority::PriorityName;

#[derive(Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct JiraFields {
    pub issuetype: IssueType,
    pub status: IssueType,
    pub summary: Option<String>,
    pub subtasks: Option<Vec<JiraIssue>>,
    pub assignee: Option<Assignee>,
    pub priority: Priority,
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

#[derive(Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Priority {
    pub name: PriorityName,
}
