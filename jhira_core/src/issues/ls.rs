use crate::async_task::{AsyncTask, TaskOutput};
use crate::context::Context;
use crate::http::HttpString;
use crate::http_jql::HttpJql;
use crate::issues::jira_issues::JiraIssues;
use crate::issues::output_compact::{output_compact, CompactOpts};
use crate::issues::output_verbose::output_verbose;
use crate::issues::sort_by::SortBy;
use crate::task::TaskSequence;
use async_trait::async_trait;
use std::str::FromStr;
use std::sync::Arc;
use structopt::StructOpt;

///
/// List issues
///
#[derive(Debug, StructOpt, Clone)]
pub struct IssuesLs {
    /// Issue search against all users (rather than just assigned)
    #[structopt(long = "all")]
    pub all: bool,

    /// Which issue ids to fetch
    #[structopt(long = "verbose", short = "v")]
    pub verbose: bool,

    /// Search by summary, eg: 'checkout'
    #[structopt(long = "summary", short = "s")]
    pub summary: Option<String>,

    /// Which issue ids to fetch
    #[structopt(long = "id")]
    pub id: Option<Vec<String>>,

    /// Which projects to fetch issues for. eg: 'abc'
    #[structopt(long = "project")]
    pub project: Option<Vec<String>>,

    /// Which issue types to show. eg: 'epic' 'bug' 'story'
    #[structopt(short = "k", long = "kind")]
    pub kind: Option<Vec<String>>,

    /// Which issue types to exclude
    #[structopt(long = "not-kind")]
    pub not_kind: Option<Vec<String>>,

    /// Which statuses to include, eg: 'refinement' 'ready' 'validated'
    #[structopt(long = "status")]
    pub status: Option<Vec<String>>,

    /// Which statuses to exclude
    #[structopt(long = "not-status")]
    pub not_status: Option<Vec<String>>,

    /// Max number of results to fetch
    #[structopt(long = "max")]
    pub max: Option<u16>,

    /// The id of an epic
    #[structopt(long = "epic")]
    pub epic: Option<String>,

    /// Which order to show the results in
    #[structopt(long = "sort")]
    pub sort: Option<SortBy>,
}

impl IssuesLs {
    ///
    /// Decide if we're only looking at a currentUser
    ///
    pub fn current_user_only(&self) -> bool {
        if let Some(kinds) = &self.kind {
            if kinds.iter().any(|n| n == "epic" || n == "Epic") {
                return false;
            }
        }
        // If we've asked for specific IDs, don't limit to assigned
        if self.id.is_some() {
            return false;
        }

        if self.all {
            return false;
        }

        // if it's explicitly an epic, get all issues
        if self.epic.is_some() {
            return false;
        }

        true
    }
    ///
    /// Should the issue list be filtered to the current
    /// user? Mostly I think they should, But in cases like
    /// wanting to view epics only, assignee wouldn't work
    ///
    pub fn jql_assignee(&self) -> Option<String> {
        if self.current_user_only() {
            // if we get here, we are limiting the issue search to
            // only include those assigned
            return Some(String::from("assignee = currentUser()"));
        }
        None
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
    /// Should the issue list be orderd by updated time (jql only)
    ///
    pub fn jql_order(&self) -> Option<String> {
        Some(String::from("order by updated"))
    }
    ///
    /// Should the issue list be filtered by a single epic
    ///
    /// eg: `issues ls --kind epic`
    ///
    pub fn jql_epic(&self) -> Option<String> {
        self.epic
            .as_ref()
            .map(|epic_key| format!(r#""Epic Link" = {id}"#, id = epic_key))
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
    /// Should the issue list be filtered by a search string
    ///
    /// eg: `issues ls --kind epic`
    ///
    pub fn jql_summary(&self) -> Option<String> {
        self.summary
            .as_ref()
            .map(|summary| format!(r#"summary ~ "{}""#, snailquote::escape(summary)))
    }
    ///
    /// Perform the actual fetch by converting this type into
    /// a `HttpJql`
    ///
    async fn fetch(&self, ctx: Arc<Context>) -> Result<JiraIssues, failure::Error> {
        let mut jql: HttpJql = self.into();
        let req = jql.max_results(self.max.unwrap_or(100)).build();

        debug!("jql = {:#?}", jql);

        let resp = req.exec_http(ctx.clone()).await?;

        let issues = JiraIssues::from_str(&resp)?;

        Ok(issues)
    }
}

impl From<&IssuesLs> for HttpJql {
    fn from(fetch: &IssuesLs) -> Self {
        let and_items: String = vec![
            fetch.jql_assignee(),
            fetch.jql_id(),
            fetch.jql_epic(),
            fetch.jql_summary(),
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
impl AsyncTask for IssuesLs {
    async fn exec(&self, ctx: Arc<Context>) -> Result<TaskOutput, failure::Error> {
        // fetch the issues
        let issues = self.fetch(ctx.clone()).await?;

        // output the issues
        let show_assignee = !self.current_user_only();
        let output = if self.verbose {
            output_verbose(&issues, &ctx)
        } else {
            output_compact(
                &issues,
                &ctx,
                CompactOpts {
                    show_assignee,
                    sort_by: self.sort.clone(),
                },
            )
        };

        Ok(TaskOutput::string(output))
    }
    async fn dry_run(&self) -> Result<TaskOutput, failure::Error> {
        println!("{:#?}", self);
        Ok(TaskOutput::DryRun)
    }
}

impl From<IssuesLs> for TaskSequence {
    fn from(ls: IssuesLs) -> Self {
        Ok(vec![Box::new(ls)])
    }
}
