use crate::async_task::{AsyncTask, TaskOutput};

use async_trait::async_trait;

pub mod cmd;

#[derive(Debug)]
pub struct Issues {
    pub url: String,
}

#[async_trait(?Send)]
impl AsyncTask for Issues {
    async fn exec(&self) -> Result<TaskOutput, failure::Error> {
        Ok(TaskOutput::String(vec![String::from("Output")]))
    }
    async fn dry_run(&self) -> Result<TaskOutput, failure::Error> {
        println!("{:#?}", self);
        Ok(TaskOutput::DryRun)
    }
}
