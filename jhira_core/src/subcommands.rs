use crate::epic::EpicCmd;
use crate::issues::issues_cmd::IssuesCmd;
use crate::jql::JqlCmd;
use crate::login::LoginCmd;
use crate::self_update::SelfUpdateCmd;
use crate::task::TaskSequence;
use crate::worklog::WorklogCmd;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Subcommands {
    #[structopt(name = "login")]
    Login(LoginCmd),
    #[structopt(name = "issues", alias = "i")]
    Issues {
        #[structopt(subcommand)]
        cmd: IssuesCmd,
    },
    #[structopt(name = "epic")]
    Epic(EpicCmd),
    #[structopt(name = "jql")]
    Jql(JqlCmd),
    #[structopt(name = "worklog", alias = "wl")]
    Worklog {
        #[structopt(subcommand)]
        cmd: WorklogCmd,
    },
    /// Update to the latest version
    #[structopt(name = "self-update", alias = "update")]
    SelfUpdate(SelfUpdateCmd),
}

impl From<Subcommands> for TaskSequence {
    fn from(subcommand: Subcommands) -> Self {
        use Subcommands::*;
        match subcommand {
            Issues { cmd } => cmd.into(),
            Worklog { cmd } => cmd.into(),
            Epic(cmd) => cmd.into(),
            Jql(cmd) => cmd.into(),
            Login(cmd) => cmd.into(),
            SelfUpdate(self_update) => self_update.into(),
        }
    }
}
