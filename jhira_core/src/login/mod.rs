use crate::async_task::{AsyncTask, TaskOutput};
use crate::auth::{Auth, AuthError};
use structopt::StructOpt;

use crate::context::Context;
use crate::http::HttpString;
use crate::http_get::HttpGet;
use crate::task::TaskSequence;
use async_trait::async_trait;
use std::sync::Arc;

#[derive(StructOpt, Debug, Clone)]
pub struct LoginCmd {
    #[structopt(long = "domain")]
    pub domain: String,

    #[structopt(long = "api")]
    pub api: String,

    #[structopt(long = "email")]
    pub email: String,
}

impl From<LoginCmd> for TaskSequence {
    fn from(login_cmd: LoginCmd) -> Self {
        let auth = Auth {
            domain: login_cmd.domain,
            api: login_cmd.api,
            email: login_cmd.email,
        };

        let a1 = auth.clone();
        let a2 = auth;

        // verify via HTTP call
        let verify = LoginVerify { auth: a1 };

        // write to disk
        let write = LoginWrite { auth: a2 };

        Ok(vec![Box::new(verify), Box::new(write)])
    }
}

#[derive(Debug)]
pub struct LoginVerify {
    pub auth: Auth,
}

#[async_trait(?Send)]
impl AsyncTask for LoginVerify {
    async fn exec(&self, _ctx: Arc<Context>) -> Result<TaskOutput, failure::Error> {
        let ctx: Context = self.auth.clone().into();
        let http = HttpGet {
            url: format!(
                "https://{}.atlassian.net/rest/api/3/myself",
                self.auth.domain
            ),
        };
        let _resp = http
            .exec_http(Arc::new(ctx))
            .await
            .map_err(|e| AuthError::CouldNotVerify(e.to_string()))?;
        debug!("Auth all good");
        Ok(TaskOutput::Done)
    }
    fn authenticated(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct LoginWrite {
    pub auth: Auth,
}

#[async_trait(?Send)]
impl AsyncTask for LoginWrite {
    async fn exec(&self, _ctx: Arc<Context>) -> Result<TaskOutput, failure::Error> {
        Auth::write_to_file(&self.auth)?;
        Ok(TaskOutput::String(vec![String::from(
            "Login verified and saved :)",
        )]))
    }
    fn authenticated(&self) -> bool {
        false
    }
}
