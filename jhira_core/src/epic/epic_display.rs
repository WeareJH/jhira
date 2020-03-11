use crate::context::Context;

use async_trait::async_trait;
use std::sync::{Arc, Mutex};

use crate::async_task::{AsyncTask, Return, TaskOutput};
use crate::epic::output_compact::output_compact;
use crate::epic::Epic;

#[derive(Debug)]
pub struct EpicDisplay {
    pub id: String,
    pub context: Arc<Context>,

    pub epic: Arc<Mutex<Option<Epic>>>,
}

#[derive(Debug, Fail)]
pub enum EpicDisplayError {
    #[fail(display = "Missing data epic data.")]
    Missing,
}

#[async_trait(?Send)]
impl AsyncTask for EpicDisplay {
    async fn exec(&self) -> Return {
        let epic = self.epic.lock().expect("epic mutex unlock");
        let epic = epic.as_ref().ok_or(EpicDisplayError::Missing)?;
        let output = output_compact(&epic, &self.context.clone());
        Ok(TaskOutput::string(output))
    }
    async fn dry_run(&self) -> Result<TaskOutput, failure::Error> {
        dbg!(&self);
        Ok(TaskOutput::DryRun)
    }
}
