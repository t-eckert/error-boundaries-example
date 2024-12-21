pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NotFound(String),
    FailedToCreate,
    FailedToRead,
    FailedToUpdate,
    FailedToDelete,
}

impl Error {
    pub fn not_found(msg: &str) -> Self {
        Self::NotFound(msg.to_string())
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
