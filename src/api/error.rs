use axum::http::StatusCode;

pub type Result<T> = core::result::Result<T, Error>;

pub type Error = axum::http::StatusCode;

impl From<crate::bank::Error> for Error {
    fn from(err: crate::bank::Error) -> Self {
        match err {
            crate::bank::Error::AccountHasInsufficientBalance => StatusCode::BAD_REQUEST.into(),
            crate::bank::Error::UserDoesNotOwnAccount => StatusCode::FORBIDDEN.into(),
            crate::bank::Error::PasswordIsNotSufficientlyComplex => StatusCode::BAD_REQUEST.into(),
            crate::bank::Error::FailedToCreateUser(_) => StatusCode::INTERNAL_SERVER_ERROR.into(),
            crate::bank::Error::StoreError(_) => StatusCode::INTERNAL_SERVER_ERROR.into(),
        }
    }
}
