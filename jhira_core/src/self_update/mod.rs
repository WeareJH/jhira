use crate::async_task::{AsyncTask, Return, TaskOutput};
use crate::context::Context;
use crate::task::TaskSequence;
use ansi_term::Colour::{Blue, Green, Red};
use async_trait::async_trait;
use reqwest::header::USER_AGENT;
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use std::{env, io};
use structopt::StructOpt;

const GITHUB_URL: &str = "https://api.github.com/repos/wearejh/jhira/releases/latest";

#[derive(Debug, StructOpt, Clone)]
pub struct SelfUpdateCmd {
    /// Accept all prompts and update automatically
    #[structopt(long = "yes", short = "y")]
    pub yes: bool,
}

impl From<SelfUpdateCmd> for TaskSequence {
    fn from(self_update: SelfUpdateCmd) -> Self {
        Ok(vec![Box::new(self_update)])
    }
}

#[async_trait(?Send)]
impl AsyncTask for SelfUpdateCmd {
    async fn exec(&self, _ctx: Arc<Context>) -> Return {
        run_self_update(self.yes).await?;
        Ok(TaskOutput::Done)
    }
}

#[derive(Debug, Fail)]
enum SelfUpdateError {
    #[fail(display = "Cannot read path to executable")]
    PermissionDenied,
    #[fail(display = "Assets contained no items")]
    NoItems,
}

#[derive(Serialize, Deserialize, Debug)]
struct JhiraJson {
    assets: Vec<JhiraJsonAsset>,
    name: String,
    tag_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct JhiraJsonAsset {
    browser_download_url: String,
    size: i32,
    name: String,
}

pub async fn run_self_update(is_auto_confirmed: bool) -> Result<(), failure::Error> {
    let jhira = github_data().await?;

    let jhira_path = env::current_exe().map_err(|_| SelfUpdateError::PermissionDenied)?;

    let url = jhira
        .assets
        .get(0)
        .map(|asset| asset.browser_download_url.clone())
        .ok_or(SelfUpdateError::NoItems)?;

    let name = jhira
        .assets
        .get(0)
        .map(|asset| asset.name.clone())
        .ok_or(SelfUpdateError::NoItems)?;

    let size = jhira
        .assets
        .get(0)
        .map(|asset| asset.size)
        .ok_or(SelfUpdateError::NoItems)?;

    clear_terminal(is_auto_confirmed);
    let mut ok_to_proceed: bool = false;
    if !is_auto_confirmed {
        println!("{}", Green.paint("=====[Jhira Self Updater]====="));
        println!();
        println!("File name   : {}", name);
        println!("Description : {}", jhira.name);
        println!("Url         : {}", url);
        println!("Version     : {}", jhira.tag_name);
        println!("Size        : {}kb", size / 1024);
        println!();
        println!(
            "Current jhira directory is reported as: {}",
            Blue.paint(jhira_path.to_string_lossy())
        );
        println!();
        if jhira_path != PathBuf::from("/opt/jhira") {
            println!(
                "{}",
                Red.paint("Warning! Working directory is NOT the standard directory expected.")
            );
            println!("{}", Red.paint("Expected directory to be /opt/jhira"));
            println!(
                "{}",
                Red.paint("You can proceed with the update, but at your own risk!")
            );
            println!();
            println!(
                "{} {} {}",
                Blue.paint("If you wish to fix this, exit out of this app and run 'sudo mv"),
                Blue.paint(jhira_path.to_string_lossy()),
                Blue.paint("/opt/jhira'")
            );
            println!(
                "{}",
                Blue.paint("More info here: https://github.com/WeareJH/jhira#manual")
            );
        } else {
            println!("{}", Green.paint("Working directory is ok!"));
        }
        println!();

        loop {
            println!("Ok to proceed? (y/n)");
            let mut user_input = String::new();

            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");

            if let Some('\n') = user_input.chars().next_back() {
                user_input.pop();
            }
            if let Some('\r') = user_input.chars().next_back() {
                user_input.pop();
            }
            if user_input == "y" || user_input == "yes" {
                ok_to_proceed = true;
                break;
            } else if user_input == "n" || user_input == "no" {
                break;
            } else {
                clear_terminal(is_auto_confirmed);
                println!("Unrecognised input: '{}'", user_input);
            }
        }
    } else {
        println!("Auto confirm flag passed, continuing...");
        ok_to_proceed = true;
    }

    if ok_to_proceed {
        clear_terminal(is_auto_confirmed);

        println!("Starting update...");

        download_binary(url, &jhira_path)?;

        clear_terminal(is_auto_confirmed);

        let version = Command::new(&jhira_path)
            .arg("-V")
            .output()
            .expect("failed to execute process");

        println!("Success!");
        println!(
            "You updated to {}",
            std::str::from_utf8(&version.stdout).unwrap()
        );
    } else {
        clear_terminal(is_auto_confirmed);
        println!("Aborted update");
    }

    Ok(())
}

///
/// Use the Github api to determine the latest version
///
async fn github_data() -> Result<JhiraJson, failure::Error> {
    let request_url = String::from(GITHUB_URL);

    let client = reqwest::Client::new();

    let response = client
        .get(&request_url)
        .header(USER_AGENT, "curl") // gh needs a user-agent
        .send()
        .await?;

    let resp = match response.error_for_status() {
        Ok(res) => Ok(res.text().await?),
        Err(err) => {
            println!("{:?}", err);
            Err(err)
        }
    }?;

    debug!("got jsoon response === {}", &resp);

    let output: JhiraJson = serde_json::from_str(&resp)?;

    Ok(output)
}

///
/// Actually download and save the binary
///
fn download_binary(
    url: impl Into<String>,
    output: impl Into<PathBuf>,
) -> Result<(), failure::Error> {
    let pb = output.into();
    let mut response = reqwest::blocking::get(&url.into())?;
    let mut ouput_file = File::create(&pb)?;
    println!("Attempting to copy to {}", pb.display());
    response.copy_to(&mut ouput_file)?;
    Ok(())
}

fn clear_terminal(is_auto_confirmed: bool) {
    if !is_auto_confirmed {
        print!("{}[2J", 27 as char);
    }
}
