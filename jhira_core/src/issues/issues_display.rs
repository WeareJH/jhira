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
        let resp_string = resp.clone().ok_or(IssuesDisplayError::Missing)?;
        let _parsed_issues = JiraIssues::from_str(&resp_string)?;
        Ok(TaskOutput::Done)
    }
}
