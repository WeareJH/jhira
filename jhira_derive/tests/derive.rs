use structopt::{StructOpt, clap};
use std::path::PathBuf;

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

#[test]
fn test_derive() {

}
