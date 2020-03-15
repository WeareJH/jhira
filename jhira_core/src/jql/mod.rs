use crate::async_task::{AsyncTask, Return, TaskOutput};
use crate::context::Context;
use crate::http::HttpString;
use crate::http_jql::HttpJql;
use crate::issues::issues_types::JiraIssues;
use crate::issues::output_compact::{output_compact, CompactOpts};
use crate::task::TaskSequence;
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct JqlCmd {
    jql: HttpJql,
    context: Arc<Context>,
    json: bool,
}

impl JqlCmd {
    pub fn new(jql: HttpJql, json: bool, context: Arc<Context>) -> Self {
        Self { jql, context, json }
    }
}

impl From<JqlCmd> for TaskSequence {
    fn from(jql: JqlCmd) -> Self {
        Ok(vec![Box::new(jql)])
    }
}

#[async_trait(?Send)]
impl AsyncTask for JqlCmd {
    async fn exec(&self) -> Return {
        let resp = self.jql.exec_http(self.context.clone()).await?;
        if self.json {
            Ok(TaskOutput::string(resp))
        } else {
            let issues: JiraIssues = serde_json::from_str(&resp)?;
            let output = output_compact(
                &issues,
                &self.context.clone(),
                CompactOpts {
                    show_assignee: true,
                },
            );
            Ok(TaskOutput::string(output))
        }
    }
    async fn dry_run(&self) -> Result<TaskOutput, failure::Error> {
        dbg!(&self);
        Ok(TaskOutput::DryRun)
    }
}
