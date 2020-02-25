use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
pub enum Worklog {
    /// List your worklogs
    #[structopt(name = "ls")]
    Ls,
    /// Create a worklog
    #[structopt(name = "create")]
    Create,
}
