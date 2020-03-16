use crate::subcommands::Subcommands;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Args {
    #[structopt(long = "dryrun")]
    pub dry_run: bool,
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,
    #[structopt(subcommand)]
    pub cmd: Subcommands,
}
