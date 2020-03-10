use crate::context::Context;
use prettytable::format;

use crate::issues::issues_types::JiraIssues;
use prettytable::{Cell, Row, Table};

use crate::issues::issues_display::IssueLink;
use ansi_term::Colour::{Cyan, Green};

pub fn output_verbose(issues: JiraIssues, context: &Context) -> String {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
    for (_i, issue) in issues.issues.iter().enumerate() {
        let mut t1 = Table::new();
        t1.set_format(*format::consts::FORMAT_CLEAN);
        let summary = issue
            .fields
            .summary
            .clone()
            .unwrap_or(String::from("No summary"));
        let title = format!("{} {}", Green.bold().paint(&issue.key), Cyan.paint(summary));
        let issue_link = IssueLink::from_context(&context, &issue.key);
        t1.set_titles(Row::new(vec![Cell::new(&title)]));
        t1.add_row(row![issue_link]);
        t1.add_row(row![format!(
            "{}, {}",
            issue.fields.issuetype.name, issue.fields.status.name
        )]);
        table.add_row(row![t1]);
        table.add_empty_row();
    }
    table.to_string()
}

#[test]
fn test_output_verbose() {
    let b = include_str!("../../../fixtures/issues-sub-task.json");
    let i: JiraIssues = serde_json::from_str(b).expect("Should deserialize");
    let ctx: Context = Auth::default().into();
    let next = output_verbose(i, &ctx);
    println!("{}", next);
}
