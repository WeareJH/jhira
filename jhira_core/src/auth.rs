use std::fs;
use std::path::PathBuf;

use crate::context::Context;
use crate::task::TaskSequence;

use crate::login::{LoginVerify, LoginWrite};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Auth {
    pub domain: String,
    pub email: String,
    pub api: String,
}

impl Default for Auth {
    fn default() -> Self {
        Auth {
            domain: "example".into(),
            api: "123456".into(),
            email: "shane@exampe.com".into(),
        }
    }
}

#[derive(Fail, Debug)]
pub enum AuthError {
    #[fail(display = "Could not read")]
    CouldNotRead,
    #[fail(display = "Could not verify, {:?}", _0)]
    CouldNotVerify(String),
    #[fail(display = "Could not create .jhira directory")]
    CouldNotCreate,
    #[fail(display = "You are not logged in, please run `jhira login` first")]
    NotLoggedIn,
    #[fail(
        display = "Login details could not be deserialized, please run `jhira login` again.\n{}",
        _0
    )]
    LoginCorrupt(serde_json::Error),
}

impl Auth {
    pub fn login(&self) -> TaskSequence {
        let a1 = self.clone();
        let a2 = self.clone();

        // verify via HTTP call
        let verify = LoginVerify { auth: a1 };

        // write to disk
        let write = LoginWrite { auth: a2 };

        Ok(vec![Box::new(verify), Box::new(write)])
    }
    pub fn basic(&self) -> String {
        format!(
            "Basic {}",
            base64::encode(&format!("{}:{}", self.email, self.api))
        )
    }

    pub fn output_file() -> Result<PathBuf, failure::Error> {
        dirs::home_dir()
            .ok_or_else(|| AuthError::CouldNotRead.into())
            .map(|home| home.join(".jhira").join("jhira.json"))
    }

    pub fn from_file() -> Result<Auth, failure::Error> {
        let pb = Auth::output_file()?;
        let bytes = fs::read(pb).map_err(|_| AuthError::NotLoggedIn)?;
        let output = serde_json::from_slice::<Auth>(&bytes).map_err(AuthError::LoginCorrupt)?;
        Ok(output)
    }

    pub fn write_to_file(auth: &Auth) -> Result<(), failure::Error> {
        let pb = Auth::output_file()?;
        let as_string = serde_json::to_string(&auth)?;
        let dir = pb.parent().ok_or(AuthError::CouldNotCreate)?;
        fs::create_dir_all(dir)?;
        fs::write(pb, as_string)?;
        Ok(())
    }
}

impl From<Auth> for Context {
    fn from(auth: Auth) -> Self {
        Context { auth }
    }
}

#[test]
fn test_auth() -> Result<(), failure::Error> {
    let a = Auth::from_file();
    dbg!(a?.basic());
    Ok(())
}
