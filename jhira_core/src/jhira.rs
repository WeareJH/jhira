use crate::worklog::Worklog;
use structopt::StructOpt;

use crate::async_task::AsyncTask;
use crate::auth::Auth;
use crate::context::Context;
use crate::http_jql::HttpJql;
use crate::issues::issues_cmd::IssuesCmd;

use crate::epic::EpicCmd;
use crate::jql::JqlCmd;

use crate::self_update::SelfUpdate;

#[derive(Debug)]
pub struct Jhira {
    pub args: Vec<String>,
}

pub struct JhiraOutput {
    pub args: Args,
    pub tasks: Vec<Box<dyn AsyncTask>>,
    pub context: Context,
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
    #[structopt(name = "epic")]
    Epic(EpicCmd),
    #[structopt(name = "jql")]
    Jql {
        jql: HttpJql,
        #[structopt(long = "json")]
        json: bool,
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
    /// Update to the latest version
    #[structopt(name = "self-update", alias = "update")]
    SelfUpdate(SelfUpdate),
}

#[derive(Debug, Fail)]
pub enum CliError {
    #[fail(display = "invalid args")]
    Invalid,
    #[fail(display = "help shown")]
    HelpShown,
}

impl Jhira {
    pub fn from_args(args: impl Iterator<Item = String>) -> Result<JhiraOutput, failure::Error> {
        let c = args.collect::<Vec<String>>();
        let opt: Args = Args::from_iter(&c);
        let opt2: Args = Args::from_iter(&c);
        use Subcommands::*;
        let tasks: Vec<Box<dyn AsyncTask>> = match opt.cmd {
            Issues { cmd } => cmd.into(),
            Worklog { cmd } => cmd.match_cmd(),
            Epic(cmd) => cmd.into(),
            Jql {
                mut jql,
                max,
                fields,
                json,
            } => {
                let jql_http = jql.max_opt(max).fields_opt(fields).build();
                JqlCmd::new(jql_http, json).into()
            }
            Login { api, domain, email } => {
                let auth = Auth { api, domain, email };
                auth.login()
            }
            SelfUpdate(self_update) => self_update.into(),
        }?;
        let requires_auth = tasks.iter().any(|x| x.authenticated());
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

#[tokio::main]
#[test]
async fn test_jhira() -> Result<(), failure::Error> {
    let args = vec!["jira", "issues", "ls", "--kind", "bug", "epic"]
        .into_iter()
        .map(String::from);
    let output = Jhira::from_args(args)?;
    for t in output.tasks {
        let _ = t.dry_run().await?;
    }
    Ok(())
}
