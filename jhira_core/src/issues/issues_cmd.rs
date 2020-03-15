use crate::async_task::AsyncTask;
use structopt::clap::AppSettings;
use structopt::StructOpt;

use crate::context::Context;
use crate::issues::issues_display::IssuesDisplay;
use crate::issues::issues_fetch::IssuesFetch;
use std::sync::Arc;

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
pub enum IssuesCmd {
    /// List issues assigned to you
    #[structopt(name = "ls")]
    List {
        /// Issue search against all users (rather than just assigned)
        #[structopt(long = "all")]
        all: bool,

        /// Which issue ids to fetch
        #[structopt(long = "verbose", short = "v")]
        verbose: bool,

        /// Search by summary
        #[structopt(long = "summary", short = "s")]
        summary: Option<String>,

        /// Which issue ids to fetch
        #[structopt(long = "id")]
        id: Option<Vec<String>>,

        /// Which projects to fetch issues for. eg: 'abc'
        #[structopt(long = "project")]
        project: Option<Vec<String>>,

        /// Which issue types to show. eg: 'epic' 'bug' 'story'
        #[structopt(short = "k", long = "kind")]
        kind: Option<Vec<String>>,

        /// Which issue types to exclude
        #[structopt(long = "not-kind")]
        not_kind: Option<Vec<String>>,

        /// Which statuses to include, eg: 'refinement' 'ready' 'validated'
        #[structopt(long = "status")]
        status: Option<Vec<String>>,

        /// Which statuses to exclude
        #[structopt(long = "not-status")]
        not_status: Option<Vec<String>>,

        /// Max number of results to fetch
        #[structopt(long = "max")]
        max: Option<u16>,
    },
}

impl IssuesCmd {
    pub fn match_cmd(
        &self,
        context: Arc<Context>,
    ) -> Result<Vec<Box<dyn AsyncTask>>, failure::Error> {
        match self {
            IssuesCmd::List {
                project,
                max,
                kind,
                not_kind,
                status,
                not_status,
                id,
                verbose,
                summary,
                all,
            } => {
                let mut fetch = IssuesFetch::new(context.clone());

                fetch.kind = kind.clone();
                fetch.not_kind = not_kind.clone();
                fetch.status = status.clone();
                fetch.not_status = not_status.clone();
                fetch.project = project.clone();
                fetch.id = id.clone();
                fetch.summary = summary.clone();
                fetch.all = *all;

                fetch.max = *max;

                let display = IssuesDisplay {
                    resp: fetch.resp.clone(),
                    context,
                    verbose: *verbose,
                    current_user_only: fetch.current_user_only(),
                };
                Ok(vec![Box::new(fetch), Box::new(display)])
            }
        }
    }
}
