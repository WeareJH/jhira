use crate::context::Context;
use crate::http::HttpString;
use crate::http_get::HttpGet;
use crate::issues::issues_display::IssueLink;
use std::str::FromStr;
use std::sync::Arc;

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
            .unwrap_or_else(|| String::from("Missing sumary"))
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
