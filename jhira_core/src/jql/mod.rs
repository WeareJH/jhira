use crate::async_task::{AsyncTask, Return, TaskOutput};
use crate::context::Context;
use crate::http::HttpString;
use crate::http_jql::HttpJql;
use crate::issues::jira_issues::JiraIssues;
use crate::issues::output_compact::{output_compact, CompactOpts};
use crate::task::TaskSequence;
use async_trait::async_trait;
use std::str::FromStr;
use std::sync::Arc;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone, Default)]
pub struct JqlCmd {
    jql: HttpJql,
    #[structopt(long = "json")]
    json: bool,
    #[structopt(long = "max")]
    max: Option<u16>,
    #[structopt(long = "fields")]
    fields: Option<Vec<String>>,
}

impl JqlCmd {
    pub fn new(jql: HttpJql, json: bool) -> Self {
        Self {
            jql,
            json,
            ..Self::default()
        }
    }
}

impl From<JqlCmd> for TaskSequence {
    fn from(jql: JqlCmd) -> Self {
        Ok(vec![Box::new(jql)])
    }
}

#[async_trait(?Send)]
impl AsyncTask for JqlCmd {
    async fn exec(&self, ctx: Arc<Context>) -> Return {
        let jql_http = self
            .jql
            .clone()
            .max_opt(self.max)
            .fields_opt(self.fields.clone())
            .build();
        let resp = jql_http.exec_http(ctx.clone()).await?;
        if self.json {
            Ok(TaskOutput::string(resp))
        } else {
            let issues = JiraIssues::from_str(&resp)?;
            let output = output_compact(
                &issues,
                &ctx.clone(),
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
