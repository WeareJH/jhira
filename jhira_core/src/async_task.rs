use async_trait::async_trait;

pub type Return = Result<TaskOutput, failure::Error>;

#[derive(Debug)]
pub enum TaskOutput {
    Chain(Vec<TaskOutput>),
    String(Vec<String>),
    Done,
    DryRun,
}

impl TaskOutput {
    pub fn string(s: impl Into<String>) -> TaskOutput {
        TaskOutput::String(vec![s.into()])
    }
}

#[async_trait(?Send)]
pub trait AsyncTask {
    async fn exec(&self) -> Return {
        println!("Missing impl for AsyncTask::exec");
        Ok(TaskOutput::Done)
    }
    async fn dry_run(&self) -> Result<TaskOutput, failure::Error> {
        Ok(TaskOutput::DryRun)
    }
}
