use crate::context::Context;
use crate::epic::Epic;
use crate::http::HttpString;
use crate::http_jql::HttpJql;

use crate::issues::jira_issue::JiraIssue;
use crate::issues::jira_issues::JiraIssues;
use std::sync::Arc;

#[derive(Debug, Fail)]
pub enum EpicError {
    #[fail(display = "This issue is not an epic, it's a `{}`", _0)]
    NotAnEpic(String),
}

pub async fn fetch(id: impl Into<String>, ctx: Arc<Context>) -> Result<Epic, failure::Error> {
    // first try to get the main issue
    let issue = JiraIssue::fetch(id, ctx.clone()).await?;

    // if the main issue is successful, check it's an epic.
    if !issue.is_epic() {
        return Err(EpicError::NotAnEpic(issue.fields.issuetype.name.clone()).into());
    }

    // now fetch associated issues
    let query = format!(
        r#""Epic Link" = {id} OR parent in ("{id}")"#,
        id = issue.key
    );
    let jql = HttpJql::new(query).exec_http(ctx.clone()).await?;
    // fs::write(std::path::PathBuf::from("epic-issues.json"), &jql);
    let issues: JiraIssues = serde_json::from_str(&jql)?;

    Ok(Epic { issue, issues })
}
