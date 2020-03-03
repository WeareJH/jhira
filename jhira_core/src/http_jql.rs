use crate::context::Context;
use crate::http::HttpString;
use async_trait::async_trait;
use reqwest::header::AUTHORIZATION;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct HttpJql {
    pub jql: String,
    pub max_results: usize,
}

impl HttpJql {
    pub fn new(jql: impl Into<String>) -> HttpJql {
        HttpJql {
            jql: jql.into(),
            max_results: 200,
        }
    }
}

#[async_trait(?Send)]
impl HttpString for HttpJql {
    async fn exec(&self, context: Arc<Context>) -> Result<String, failure::Error> {
        let client = reqwest::Client::new();

        let url = format!(
            "https://{}.atlassian.net/rest/api/2/search?fields=-issuetype",
            context.auth.domain
        );

        let mut map = HashMap::new();
        map.insert("jql", self.jql.clone());
        map.insert("maxResults", self.max_results.to_string());

        let res = client
            .post(&url)
            .json(&map)
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
