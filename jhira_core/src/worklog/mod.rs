use crate::task::TaskSequence;

use crate::worklog::worklog_create::WorklogCreate;
use crate::worklog::worklog_ls::WorklogLs;
use structopt::clap::AppSettings;
use structopt::StructOpt;

pub mod worklog_create;
pub mod worklog_ls;

mod date_range;

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
pub enum WorklogCmd {
    /// List your worklogs
    #[structopt(name = "ls")]
    List(WorklogLs),
    /// Create a worklog
    #[structopt(name = "create")]
    Create(WorklogCreate),
}

impl From<WorklogCmd> for TaskSequence {
    fn from(worklog_cmd: WorklogCmd) -> Self {
        match worklog_cmd {
            WorklogCmd::List(ls) => ls.into(),
            WorklogCmd::Create(create) => create.into(),
        }
    }
}
