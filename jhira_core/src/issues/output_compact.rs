use crate::context::Context;
use crate::issues::issues_display::IssueLink;
use crate::issues::issues_types::JiraIssues;
use prettytable::format;
use prettytable::Table;

pub fn output_compact(issues: JiraIssues, context: &Context) -> String {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);

    for v in &issues.issues {
        let row_1 = &v.fields.issuetype.name;
        let row_2 = &v.fields.status.name;
        let row_3 = IssueLink::from_context(&context, &v.key);

        table.add_row(row![
            row_1, row_2, row_3,
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
                    row_1, row_2, row_3,
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
