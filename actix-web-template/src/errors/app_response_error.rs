use crate::errors::AppError;

#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
    Default,
    derive_more::Display,
    derive_more::Error,
    actix_web_error::Json,
)]
#[display(
    "Internal Server Error{0}",
    message.as_ref().map(|message| format!(": {message}")).unwrap_or_default()
)]
pub struct AppResponseError {
    message: Option<String>,
}

impl From<error_stack::Report<AppError>> for AppResponseError {
    fn from(report: error_stack::Report<AppError>) -> Self {
        log::error!("{report:?}");

        Self {
            message: format!("{report:#}").into(),
        }
    }
}
