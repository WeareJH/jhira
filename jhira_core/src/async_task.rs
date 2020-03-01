use async_trait::async_trait;

#[derive(Debug)]
pub enum TaskOutput {
    Chain(Vec<TaskOutput>),
    String(Vec<String>),
    Done,
    DryRun,
}

#[async_trait(?Send)]
pub trait AsyncTask {
    async fn exec(&self) -> Result<TaskOutput, failure::Error> {
        println!("Missing impl for AsyncTask::exec");
        Ok(TaskOutput::Done)
    }
    async fn dry_run(&self) -> Result<TaskOutput, failure::Error> {
        Ok(TaskOutput::DryRun)
    }
}
