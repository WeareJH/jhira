use crate::async_task::{AsyncTask, TaskOutput};
use crate::context::Context;
use crate::issues::issues_types::JiraIssues;
use crate::issues::output_compact::output_compact;
use crate::issues::output_verbose::output_verbose;
use async_trait::async_trait;
use std::fmt;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

pub struct IssuesDisplay {
    pub resp: Arc<Mutex<Option<String>>>,
    pub context: Arc<Context>,
    pub verbose: bool,
}

#[derive(Fail, Debug)]
enum IssuesDisplayError {
    #[fail(display = "Response missing")]
    Missing,
}

#[async_trait(? Send)]
impl AsyncTask for IssuesDisplay {
    async fn exec(&self) -> Result<TaskOutput, failure::Error> {
        let resp = self.resp.lock().unwrap();
        let resp_string = resp.clone().ok_or(IssuesDisplayError::Missing)?;
        let issues = JiraIssues::from_str(&resp_string)?;
        let output = if self.verbose {
            output_verbose(&issues, &self.context)
        } else {
            output_compact(&issues, &self.context)
        };
        Ok(TaskOutput::String(vec![output]))
    }
}

pub struct IssueLink(pub String);

impl IssueLink {
    pub fn from_context(ctx: &Context, key: &str) -> IssueLink {
        IssueLink(format!(
            "https://{}.atlassian.net/browse/{}",
            ctx.auth.domain, key
        ))
    }
    pub fn http_get(ctx: &Context, key: &str) -> IssueLink {
        IssueLink(format!(
            "https://{}.atlassian.net/rest/api/3/issue/{}",
            ctx.auth.domain, key
        ))
    }
}

impl fmt::Display for IssueLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<IssueLink> for String {
    fn from(link: IssueLink) -> Self {
        format!("{}", link)
    }
}
