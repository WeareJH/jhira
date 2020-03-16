use crate::context::Context;
use crate::http::HttpString;
use crate::http_get::HttpGet;

use crate::issues::issue_link::IssueLink;
use crate::issues::issues_types::JiraFields;
use std::sync::Arc;

#[derive(Deserialize, Debug, Clone)]
pub struct JiraIssue {
    pub fields: JiraFields,
    pub key: String,
}

impl JiraIssue {
    pub fn is_epic(&self) -> bool {
        self.fields.issuetype.name == "Epic"
    }
    pub fn summary(&self) -> String {
        self.fields
            .summary
            .as_ref()
            .clone()
            .map(|x| x.to_owned())
            .unwrap_or_else(|| String::from("Missing summary"))
    }
    pub fn short_summary(&self) -> String {
        JiraIssue::_short_summary(&self.summary())
    }
    pub fn assignee_name(&self) -> Option<String> {
        self.fields
            .assignee
            .as_ref()
            .and_then(|x| x.display_name.clone())
    }
    pub fn _short_summary(s: &str) -> String {
        let limit = 50;
        let padding = 3;
        let len = s.len();
        if s.len() > limit {
            let diff = len - limit;
            if diff > 0 {
                return format!("{}...", s.chars().take(limit - padding).collect::<String>());
            } else {
                s.chars().take(limit).collect()
            }
        } else {
            s.to_string()
        }
    }
    pub async fn fetch(
        id: impl Into<String>,
        context: Arc<Context>,
    ) -> Result<JiraIssue, failure::Error> {
        let id = id.into();
        let url = IssueLink::http_get(&context.clone(), &id);
        let resp = HttpGet::new(url).exec_http(context.clone()).await?;
        // fs::write(std::path::PathBuf::from("out.json"), &resp);
        let epic_issue: JiraIssue = serde_json::from_str(&resp)?;
        Ok(epic_issue)
    }
}
