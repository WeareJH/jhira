#[derive(Debug)]
pub enum Http {
    Get(HttpGet),
}

#[derive(Debug)]
pub struct HttpGet {
    pub url: String,
}

impl HttpGet {
    pub async fn exec(&self) -> Result<(), failure::Error> {
        let _body = reqwest::get(&self.url).await?.text().await?;
        Ok(())
    }
}
