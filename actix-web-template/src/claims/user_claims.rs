use std::borrow::Borrow;

use better_default::Default as BetterDefault;
use time::ext::NumericalDuration as _;

use crate::models::User;
use crate::models::UserId;

#[derive(Debug, Clone, Eq, PartialEq, BetterDefault, serde::Serialize, serde::Deserialize)]
pub struct UserClaims {
    #[serde(serialize_with = "as_sec")]
    #[serde(deserialize_with = "from_sec")]
    #[serde(rename = "exp")]
    #[default(time::OffsetDateTime::now_utc() + time::Duration::seconds(15))]
    pub expiration: time::OffsetDateTime,
    pub id: UserId,
    pub username: String,
}

impl<U> From<U> for UserClaims
where
    U: Borrow<User>,
{
    fn from(borrowed_user: U) -> Self {
        Self {
            id: borrowed_user.borrow().id,
            username: borrowed_user.borrow().username.clone(),
            ..Default::default()
        }
    }
}

fn as_sec<S>(expiration: &time::OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let delta = *expiration - time::OffsetDateTime::UNIX_EPOCH;
    serializer.serialize_u64(delta.whole_seconds() as u64)
}

fn from_sec<'de, D>(deserializer: D) -> Result<time::OffsetDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let sec: f64 = serde::Deserialize::deserialize(deserializer)?;
    Ok(time::OffsetDateTime::UNIX_EPOCH + sec.seconds())
}
