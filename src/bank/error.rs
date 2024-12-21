pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    AccountHasInsufficientBalance,
    UserDoesNotOwnAccount,
    PasswordIsNotSufficientlyComplex,
    FailedToCreateUser(String),
    StoreError(crate::store::Error),
}

impl Error {
    pub fn failed_to_create_user(msg: &str) -> Self {
        Self::FailedToCreateUser(msg.to_string())
    }
}

impl From<crate::auth::Error> for Error {
    fn from(err: crate::auth::Error) -> Self {
        match err {
            crate::auth::Error::PasswordTooShort => Error::PasswordIsNotSufficientlyComplex,
            crate::auth::Error::PasswordDoesNotMatch => Error::UserDoesNotOwnAccount,
            crate::auth::Error::UnableToHashPassword(_) => {
                Error::failed_to_create_user("Unable to hash password")
            }
        }
    }
}

impl From<crate::store::Error> for Error {
    fn from(err: crate::store::Error) -> Self {
        match err {
            _ => Error::StoreError(err),
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
