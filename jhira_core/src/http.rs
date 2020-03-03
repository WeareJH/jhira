use crate::auth::Auth;
use reqwest::header::AUTHORIZATION;

#[derive(Debug)]
pub enum Http {
    Get(HttpGet),
}

#[derive(Debug)]
pub struct HttpGet {
    pub url: String,
}

impl HttpGet {
    pub async fn exec(&self, auth: &Auth) -> Result<String, failure::Error> {
        let client = reqwest::Client::builder().build()?;

        let res: reqwest::Response = client
            .get(&self.url)
            .header(AUTHORIZATION, auth.basic())
            .send()
            .await?;

        let output = match res.error_for_status() {
            Ok(res) => {
                Ok(res.text().await?)
            }
            Err(err) => Err(err)
        }?;

        Ok(output)
    }
}
