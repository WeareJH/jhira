use structopt::{StructOpt, clap};
use std::path::PathBuf;
use async_trait::async_trait;
use crate::Exec;
use crate::context::Context;

#[derive(StructOpt, Debug, Clone)]
pub struct Up {
    #[structopt(short, long)]
    attached: bool,
    #[structopt(short, long)]
    clean: bool,
    #[structopt(short, long)]
    build: bool,
    #[structopt(short, long)]
    sync: Option<Vec<PathBuf>>,
}

#[async_trait(?Send)]
impl Exec for Up {
    async fn exec(&self, ctx: &Context) -> Result<(), failure::Error> {
        Ok(())
    }
}

#[derive(StructOpt, Debug, Clone)]
pub struct Down {
    /// Take down the containers
    #[structopt(long = "api")]
    api: Option<String>,
}

#[async_trait(?Send)]
impl Exec for Down {
    async fn exec(&self, ctx: &Context) -> Result<(), failure::Error> {
        Ok(())
    }
}

/// Import a DB
#[derive(StructOpt, Debug, Clone)]
pub struct DbImport {
    #[structopt(parse(from_os_str))]
    file: PathBuf,
    #[structopt(long, short)]
    force: bool,
}

#[async_trait(?Send)]
impl Exec for DbImport {
    async fn exec(&self, ctx: &Context) -> Result<(), failure::Error> {
        Ok(())
    }
}

append_subcommands!(
    as Main,
    Up,
    Down,
    DbImport
);
