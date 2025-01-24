use error_stack::ResultExt;

use crate::errors::AppError;
use crate::errors::AppResult;

pub trait ToAppResult {
    type Ok;
    fn to_result(self) -> AppResult<Self::Ok>;
}

impl<T, C> ToAppResult for Result<T, C>
where
    C: error_stack::Context,
{
    type Ok = T;

    fn to_result(self) -> AppResult<T> {
        self.change_context(AppError)
    }
}
