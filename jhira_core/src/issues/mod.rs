use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
pub enum Issues {
    /// List issues assigned to you
    #[structopt(name = "ls")]
    Ls,
}

impl Issues {
    pub fn match_cmd(&self) -> Result<(), failure::Error> {
        use Issues::*;
        match self {
            Ls => {
                println!("issues ls!");
            }
        };
        Ok(())
    }
}
