use crate::task::TaskSequence;
use structopt::StructOpt;

///
/// Create a worklog
///
#[derive(Debug, StructOpt, Clone)]
pub struct WorklogCreate {
    /// The issue ID
    issue: String,
    /// The time spent
    spent: String,
    /// The date to log, format: YYYY-MM-DD, eg: 2020-01-24
    #[structopt(short)]
    date: Option<String>,
    /// The time for the start of the log, format: HH:MM:SS, eg: 08:30:00
    #[structopt(short)]
    time: Option<String>,
    /// A comment for the log, eg: 'overtime'
    #[structopt(short)]
    comment: Option<String>,
}

impl From<WorklogCreate> for TaskSequence {
    fn from(_ls: WorklogCreate) -> Self {
        unimplemented!();
    }
}
