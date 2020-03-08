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

    for v in &issues.issues {
        let row_1 = &v.fields.issuetype.name;
        let row_2 = &v.fields.status.name;
        let row_3 = IssueLink::from_context(&context, &v.key);

        table.add_row(row![
            row_1, row_2, row_3,
            // row_4
        ]);

        if let Some(ref sub) = v.fields.subtasks {
            let iter = sub.iter().enumerate();
            let count = iter.len();
            for (i, v) in iter {
                let row_1 = &v.fields.issuetype.name;
                let row_2 = &v.fields.status.name;
                let is_last = i + 1 == count;
                let prefix = if is_last { "└─" } else { "├─" };
                let row_3 = format!("{} {}", prefix, IssueLink::from_context(&context, &v.key));
                table.add_row(row![
                    row_1, row_2, row_3,
                    // row_4
                ]);
            }
        }
    }

    let issue_table = table.to_string();
    let summary_table = summary_table(&issues);
    format!("{}\n{}", issue_table, summary_table)
}

fn summary_table(issues: &JiraIssues) -> String {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
    table.add_row(row!["total", issues.total]);
    table.to_string()
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
