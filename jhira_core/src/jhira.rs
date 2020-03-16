use crate::args::Args;
use crate::async_task::AsyncTask;
use crate::auth::Auth;
use crate::context::Context;
use crate::task::TaskSequence;
use structopt::StructOpt;

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
