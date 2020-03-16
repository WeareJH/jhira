use crate::issues::jira_issue::JiraIssue;

pub enum SortBy {
    IssueType,
    Status,
    Assignee,
    // Project,
    Summary,
}

impl SortBy {
    pub fn sort(&self, mut issues: Vec<JiraIssue>) -> Vec<JiraIssue> {
        issues.sort_by(|a, b| {
            match self {
                SortBy::IssueType => a.fields.issuetype.cmp(&b.fields.issuetype),
                SortBy::Status => a.fields.status.cmp(&b.fields.status),
                SortBy::Assignee => a.fields.assignee.cmp(&b.fields.assignee),
                // SortBy::Project => a.fields.,
                SortBy::Summary => a.fields.summary.cmp(&b.fields.summary),
            }
        });
        issues
    }
}
