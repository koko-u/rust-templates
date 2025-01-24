#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Default,
    derive_more::Display,
    derive_more::Constructor,
    derive_more::From,
    derive_more::FromStr,
    derive_more::Into,
    serde::Serialize,
    serde::Deserialize,
    sqlx::Type,
)]
#[display("{0}", _0)]
#[sqlx(transparent)]
pub struct UserId(i64);
