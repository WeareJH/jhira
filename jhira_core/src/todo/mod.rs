use crate::task::TaskSequence;

use crate::issues::ls::IssuesLs;
use crate::issues::sort_by::SortBy;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone, Default)]
pub struct Todo {
    /// Which projects to fetch issues for. eg: 'abc'
    #[structopt(long = "project")]
    pub project: Option<Vec<String>>,

    /// Which order to show the results in
    #[structopt(long = "sort")]
    pub sort: Option<SortBy>,
}

impl From<Todo> for TaskSequence {
    fn from(todo: Todo) -> Self {
        let issue_cmd = IssuesLs {
            active_sprint: true,
            project: todo.project.clone(),
            sort: todo.sort.clone(),
            status: Some(vec![
                "refinement".to_string(),
                "ready".to_string(),
                "work in progress".to_string(),
            ]),
            ..IssuesLs::default()
        };
        Ok(vec![Box::new(issue_cmd)])
    }
}
