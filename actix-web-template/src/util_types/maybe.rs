use std::borrow::Cow;

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Maybe<T> {
    Success(T),
    Error { message: String },
}

impl<T> From<T> for Maybe<T> {
    fn from(value: T) -> Self {
        Self::Success(value)
    }
}

impl<T> Maybe<T> {
    pub fn error_with<'a, S>(message: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self::Error {
            message: message.into().to_string(),
        }
    }
}
