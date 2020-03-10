use crate::context::Context;
use crate::http::HttpString;
use async_trait::async_trait;
use reqwest::header::AUTHORIZATION;

use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct HttpJql {
    pub jql: String,
    pub max_results: u16,
    pub fields: Option<Vec<String>>,
}

#[derive(Debug, Fail)]
pub enum HttpJqlError {
    #[fail(display = "You didn't provide a query")]
    Empty,
}

impl FromStr for HttpJql {
    type Err = HttpJqlError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(HttpJqlError::Empty);
        }
        Ok(HttpJql::new(s))
    }
}

impl HttpJql {
    pub fn new(jql: impl Into<String>) -> HttpJql {
        HttpJql {
            jql: jql.into(),
            max_results: 200u16,
            fields: None,
        }
    }
    pub fn max_results(&mut self, max_results: u16) -> &mut Self {
        self.max_results = max_results;
        self
    }
    pub fn max_opt(&mut self, max_results: Option<u16>) -> &mut Self {
        if let Some(max) = max_results {
            self.max_results = max;
        }
        self
    }
    pub fn fields_opt(&mut self, fields: Option<Vec<String>>) -> &mut Self {
        if let Some(fields) = fields {
            self.fields = Some(fields);
        }
        self
    }
    pub fn build(&mut self) -> HttpJql {
        HttpJql { ..self.clone() }
    }
}

#[async_trait(?Send)]
impl HttpString for HttpJql {
    async fn exec_http(&self, context: Arc<Context>) -> Result<String, failure::Error> {
        let client = reqwest::Client::new();

        let url = format!(
            "https://{}.atlassian.net/rest/api/3/search",
            context.auth.domain
        );

        let j = serde_json::json!({
            "jql": self.jql.clone(),
            "maxResults": self.max_results.to_string(),
            "validateQuery": true,
            "fields": self.fields.clone()
        });

        let res = client
            .post(&url)
            .json(&j)
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
