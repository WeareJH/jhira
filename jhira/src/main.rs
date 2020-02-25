///
/// Examples
///
/// ```
/// assert_eq!(2, 1)
/// ```
fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let _jhira = jhira_core::Jhira::from_args(args.into_iter().collect());
}
