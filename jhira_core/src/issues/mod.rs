use crate::async_task::{AsyncTask, TaskOutput};

use async_trait::async_trait;
use crate::auth::Auth;
use crate::http::HttpGet;

pub mod cmd;

#[derive(Debug)]
pub struct Issues {
    pub url: String,
    pub auth: Auth,
}

#[async_trait(?Send)]
impl AsyncTask for Issues {
    async fn exec(&self) -> Result<TaskOutput, failure::Error> {
        let h = HttpGet{url: self.url.clone()};
        let r = h.exec(&self.auth).await?;
        Ok(TaskOutput::String(vec![r]))
    }
    async fn dry_run(&self) -> Result<TaskOutput, failure::Error> {
        println!("{:#?}", self);
        Ok(TaskOutput::DryRun)
    }
}
