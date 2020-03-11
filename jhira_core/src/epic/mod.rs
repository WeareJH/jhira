use crate::context::Context;
use crate::epic::epic_display::EpicDisplay;
use crate::epic::epic_fetch::EpicFetch;
use crate::issues::issues_types::{JiraIssue, JiraIssues};
use crate::task::TaskSequence;
use std::sync::{Arc, Mutex};

pub mod epic_display;
pub mod epic_fetch;
pub mod output_compact;

#[derive(Debug)]
pub struct Epic {
    issue: JiraIssue,
    issues: JiraIssues,
}

#[derive(Debug, Clone)]
pub struct EpicCmd {
    id: String,
    context: Arc<Context>,
}

impl EpicCmd {
    pub fn new(id: impl Into<String>, context: Arc<Context>) -> Self {
        Self {
            id: id.into(),
            context,
        }
    }
}

impl From<EpicCmd> for TaskSequence {
    fn from(epic_cmd: EpicCmd) -> Self {
        let epic: Arc<Mutex<Option<Epic>>> = Arc::new(Mutex::new(None));
        let fetch = EpicFetch {
            context: epic_cmd.context.clone(),
            id: epic_cmd.id.clone(),
            epic: epic.clone(),
        };
        let display = EpicDisplay {
            context: epic_cmd.context,
            id: epic_cmd.id.clone(),
            epic,
        };
        Ok(vec![Box::new(fetch), Box::new(display)])
    }
}
