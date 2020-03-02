use crate::auth::Auth;

#[derive(Debug)]
pub struct Context {
    pub auth: Auth,
}
