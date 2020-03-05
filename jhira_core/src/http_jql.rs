use crate::context::Context;
use crate::http::HttpString;
use async_trait::async_trait;
use reqwest::header::AUTHORIZATION;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct HttpJql {
    pub jql: String,
    pub max_results: u16,
}

impl HttpJql {
    pub fn new(jql: impl Into<String>) -> HttpJql {
        HttpJql {
            jql: jql.into(),
            max_results: 200u16,
        }
    }
    pub fn max_results(&mut self, max_results: u16) -> &mut Self {
        self.max_results = max_results;
        self
    }
    pub fn build(&mut self) -> HttpJql {
        HttpJql { ..self.clone() }
    }
}

#[async_trait(?Send)]
impl HttpString for HttpJql {
    async fn exec(&self, context: Arc<Context>) -> Result<String, failure::Error> {
        let client = reqwest::Client::new();

        let url = format!(
            "https://{}.atlassian.net/rest/api/3/search",
            context.auth.domain
        );

        let j = serde_json::json!({
            "jql": self.jql.clone(),
            "maxResults": self.max_results.to_string(),
            "validateQuery": true,
        });

        let res = client
            .post(&url)
            .json(&j)
            .header(AUTHORIZATION, context.auth.basic())
            .send()
            .await?;

        let output = match res.error_for_status() {
            Ok(res) => Ok(res.text().await?),
            Err(err) => {
                Err(err)
            },
        }?;

        Ok(output)
    }
}
