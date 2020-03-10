use crate::context::Context;
use crate::http::HttpString;
use async_trait::async_trait;
use reqwest::header::AUTHORIZATION;
use std::sync::Arc;

#[derive(Debug)]
pub struct HttpGet {
    pub url: String,
}

#[async_trait(?Send)]
impl HttpString for HttpGet {
    async fn exec_http(&self, context: Arc<Context>) -> Result<String, failure::Error> {
        let client = reqwest::Client::builder().build()?;

        let res: reqwest::Response = client
            .get(&self.url)
            .header(AUTHORIZATION, context.auth.basic())
            .send()
            .await?;

        let output = match res.error_for_status() {
            Ok(res) => Ok(res.text().await?),
            Err(err) => Err(err),
        }?;

        Ok(output)
    }
}
