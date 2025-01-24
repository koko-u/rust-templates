use std::net;

use crate::errors::AppResult;
use crate::errors::ToAppResult as _;

#[derive(Debug, Clone, Eq, PartialEq, clap::Parser)]
pub struct EnvArgs {
    #[arg(long, env, default_value = "info")]
    pub rust_log: String,

    #[arg(long, env, default_value = "127.0.0.1")]
    pub host: net::IpAddr,

    #[arg(long, env, default_value_t = 3000)]
    pub port: u16,

    #[arg(long, env)]
    pub database_url: url::Url,

    #[arg(long, env)]
    pub jwt_secret: String,

    #[arg(long, env, value_delimiter = ';')]
    pub allowed_origins: Vec<String>,
}

impl EnvArgs {
    pub fn init() -> AppResult<Self> {
        use clap::Parser;
        Self::try_parse().to_result()
    }
}

