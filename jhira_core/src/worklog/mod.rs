use structopt::clap::AppSettings;
use structopt::StructOpt;
use crate::worklog::date_range::DateRange;
use crate::task::Task;
use async_trait::async_trait;

mod date_range;

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
pub enum Worklog {
    /// List your worklogs
    #[structopt(name = "ls")]
    Ls {
        /// which day/days to fetch, eg: 'today', 'yesterday', '3' or '2019-10-29'
        #[structopt(default_value = "1", )]
        range: DateRange,
        /// Filters to apply
        #[structopt(short)]
        filters: Option<String>,
    },
    /// Create a worklog
    #[structopt(name = "create")]
    Create {
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
        comment: Option<String>
    },
}

impl Worklog {
    pub fn match_cmd(&self) -> Result<Vec<Box<dyn Task>>, failure::Error> {
        use Worklog::*;
        match self {
            Ls { range, filters } => {
                println!("worklog ls {:#?}", range);
            },
            Create { .. } => {
                println!("worklog create!");
            }
        };
        Ok(vec![Box::new(Wl)])
    }
}


struct Wl;

#[async_trait(?Send)]
impl Task for Wl {}
