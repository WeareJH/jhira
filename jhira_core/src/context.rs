use crate::auth::Auth;

#[derive(Debug, Default)]
pub struct Context {
    pub auth: Auth,
}
