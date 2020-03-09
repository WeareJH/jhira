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
        #[structopt(short = "k", long = "kind")]
        kind: Option<Vec<String>>,

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
            IssuesCmd::List { kind, max } => {
                let mut fetch = IssuesFetch::new(context.clone());
                fetch.kind = kind.clone();
                fetch.max = max.clone();
                let display = IssuesDisplay {
                    resp: fetch.resp.clone(),
                    context,
                };
                Ok(vec![Box::new(fetch), Box::new(display)])
            }
        }
    }
}
