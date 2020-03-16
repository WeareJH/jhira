use crate::async_task::{AsyncTask, TaskOutput};
use crate::auth::{Auth, AuthError};

use crate::context::Context;
use crate::http::HttpString;
use crate::http_get::HttpGet;
use async_trait::async_trait;
use std::sync::Arc;

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
