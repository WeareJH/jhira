use crate::context::Context;
use std::fmt;

pub struct IssueLink(pub String);

impl IssueLink {
    pub fn from_context(ctx: &Context, key: &str) -> IssueLink {
        IssueLink(format!(
            "https://{}.atlassian.net/browse/{}",
            ctx.auth.domain, key
        ))
    }
    pub fn http_get(ctx: &Context, key: &str) -> IssueLink {
        IssueLink(format!(
            "https://{}.atlassian.net/rest/api/3/issue/{}",
            ctx.auth.domain, key
        ))
    }
}

impl fmt::Display for IssueLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<IssueLink> for String {
    fn from(link: IssueLink) -> Self {
        format!("{}", link)
    }
}
