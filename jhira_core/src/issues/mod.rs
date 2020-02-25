use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
pub enum Issues {
    /// List issues assigned to you
    #[structopt(name = "ls")]
    Ls,
}
