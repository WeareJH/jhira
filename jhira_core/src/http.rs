use crate::context::Context;
use async_trait::async_trait;

use crate::http_get::HttpGet;
use crate::http_jql::HttpJql;
use std::sync::Arc;

#[derive(Debug)]
pub enum Http {
    Get(HttpGet),
    Jql(HttpJql),
}

#[async_trait(?Send)]
pub trait HttpString {
    async fn exec_http(&self, context: Arc<Context>) -> Result<String, failure::Error>;
}
