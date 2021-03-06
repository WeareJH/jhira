use crate::context::Context;
use prettytable::format;
use prettytable::Table;

use crate::issues::issue_link::IssueLink;
use crate::issues::jira_issues::JiraIssues;
use crate::issues::priority::PriorityName;
use crate::issues::sort_by::SortBy;
use ansi_term::Colour::{Cyan, Green, Red};

pub struct CompactOpts {
    pub show_assignee: bool,
    pub sort_by: Option<SortBy>,
}

pub fn output_compact(issues: &JiraIssues, context: &Context, opts: CompactOpts) -> String {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);

    let sorted = opts
        .sort_by
        .map(|sort_by| sort_by.sort(issues.issues.clone()))
        .unwrap_or_else(|| issues.issues.clone());

    for v in sorted {
        let sub_task_count = v
            .fields
            .subtasks
            .as_ref()
            .map(|tasks| tasks.len())
            .unwrap_or(0);
        let has_sub_tasks = sub_task_count > 0;
        let row_0 = priority_string(&v.fields.priority.name);
        let row_1 = &v.fields.issuetype.name;
        let row_2 = &v.fields.status.name;
        let row_3 = &v.short_summary();
        let row_4 = format!(
            "{}{}",
            if has_sub_tasks { "• " } else { "" },
            IssueLink::from_context(&context, &v.key)
        );
        let row_5 = if opts.show_assignee {
            v.assignee_name()
        } else {
            None
        };

        if let Some(assignee) = row_5 {
            table.add_row(row![
                row_0,
                Green.bold().paint(row_1),
                Cyan.paint(row_2),
                row_3,
                row_4,
                assignee
            ]);
        } else {
            table.add_row(row![
                row_0,
                Green.bold().paint(row_1),
                Cyan.paint(row_2),
                row_3,
                row_4,
            ]);
        }

        if let Some(ref sub) = v.fields.subtasks {
            let iter = sub.iter().enumerate();
            let count = iter.len();
            for (i, v) in iter {
                let row_0 = priority_string(&v.fields.priority.name);
                let row_1 = &v.fields.issuetype.name;
                let row_2 = &v.fields.status.name;
                let row_3 = &v.short_summary();
                let is_last = i + 1 == count;
                let prefix = if is_last { "└─" } else { "├─" };
                let row_4 = format!("{} {}", prefix, IssueLink::from_context(&context, &v.key));
                let row_5 = if opts.show_assignee {
                    v.assignee_name()
                } else {
                    None
                };

                // todo, subtasks cannot access assignee yet...
                if let Some(assignee) = row_5 {
                    table.add_row(row![
                        row_0,
                        Green.bold().paint(row_1),
                        Cyan.paint(row_2),
                        row_3,
                        row_4,
                        assignee
                    ]);
                } else {
                    table.add_row(row![
                        row_0,
                        Green.bold().paint(row_1),
                        Cyan.paint(row_2),
                        row_3,
                        row_4,
                    ]);
                }
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
    // // let b = include_str!("../../../fixtures/issues-sub-task.json");
    // let b = include_str!("../../../large-list.json");
    // // let b = include_str!("../../../epic.json");
    // let i: JiraIssues = serde_json::from_str(b).expect("Should deserialize");
    // let ctx: Context = Auth::default().into();
    // let next = output_compact(
    //     &i,
    //     &ctx,
    //     CompactOpts {
    //         show_assignee: true,
    //         sort_by: Some(SortBy::Assignee),
    //     },
    // );
    // println!("{}", next);
}

fn priority_string(p: &PriorityName) -> String {
    match p {
        PriorityName::Low => p.to_string(),
        PriorityName::Medium => Green.paint(p.to_string()).to_string(),
        PriorityName::High => Cyan.bold().paint(p.to_string()).to_string(),
        PriorityName::Critical => Red.bold().paint(p.to_string()).to_string(),
    }
}
