use jhira_core::async_task::TaskOutput;

use std::sync::Arc;

///
/// Examples
///
/// ```
/// assert_eq!(2, 1)
/// ```
fn main() -> Result<(), failure::Error> {
    env_logger::init();
    let args = std::env::args().collect::<Vec<String>>();

    match async_std::task::block_on(run(args)) {
        Ok(..) => {
            // do nothing if all good!
        }
        Err(e) => eprintln!("{}", e.to_string()),
    };

    Ok(())
}

async fn run(args: Vec<String>) -> Result<(), failure::Error> {
    let output = jhira_core::Jhira::from_args(args.into_iter())?;
    let context_arc = Arc::new(output.context);
    for t in output.tasks {
        let task_output = if output.args.dry_run {
            t.dry_run().await?
        } else {
            t.exec(context_arc.clone()).await?
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
        TaskOutput::Done => {
            // println!("done");
        }
    }
}
