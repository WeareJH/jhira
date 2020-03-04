use crate::async_task::{AsyncTask, TaskOutput};
use crate::context::Context;
use crate::issues::issues_types::JiraIssues;
use async_trait::async_trait;
use prettytable::format;
use prettytable::Table;
use std::fmt;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

pub struct IssuesDisplay {
    pub resp: Arc<Mutex<Option<String>>>,
    pub context: Arc<Context>,
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
        let output = display(JiraIssues::from_str(&resp_string)?, &self.context);
        Ok(TaskOutput::String(vec![output]))
    }
}

fn display(issues: JiraIssues, context: &Context) -> String {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
    for i in issues.issues {
        table.add_row(row![IssueLink::from_context(&context, &i.key), "summary"]);
    }
    let s = table.to_string();
    s
}

pub struct IssueLink(pub String);

impl IssueLink {
    pub fn from_context(ctx: &Context, key: &str) -> IssueLink {
        IssueLink(format!(
            "https://{}.atlassian.net/browse/{}",
            ctx.auth.domain, key
        ))
    }
}

impl fmt::Display for IssueLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
