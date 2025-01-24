use sqlx::postgres::PgConnectOptions;
use sqlx::ConnectOptions as _;

use crate::errors::AppResult;
use crate::errors::ToAppResult;
use crate::EnvArgs;

#[derive(Debug, Clone, derive_more::Deref, derive_more::DerefMut)]
pub struct Db(sqlx::PgPool);

impl Db {
    pub async fn init(env_args: &EnvArgs) -> AppResult<Self> {
        let opts = PgConnectOptions::from_url(&env_args.database_url).to_result()?;
        let pool = sqlx::PgPool::connect_with(opts).await.to_result()?;
        Ok(Self(pool))
    }
}


