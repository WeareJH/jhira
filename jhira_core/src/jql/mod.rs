use crate::async_task::{AsyncTask, Return, TaskOutput};
use crate::context::Context;
use crate::http::HttpString;
use crate::http_jql::HttpJql;
use crate::task::TaskSequence;
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct JqlCmd {
    jql: HttpJql,
    context: Arc<Context>,
}

impl JqlCmd {
    pub fn new(jql: HttpJql, context: Arc<Context>) -> Self {
        Self { jql, context }
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
        Ok(TaskOutput::string(resp))
    }
    async fn dry_run(&self) -> Result<TaskOutput, failure::Error> {
        dbg!(&self);
        Ok(TaskOutput::DryRun)
    }
}
