use crate::worklog::Worklog;
use structopt::StructOpt;

use crate::async_task::AsyncTask;
use crate::auth::Auth;
use crate::context::Context;
use crate::issues::cmd::IssuesCmd;

#[derive(Debug)]
pub struct Jhira {
    pub args: Vec<String>,
}

#[derive(StructOpt, Debug)]
pub struct Args {
    #[structopt(long = "dryrun")]
    pub dry_run: bool,
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,
    #[structopt(subcommand)]
    pub cmd: Subcommands,
}

#[derive(StructOpt, Debug)]
pub enum Subcommands {
    #[structopt(name = "issues")]
    Issues {
        #[structopt(subcommand)]
        cmd: IssuesCmd,
    },
    #[structopt(name = "worklog", alias = "wl")]
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
    pub fn from_args(args: Vec<String>) -> Result<(Args, Vec<Box<dyn AsyncTask>>), failure::Error> {
        let strs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        let opt: Args = Args::from_iter(&strs);
        let opt2: Args = Args::from_iter(&strs);
        let a = Auth::from_file()?;
        let context = Context { auth: a };
        use Subcommands::*;
        let upcoming = match opt.cmd {
            Issues { cmd } => cmd.match_cmd(&context),
            Worklog { cmd } => cmd.match_cmd(&context),
        }?;
        Ok((opt2, upcoming))
    }
}
