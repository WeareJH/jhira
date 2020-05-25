use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug, Clone)]
pub enum Up {
    /// Bring up the containers
    Up {
        #[structopt(long = "api")]
        api: Option<String>,
    }
}

#[derive(StructOpt, Debug, Clone)]
pub enum Down {
    /// Take down the containers
    Down {
        #[structopt(long = "api")]
        api: Option<String>,
    }
}

#[derive(StructOpt, Debug, Clone)]
pub enum DbImport {
    /// Import a DB
    DbImport {
        #[structopt(parse(from_os_str))]
        file: PathBuf,
        #[structopt(long, short)]
        force: bool,
    }
}

append_subcommands!(
    as Main,
    Up,
    Down,
    DbImport
);
