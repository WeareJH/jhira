use crate::async_task::AsyncTask;
use structopt::clap::AppSettings;
use structopt::StructOpt;

use crate::context::Context;
use crate::issues::Issues;

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
pub enum IssuesCmd {
    /// List issues assigned to you
    #[structopt(name = "ls")]
    Ls,
}

impl IssuesCmd {
    pub fn match_cmd(&self, context: &Context) -> Result<Vec<Box<dyn AsyncTask>>, failure::Error> {
        use IssuesCmd::*;
        match self {
            Ls => {
                let a = context.auth.clone();
                let issue_url = format!("https://{}.atlassian.net/rest/api/2/myself", a.domain);
                let t1 = Issues {
                    url: issue_url,
                    auth: a,
                };
                Ok(vec![Box::new(t1)])
            }
        }
    }
}
