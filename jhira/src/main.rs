use jhira_core::async_task::TaskOutput;

///
/// Examples
///
/// ```
/// assert_eq!(2, 1)
/// ```
#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    let args = std::env::args().collect::<Vec<String>>();
    let (opt, tasks) = jhira_core::Jhira::from_args(args.into_iter().collect())?;
    for t in tasks {
        let task_output = if opt.dry_run {
            t.dry_run().await?
        } else {
            t.exec().await?
        };
        handle_output(task_output);
    }
    Ok(())
}

fn handle_output(t: TaskOutput) {
    match t {
        TaskOutput::String(strings) => {
            for s in strings {
                println!("{}", s);
            }
        }
        TaskOutput::Chain(outputs) => {
            for output in outputs {
                handle_output(output);
            }
        }
        TaskOutput::DryRun => {
            // println!("dryrun");
        }
        _ => {
            unimplemented!();
        }
    }
}
