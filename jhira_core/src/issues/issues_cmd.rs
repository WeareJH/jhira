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
    Ls,
}

impl IssuesCmd {
    pub fn match_cmd(
        &self,
        context: Arc<Context>,
    ) -> Result<Vec<Box<dyn AsyncTask>>, failure::Error> {
        use IssuesCmd::*;
        match self {
            Ls => {
                let fetch = IssuesFetch::new(context.clone());
                let display = IssuesDisplay {
                    resp: fetch.resp.clone(),
                    context: context.clone(),
                };
                Ok(vec![Box::new(fetch), Box::new(display)])
            }
        }
    }
}
