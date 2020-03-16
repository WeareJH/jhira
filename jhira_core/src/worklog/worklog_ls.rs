use crate::task::TaskSequence;
use crate::worklog::date_range::DateRange;
use structopt::StructOpt;

///
/// List your worklogs
///
#[derive(Debug, StructOpt, Clone)]
pub struct WorklogLs {
    /// which day/days to fetch, eg: 'today', 'yesterday', '3' or '2019-10-29'
    #[structopt(default_value = "1")]
    range: DateRange,
    /// Filters to apply
    #[structopt(short)]
    filters: Option<String>,
}

impl From<WorklogLs> for TaskSequence {
    fn from(_ls: WorklogLs) -> Self {
        unimplemented!();
    }
}
