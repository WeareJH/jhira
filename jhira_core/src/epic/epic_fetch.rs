use crate::async_task::{AsyncTask, Return, TaskOutput};
use crate::context::Context;
use crate::epic::Epic;
use crate::http::HttpString;
use crate::http_jql::HttpJql;
use crate::issues::issues_types::{JiraIssue, JiraIssues};
use async_trait::async_trait;

use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct EpicFetch {
    pub id: String,
    pub context: Arc<Context>,
    pub epic: Arc<Mutex<Option<Epic>>>,
}

#[derive(Debug, Fail)]
pub enum EpicError {
    #[fail(display = "This issue is not an epic, it's a `{}`", _0)]
    NotAnEpic(String),
}

#[async_trait(?Send)]
impl AsyncTask for EpicFetch {
    async fn exec(&self) -> Return {
        // first try to get the main issue
        let issue = JiraIssue::fetch(&self.id, self.context.clone()).await?;

        // if the main issue is successful, check it's an epic.
        if !issue.is_epic() {
            return Err(EpicError::NotAnEpic(issue.fields.issuetype.name.clone()).into());
        }

        // now fetch associated issues
        let query = format!(
            r#""Epic Link" = {id} OR parent in ("{id}")"#,
            id = issue.key
        );
        let jql = HttpJql::new(query).exec_http(self.context.clone()).await?;
        // fs::write(std::path::PathBuf::from("epic-issues.json"), &jql);
        let issues: JiraIssues = serde_json::from_str(&jql)?;

        // now write to the mutexes
        let mut out_epic = self.epic.lock().expect("epic mutex");
        *out_epic = Some(Epic { issue, issues });

        Ok(TaskOutput::Done)
    }
    async fn dry_run(&self) -> Result<TaskOutput, failure::Error> {
        dbg!(&self);
        Ok(TaskOutput::DryRun)
    }
}
