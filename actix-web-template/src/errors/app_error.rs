#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, derive_more::Display)]
#[display("App Error")]
pub struct AppError;

impl error_stack::Context for AppError {}

pub type AppResult<T> = error_stack::Result<T, AppError>;