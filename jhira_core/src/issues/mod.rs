use structopt::clap::AppSettings;
use structopt::StructOpt;
use crate::task::{Task, TaskOutput};
use crate::http::{HttpGet, Http};
use std::convert::{TryInto, TryFrom};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
pub enum Issues {
    /// List issues assigned to you
    #[structopt(name = "ls")]
    Ls,
}

impl Issues {
    pub fn match_cmd(&self) -> Result<Vec<Box<dyn Task>>, failure::Error> {
        use Issues::*;
        match self {
            Ls => {
                return Ok(vec![Box::new(Issue{url: String::from("https://httpbin.org/get")})])
            }
        }
    }
}

#[derive(Debug)]
struct Issue {
    url: String
}

#[async_trait(?Send)]
impl Task for Issue {
    async fn dry_run(&self) -> Result<(), failure::Error> {
        println!("{:#?}", self);
        Ok(())
    }
}

