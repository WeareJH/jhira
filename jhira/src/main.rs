use tokio::prelude::*;

///
/// Examples
///
/// ```
/// assert_eq!(2, 1)
/// ```
#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    let args = std::env::args().collect::<Vec<String>>();
    let _jhira = jhira_core::Jhira::from_args(args.into_iter().collect()).await;
    Ok(())
}
