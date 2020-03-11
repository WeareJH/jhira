use crate::async_task::{AsyncTask, TaskOutput};
use crate::context::Context;
use crate::http::HttpString;
use crate::http_jql::HttpJql;
use async_trait::async_trait;

use std::sync::{Arc, Mutex};

#[derive(Debug, Default)]
pub struct IssuesFetch {
    pub context: Arc<Context>,
    pub resp: Arc<Mutex<Option<String>>>,

    pub id: Option<Vec<String>>,

    pub project: Option<Vec<String>>,

    pub kind: Option<Vec<String>>,
    pub not_kind: Option<Vec<String>>,

    pub status: Option<Vec<String>>,
    pub not_status: Option<Vec<String>>,

    pub max: Option<u16>,
}

impl IssuesFetch {
    pub fn new(context: Arc<Context>) -> IssuesFetch {
        let resp = Arc::new(Mutex::new(None));
        IssuesFetch {
            context,
            resp,
            ..Default::default()
        }
    }
    ///
    /// Should the issue list be filtered to the current
    /// user? Mostly I think they should, But in cases like
    /// wanting to view epics only, assignee wouldn't work
    ///
    pub fn jql_assignee(&self) -> Option<String> {
        // If we're looking up any epics, don't limit to assigned
        if let Some(kinds) = &self.kind {
            if kinds.iter().any(|n| n == "epic" || n == "Epic") {
                return None;
            }
        }
        // If we've asked for specific IDs, don't limit to assigned
        if self.id.is_some() {
            return None;
        }
        // if we get here, we are limiting the issue search to
        // only include those assigned
        Some(String::from("assignee = currentUser()"))
    }
    ///
    /// Should the issue list be filtered by the issue id?
    ///
    /// eg: `issues ls --id abc-123`
    ///
    pub fn jql_id(&self) -> Option<String> {
        self.id
            .as_ref()
            .map(|ids| format!("issue in ({})", ids.join(",")))
    }
    ///
    /// Should the issue list be sorted by updated time?
    ///
    pub fn jql_order(&self) -> Option<String> {
        Some(String::from("order by updated"))
    }
    ///
    /// Should the issue list be filtered by the issue type?
    ///
    /// eg: `issues ls --kind epic`
    ///
    pub fn jql_kind(&self) -> Option<String> {
        self.kind
            .as_ref()
            .map(|kinds| format!("issuetype in ({})", kinds.join(",")))
    }
    ///
    /// Should the issue list be filtered by the issue type?
    ///
    /// eg: `issues ls --kind epic`
    ///
    pub fn jql_not_kind(&self) -> Option<String> {
        self.not_kind
            .as_ref()
            .map(|kinds| format!("issuetype not in ({})", kinds.join(",")))
    }
    ///
    /// Should the issue list be filtered by the status type
    ///
    /// eg: `issues ls --status 'refinement'`
    ///
    pub fn jql_status(&self) -> Option<String> {
        self.status.as_ref().map(|kinds| {
            let kinds = kinds
                .iter()
                .map(|k| format!(r#""{}""#, k))
                .collect::<Vec<String>>()
                .join(",");
            format!("status in ({})", kinds)
        })
    }
    ///
    /// Should the issue list be filtered by the issue type?
    ///
    /// eg: `issues ls --kind epic`
    ///
    pub fn jql_not_status(&self) -> Option<String> {
        self.not_status
            .as_ref()
            .map(|kinds| format!("status not in ({})", kinds.join(",")))
    }
    ///
    /// Should the issue list be filtered by the project
    ///
    /// eg: `issues ls --kind epic`
    ///
    pub fn jql_project(&self) -> Option<String> {
        self.project
            .as_ref()
            .map(|project| format!("project in ({})", project.join(",")))
    }
    ///
    /// Perform the actual fetch by converting this type into
    /// a `HttpJql`
    ///
    async fn fetch(&self) -> Result<String, failure::Error> {
        let mut jql: HttpJql = self.into();
        let req = jql.max_results(self.max.unwrap_or(100)).build();

        debug!("{:#?}", jql);

        let resp = req.exec_http(self.context.clone()).await?;
        Ok(resp)
    }
}

impl From<Arc<Context>> for IssuesFetch {
    fn from(c: Arc<Context>) -> Self {
        IssuesFetch::new(c)
    }
}

impl From<&IssuesFetch> for HttpJql {
    fn from(fetch: &IssuesFetch) -> Self {
        let and_items: String = vec![
            fetch.jql_assignee(),
            fetch.jql_id(),
            fetch.jql_project(),
            fetch.jql_kind(),
            fetch.jql_not_kind(),
            fetch.jql_status(),
            fetch.jql_not_status(),
        ]
        .into_iter()
        .filter_map(|f| f)
        .collect::<Vec<String>>()
        .join(" AND ");

        let jql = vec![Some(and_items), fetch.jql_order()]
            .into_iter()
            .filter_map(|f| f)
            .collect::<Vec<String>>()
            .join(" ");

        HttpJql::new(jql)
    }
}

#[async_trait(?Send)]
impl AsyncTask for IssuesFetch {
    async fn exec(&self) -> Result<TaskOutput, failure::Error> {
        let resp = self.fetch().await?;
        // fs::write(std::path::PathBuf::from("out.json"), &resp);
        let mut l = self.resp.lock().unwrap();
        *l = Some(resp);
        Ok(TaskOutput::Done)
    }
    async fn dry_run(&self) -> Result<TaskOutput, failure::Error> {
        println!("{:#?}", self);
        Ok(TaskOutput::DryRun)
    }
}

#[tokio::main]
#[test]
async fn test_issues_from_ctx() -> Result<(), failure::Error> {
    use crate::auth::Auth;
    let a = Auth::from_file()?;
    let context = Arc::new(Context { auth: a });
    let mut issues: IssuesFetch = context.into();
    issues.kind = Some(vec![String::from("epic")]);
    let jql: HttpJql = (&issues).into();
    dbg!(jql);
    // let resp = issues.fetch().await?;
    // println!("resp={}", resp);
    Ok(())
}
