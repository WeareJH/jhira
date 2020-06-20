#![allow(clippy::large_enum_variant)]

#[allow(unused_macros)]
macro_rules! append_subcommands {
    (as $name:ident, $($command_ident:ident),*) => {
        #[derive(StructOpt, Debug)]
        pub struct Main {
            #[structopt(long="dryrun")]
            pub dryrun: bool,
            #[structopt(long="config")]
            /// path to a wf2.yml config file
            pub config: Option<std::path::PathBuf>,
            #[structopt(long="cwd")]
            /// Sets the CWD for all docker commands
            pub cwd: Option<std::path::PathBuf>,
            #[structopt(subcommand)]
            pub cmd: Subcommands
        }
        #[derive(StructOpt, Debug)]
        #[structopt(setting = clap::AppSettings::InferSubcommands)]
        pub enum Subcommands {
            $(
                $command_ident($command_ident),
            )*
        }
        impl Subcommands {
            pub fn select(self) -> Box<dyn Exec> {
                match self {
                    $(
                        Self::$command_ident(inner) => Box::new(inner),
                    )*
                }
            }
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
