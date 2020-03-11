use crate::context::Context;
use crate::issues::issues_display::IssueLink;
use crate::issues::issues_types::JiraIssues;
use prettytable::format;
use prettytable::Table;

use ansi_term::Colour::{Cyan, Green};

pub fn output_compact(issues: &JiraIssues, context: &Context) -> String {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);

    for v in &issues.issues {
        let sub_task_count = v
            .fields
            .subtasks
            .as_ref()
            .map(|tasks| tasks.len())
            .unwrap_or(0);
        let has_sub_tasks = sub_task_count > 0;
        let row_1 = &v.fields.issuetype.name;
        let row_2 = &v.fields.status.name;
        let row_3 = format!(
            "{}{}",
            if has_sub_tasks { "• " } else { "" },
            IssueLink::from_context(&context, &v.key)
        );

        table.add_row(row![
            Green.bold().paint(row_1),
            Cyan.paint(row_2),
            row_3,
            // row_4
        ]);

        if let Some(ref sub) = v.fields.subtasks {
            let iter = sub.iter().enumerate();
            let count = iter.len();
            for (i, v) in iter {
                let row_1 = &v.fields.issuetype.name;
                let row_2 = &v.fields.status.name;
                let is_last = i + 1 == count;
                let prefix = if is_last { "└─" } else { "├─" };
                let row_3 = format!("{} {}", prefix, IssueLink::from_context(&context, &v.key));
                table.add_row(row![
                    Green.bold().paint(row_1),
                    Cyan.paint(row_2),
                    row_3,
                    // row_4
                ]);
            }
        }
    }

    let issue_table = table.to_string();
    let summary_table = summary_table(&issues);
    format!("{}\n{}", issue_table, summary_table)
}

fn summary_table(issues: &JiraIssues) -> String {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
    table.add_row(row!["total", issues.total]);
    table.add_row(row!["shown", issues.issues.len()]);
    table.to_string()
}

#[test]
fn test_output_compact() {
    // use crate::auth::Auth;
    // let b = include_str!("../../../fixtures/issues-sub-task.json");
    // let b = include_str!("../../../epic.json");
    // let i: JiraIssues = serde_json::from_str(b).expect("Should deserialize");
    // let ctx: Context = Auth::default().into();
    // let next = output_compact(&i, &ctx);
    // println!("{}", next);
}
