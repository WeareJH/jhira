use crate::args::Args;
use crate::async_task::AsyncTask;
use crate::auth::Auth;
use crate::context::Context;
use crate::task::TaskSequence;
use structopt::{StructOpt, clap, clap::AppSettings};
use async_trait::async_trait;

#[derive(Debug)]
pub struct Jhira {
    pub args: Vec<String>,
}

pub struct JhiraOutput {
    pub args: Args,
    pub tasks: Vec<Box<dyn AsyncTask>>,
    pub context: Context,
}

impl Jhira {
    pub fn from_args(args: impl Iterator<Item = String>) -> Result<JhiraOutput, failure::Error> {
        let c = args.collect::<Vec<String>>();
        let opt: Args = Args::from_iter(&c);
        let opt2: Args = Args::from_iter(&c);
        let task_seq: TaskSequence = opt.cmd.into();
        let tasks = task_seq?;

        // do any tasks require authentication?
        let requires_auth = tasks.iter().any(|x| x.authenticated());

        // get a context based on whether or not a task
        // requires authentication
        let context = if requires_auth {
            let ctx: Context = Auth::from_file()?.into();
            ctx
        } else {
            Context::default()
        };

        Ok(JhiraOutput {
            context,
            tasks,
            args: opt2,
        })
    }
}

#[async_trait(?Send)]
pub trait Exec: std::fmt::Debug {
    async fn exec(&self, ctx: &Context) -> Result<(), failure::Error>;
}

#[test]
fn test_from_iter_safe() {

    // let input = &["prog", "--cwd", "/users/shane", "--help"];
    use crate::my_mod::*;
    let ctx = Context::default();
    let input = vec!["prog", "--help"];
    let invalid = &["prog", "hell"];
    let a = crate::my_mod::M2::get_cli(input);

    match a {
        Ok(Main { cmd, .. }) => {
            let inner = cmd.select();
            dbg!(inner);
            // let out = inner.exec(&ctx);
            // dbg!(cmd.select());
        },
        Err(clap::Error {
                kind: clap::ErrorKind::HelpDisplayed,
                message,
                info
            }) => println!("help->{}", message),
        Err(clap::Error {
                kind: clap::ErrorKind::VersionDisplayed,
                message,
                info,
            }) => println!("help->{}", message),
        Err(other) => eprintln!("not help or version->\n{:#?}", other),
        _ => unimplemented!()
    };
}
