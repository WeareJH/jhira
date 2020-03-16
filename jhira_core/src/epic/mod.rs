use crate::async_task::{AsyncTask, TaskOutput};
use crate::context::Context;
use crate::issues::jira_issue::JiraIssue;
use crate::issues::jira_issues::JiraIssues;
use crate::task::TaskSequence;
use async_trait::async_trait;
use std::sync::Arc;
use structopt::StructOpt;

pub mod epic_display;
pub mod epic_fetch;
pub mod output_compact;

#[derive(Debug)]
pub struct Epic {
    issue: JiraIssue,
    issues: JiraIssues,
}

#[derive(StructOpt, Debug, Clone)]
pub struct EpicCmd {
    pub id: String,
}

impl EpicCmd {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

#[async_trait(?Send)]
impl AsyncTask for EpicCmd {
    async fn exec(&self, ctx: Arc<Context>) -> Result<TaskOutput, failure::Error> {
        let epic: Epic = epic_fetch::fetch(self.id.clone(), ctx.clone()).await?;
        let as_string = epic_display::epic_display(epic, ctx)?;
        Ok(TaskOutput::string(as_string))
    }
}

impl From<EpicCmd> for TaskSequence {
    fn from(epic_cmd: EpicCmd) -> Self {
        Ok(vec![Box::new(epic_cmd)])
    }
}
