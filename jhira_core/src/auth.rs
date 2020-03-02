use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Auth {
    pub domain: String,
    pub email: String,
    pub api: String,
}

#[derive(Fail, Debug)]
pub enum AuthError {
    #[fail(display = "Could not read")]
    CouldNotRead,
}

impl Auth {
    pub fn basic(&self) -> String {
        format!(
            "Basic {}",
            base64::encode(&format!("{}:{}", self.email, self.api))
        )
    }

    pub fn output_file() -> Result<PathBuf, failure::Error> {
        dirs::home_dir()
            .ok_or_else(|| AuthError::CouldNotRead.into())
            .map(|home| home.join(".wf2").join("jira.json"))
    }

    pub fn from_file() -> Result<Auth, failure::Error> {
        let pb = Auth::output_file()?;
        let bytes = fs::read(pb)?;
        let output = serde_json::from_slice::<Auth>(&bytes)?;
        Ok(output)
    }
}

#[test]
fn test_auth() -> Result<(), failure::Error> {
    let a = Auth::from_file();
    dbg!(a?.basic());
    Ok(())
}
