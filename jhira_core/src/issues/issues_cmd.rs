use structopt::clap::AppSettings;
use structopt::StructOpt;

use crate::issues::ls::IssuesLs;

use crate::task::TaskSequence;

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
pub enum IssuesCmd {
    /// List issues assigned to you
    #[structopt(name = "ls")]
    List(IssuesLs),
}

impl From<IssuesCmd> for TaskSequence {
    fn from(issues_cmd: IssuesCmd) -> Self {
        match issues_cmd {
            IssuesCmd::List(ls) => ls.into(),
        }
    }
}
