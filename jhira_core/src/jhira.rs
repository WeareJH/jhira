use crate::issues::Issues;
use crate::worklog::Worklog;
use structopt::StructOpt;
use std::future::Future;
use crate::task::Task;

#[derive(Debug)]
pub struct Jhira {
    pub args: Vec<String>,
}

#[derive(StructOpt, Debug)]
struct Args {
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
    #[structopt(subcommand)]
    cmd: Subcommands,
}

#[derive(StructOpt, Debug)]
enum Subcommands {
    #[structopt(name = "issues")]
    Issues {
        #[structopt(subcommand)]
        cmd: Issues,
    },
    #[structopt(name = "worklog", alias="wl")]
    Worklog {
        #[structopt(subcommand)]
        cmd: Worklog,
    },
}

#[derive(Debug, Fail)]
pub enum CliError {
    #[fail(display = "invalid args")]
    Invalid,
    #[fail(display = "help shown")]
    HelpShown,
}

impl Jhira {
    pub fn from_args(args: Vec<String>) -> Result<Vec<Box<dyn Task>>, failure::Error> {
        let strs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        let opt: Args = Args::from_iter(strs);
        use Subcommands::*;
        let upcoming = match opt.cmd {
            Issues { cmd } => cmd.match_cmd(),
            Worklog { cmd } => cmd.match_cmd(),
        }?;
        Ok(upcoming)
    }
}
