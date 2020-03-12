use crate::context::Context;
use crate::epic::Epic;
use crate::issues::issues_display::IssueLink;
use crate::issues::output_compact::{output_compact as issues_output, CompactOpts};
use ansi_term::Colour::{Green, Yellow};
use prettytable::Table;

pub fn output_compact(epic: &Epic, context: &Context) -> String {
    // format!("epic has {} assigned tasks", epic.issues.issues.len())
    let list = issues_output(&epic.issues, context, CompactOpts{ show_assignee: true });

    // let h1 = Green.bold().paint("Epic");
    let h1 = Green.bold().paint(epic.issue.summary());

    let h2 = Yellow
        .bold()
        .paint(format!("({} tasks)", epic.issues.issues.len()));
    let h3 = IssueLink::from_context(&context, &epic.issue.key);
    // let h3 = epic.issue.summary();
    let heading = format!("{} {} - {}", h1, h2, h3);
    let mut t = Table::new();
    t.add_row(row![heading]);
    t.add_row(row![list]);
    t.to_string()
}
