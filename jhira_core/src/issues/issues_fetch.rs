use crate::async_task::{AsyncTask, TaskOutput};
use crate::context::Context;
use crate::http::HttpString;
use crate::http_jql::HttpJql;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct IssuesFetch {
    pub context: Arc<Context>,
    pub resp: Arc<Mutex<Option<String>>>,
}

impl IssuesFetch {
    pub fn new(context: Arc<Context>) -> IssuesFetch {
        let resp = Arc::new(Mutex::new(None));
        IssuesFetch { context, resp }
    }
}

#[async_trait(?Send)]
impl AsyncTask for IssuesFetch {
    async fn exec(&self) -> Result<TaskOutput, failure::Error> {
        let resp = HttpJql::new("assignee = currentUser()")
            .max_results(2)
            .build()
            .exec(self.context.clone())
            .await?;
        let mut l = self.resp.lock().unwrap();
        *l = Some(resp);
        Ok(TaskOutput::Done)
    }
    async fn dry_run(&self) -> Result<TaskOutput, failure::Error> {
        println!("{:#?}", self);
        Ok(TaskOutput::DryRun)
    }
}
