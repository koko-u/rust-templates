use crate::Jwt;

pub struct AppState<DB> {
    pub db: DB,
    pub jwt: Jwt,
}