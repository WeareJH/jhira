use tokio::prelude::*;
use jhira_core::task::{Task, TaskOutput};
use futures::{future, stream, StreamExt};
use jhira_core::http::Http;

///
/// Examples
///
/// ```
/// assert_eq!(2, 1)
/// ```
#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    let args = std::env::args().collect::<Vec<String>>();
    let tasks = jhira_core::Jhira::from_args(args.into_iter().collect())?;
    for t in tasks {
        t.dry_run().await;
    };
    Ok(())
}
