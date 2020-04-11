use crate::issues::jira_issue::JiraIssue;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum SortBy {
    IssueType,
    Status,
    Assignee,
    // Project,
    Summary,
    Key,
}

const HELP: &str = "

    To sort on issue type: 'kind' or 'type'
    To sort on status:     'status'
    To sort on assignee:   'assignee' or 'name'
    To sort on title:      'summary' or 'title'
    To sort on id:         'key' or 'id'
";

#[derive(Fail, Debug)]
pub enum SortByError {
    #[fail(display = "Provided: {} - these are valid: {}", given, valid)]
    Invalid { given: String, valid: String },
}

impl FromStr for SortBy {
    type Err = SortByError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "kind" | "type" => Ok(SortBy::IssueType),
            "status" => Ok(SortBy::Status),
            "assignee" | "name" => Ok(SortBy::Assignee),
            "summary" | "title" => Ok(SortBy::Summary),
            "key" | "id" => Ok(SortBy::Key),
            _ => {
                let err = SortByError::Invalid {
                    given: String::from(s),
                    valid: String::from(HELP),
                };
                Err(err)
            }
        }
    }
}

impl SortBy {
    pub fn sort(&self, mut issues: Vec<JiraIssue>) -> Vec<JiraIssue> {
        issues.sort_by(|a, b| {
            match self {
                SortBy::IssueType => a.fields.issuetype.cmp(&b.fields.issuetype),
                SortBy::Status => a.fields.status.cmp(&b.fields.status),
                SortBy::Assignee => a.fields.assignee.cmp(&b.fields.assignee),
                SortBy::Key => a.key.cmp(&b.key),
                // SortBy::Project => a.fields.,
                SortBy::Summary => a.fields.summary.cmp(&b.fields.summary),
            }
        });
        issues
    }
}
