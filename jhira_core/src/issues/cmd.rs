use crate::async_task::AsyncTask;
use structopt::clap::AppSettings;
use structopt::StructOpt;

use crate::issues::Issues;

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
pub enum IssuesCmd {
    /// List issues assigned to you
    #[structopt(name = "ls")]
    Ls,
}

impl IssuesCmd {
    pub fn match_cmd(&self) -> Result<Vec<Box<dyn AsyncTask>>, failure::Error> {
        use IssuesCmd::*;
        match self {
            Ls => {
                let t1 = Issues {
                    url: String::from("https://httpbin.org/get"),
                };
                Ok(vec![Box::new(t1)])
            }
        }
    }
}
