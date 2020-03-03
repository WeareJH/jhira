use crate::async_task::{AsyncTask, TaskOutput};
use crate::issues::issues_types::JiraIssues;
use async_trait::async_trait;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

pub struct IssuesDisplay {
    pub resp: Arc<Mutex<Option<String>>>,
}

#[derive(Fail, Debug)]
enum IssuesDisplayError {
    #[fail(display = "Response missing")]
    Missing,
}

#[async_trait(?Send)]
impl AsyncTask for IssuesDisplay {
    async fn exec(&self) -> Result<TaskOutput, failure::Error> {
        let resp = self.resp.lock().unwrap();
        let c = resp.clone().ok_or(IssuesDisplayError::Missing)?;
        let d = JiraIssues::from_str(&c)?;
        dbg!(d);
        Ok(TaskOutput::Done)
    }
}
