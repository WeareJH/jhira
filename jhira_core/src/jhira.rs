use crate::worklog::Worklog;
use structopt::StructOpt;

use crate::async_task::AsyncTask;
use crate::auth::Auth;
use crate::context::Context;
use crate::http_jql::HttpJql;
use crate::issues::issues_cmd::IssuesCmd;
use std::sync::Arc;

use crate::jql::JqlCmd;

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
    #[structopt(name = "login")]
    Login {
        #[structopt(long = "domain")]
        domain: String,

        #[structopt(long = "api")]
        api: String,

        #[structopt(long = "email")]
        email: String,
    },
    #[structopt(name = "issues")]
    Issues {
        #[structopt(subcommand)]
        cmd: IssuesCmd,
    },
    #[structopt(name = "jql")]
    Jql {
        jql: HttpJql,
        #[structopt(long = "max")]
        max: Option<u16>,
        #[structopt(long = "fields")]
        fields: Option<Vec<String>>,
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
    pub fn from_args(
        args: impl Iterator<Item = String>,
    ) -> Result<(Args, Vec<Box<dyn AsyncTask>>), failure::Error> {
        let c = args.collect::<Vec<String>>();
        let opt: Args = Args::from_iter(&c);
        let opt2: Args = Args::from_iter(&c);
        use Subcommands::*;
        let upcoming = match opt.cmd {
            Issues { cmd } => {
                let context: Context = Auth::from_file()?.into();
                cmd.match_cmd(Arc::new(context))
            }
            Worklog { cmd } => {
                let context: Context = Auth::from_file()?.into();
                cmd.match_cmd(Arc::new(context))
            }
            Jql {
                mut jql,
                max,
                fields,
            } => {
                let context: Context = Auth::from_file()?.into();
                let jql_http = jql.max_opt(max).fields_opt(fields).build();
                JqlCmd::new(jql_http, Arc::new(context)).into()
            }
            Login { api, domain, email } => {
                let auth = Auth { api, domain, email };
                auth.login()
            }
        }?;
        Ok((opt2, upcoming))
    }
}

#[test]
fn test_jhira() -> Result<(), failure::Error> {
    let args = vec!["jira", "issues", "ls", "--kind", "bug", "epic"]
        .into_iter()
        .map(String::from);
    let (args, _tasks) = Jhira::from_args(args)?;
    dbg!(args);
    Ok(())
}
