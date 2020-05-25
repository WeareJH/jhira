#![allow(clippy::large_enum_variant)]

#[allow(unused_macros)]
macro_rules! append_subcommands {
    (as $name:ident, $($command_ident:ident),*) => {
        #[derive(StructOpt, Debug)]
        pub struct Main {
            #[structopt(long="dryrun")]
            dryrun: bool,
            #[structopt(long="config")]
            /// path to a wf2.yml config file
            config: Option<std::path::PathBuf>,
            #[structopt(long="cwd")]
            /// Sets the CWD for all docker commands
            cwd: Option<std::path::PathBuf>,
            #[structopt(subcommand)]
            cmd: Subcommands
        }
        #[derive(StructOpt, Debug)]
        pub enum Subcommands {
            $(
                #[structopt(flatten)]
                $command_ident($command_ident),
            )*
        }
    };
}

#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;

#[macro_use]
extern crate serde_derive;
extern crate serde;

#[macro_use]
extern crate prettytable;

pub mod args;
pub mod async_task;
pub mod auth;
pub mod context;
pub mod epic;
pub mod http;
pub mod http_get;
pub mod http_jql;
pub mod issues;
pub mod jhira;
pub mod jql;
pub mod login;
pub mod self_update;
pub mod subcommands;
pub mod task;
pub mod worklog;
pub mod my_mod;

pub use jhira::*;
