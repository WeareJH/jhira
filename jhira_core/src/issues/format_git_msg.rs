use crate::issues::issues_types::IssueType;
use crate::issues::jira_issues::JiraIssues;
use crate::issues::sort_by::SortBy;
use std::fmt;

#[derive(Debug, Clone)]
pub struct FormatGitMsg {
    pub sort_by: Option<SortBy>,
    pub issues: JiraIssues,
}

impl fmt::Display for FormatGitMsg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = self
            .issues
            .issues
            .iter()
            .map(|issue| {
                let prefix: Prefix = issue.fields.issuetype.clone().into();
                format!(
                    "{prefix}{key} {summary}",
                    prefix = prefix,
                    key = issue.key,
                    summary = issue
                        .fields
                        .summary
                        .as_ref()
                        .unwrap_or(&"summary missing".to_string())
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", output)
    }
}

pub enum Prefix {
    Feat,
    Fix,
    Misc,
}

impl From<IssueType> for Prefix {
    fn from(issue_type: IssueType) -> Self {
        match issue_type.name.as_str() {
            "Story" => Prefix::Feat,
            "Bug" => Prefix::Fix,
            _ => Prefix::Misc,
        }
    }
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Prefix::Fix => "fix: ",
                Prefix::Feat => "feat: ",
                Prefix::Misc => "",
            }
        )
    }
}

#[test]
fn test_format_git() {
    let b = include_str!("../../../fixtures/issues-sub-task.json");
    let i: JiraIssues = serde_json::from_str(b).expect("Should deserialize");
    let f = FormatGitMsg {
        sort_by: None,
        issues: i,
    };
    println!("{}", f);
}
