use async_trait::async_trait;
use crate::async_task::{Return, AsyncTask, TaskOutput};
use crate::task::TaskSequence;
use std::{env, io};
use std::fs::File;
use std::process::Command;
use ansi_term::Colour::{Green, Red, Blue};

#[derive(Debug, Clone)]
pub struct SelfUpdate {
    pub yes: bool
}

impl From<SelfUpdate> for TaskSequence {
    fn from(self_update: SelfUpdate) -> Self {
        Ok(vec![Box::new(self_update)])
    }
}

#[async_trait(?Send)]
impl AsyncTask for SelfUpdate {
    async fn exec(&self) -> Return {
        let _output = run_self_update(self.yes).await?;
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
    let request_url = String::from("https://api.github.com/repos/wearejh/jhira/releases/latest");
    let response = reqwest::get(&request_url).await?;
    let resp = response.text().await?;

    let jhira_path_cmd = env::current_exe()?;

    let jhira_path = jhira_path_cmd
        .to_str()
        .ok_or(SelfUpdateError::PermissionDenied)?;

    let jhira: JhiraJson = serde_json::from_str(&resp)?;
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
            Blue.paint(jhira_path)
        );
        println!();
        if jhira_path != "/opt/jhira" {
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
                Blue.paint(jhira_path),
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

        let mut response = reqwest::blocking::get("https://www.rust-lang.org/")?;

        let current_path = std::path::PathBuf::from(jhira_path);
        let mut current_dir = File::create(current_path)?;

        println!("Attempting to copy to {}", jhira_path);

        response.copy_to(&mut current_dir)?;

        clear_terminal(is_auto_confirmed);
        let version = Command::new(jhira_path)
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

fn clear_terminal(is_auto_confirmed: bool) {
    if !is_auto_confirmed {
        print!("{}[2J", 27 as char);
    }
}
