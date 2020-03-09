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

    pub kind: Option<Vec<String>>,
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
        if let Some(kinds) = &self.kind {
            if kinds.iter().any(|n| n == "epic" || n == "Epic") {
                return None;
            }
        }
        Some(String::from("assignee = currentUser()"))
    }
    ///
    /// Should the issue list be filtered by status?
    ///
    pub fn jql_status(&self) -> Option<String> {
        Some(String::from("status not in (Validated)"))
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
    pub fn jql_issuetype(&self) -> Option<String> {
        self.kind
            .as_ref()
            .map(|kinds| format!("issuetype in ({})", kinds.join(",")))
    }
    ///
    /// Perform the actual fetch by converting this type into
    /// a `HttpJql`
    ///
    async fn fetch(&self) -> Result<String, failure::Error> {
        let mut jql: HttpJql = self.into();
        dbg!(&jql);
        let resp = jql
            .max_results(self.max.unwrap_or(100))
            .build()
            .exec(self.context.clone())
            .await?;
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
            fetch.jql_issuetype(),
            fetch.jql_status(),
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

        HttpJql::new(jql).max_results(5).build()
    }
}

#[async_trait(?Send)]
impl AsyncTask for IssuesFetch {
    async fn exec(&self) -> Result<TaskOutput, failure::Error> {
        let resp = self.fetch().await?;
        // fs::write(PathBuf::from("out.json"), &resp);
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
