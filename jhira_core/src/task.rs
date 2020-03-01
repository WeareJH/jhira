use crate::http::{Http};
use std::future::Future;
use async_trait::async_trait;

#[derive(Debug)]
pub enum TaskOutput {
    Done { id: i8 },
}

impl TaskOutput {
    pub fn done(id: usize) -> TaskOutput {
        TaskOutput::Done { id: id as i8 }
    }
}

#[async_trait(?Send)]
pub trait Task {
    async fn exec(&self) -> Result<TaskOutput, failure::Error> {
        Ok(TaskOutput::Done { id: -1 })
    }
    async fn dry_run(&self) -> Result<(), failure::Error> {
        Ok(())
    }
}
