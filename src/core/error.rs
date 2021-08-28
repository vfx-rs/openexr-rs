use openexr_sys as sys;

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("Arguments were out of range")]
    OutOfRange,
    #[error("Base error: {0}")]
    Base(String),
    #[error("Input error: {0}")]
    Input(String),
    #[error("Logic error: {0}")]
    LogicError(String),
    #[error("Overflow: {0}")]
    Overflow(String),
    #[error("Out of memory: {0}")]
    OutOfMemory(String),
    #[error("Invalid type: {0}")]
    InvalidType(String),
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    #[error("I/O: {0}")]
    Io(String),
    #[error("No preview image: {0}")]
    NoPreviewImage(String),
}

impl From<sys::Error> for Error {
    fn from(e: sys::Error) -> Error {
        use sys::Error as E;
        match e {
            E::IexBase(s) => Error::Base(s),
            E::IexInput(s) => Error::Input(s),
            E::IexLogicError(s) => Error::LogicError(s),
            E::IexOverflow(s) => Error::Overflow(s),
            E::IexOutOfMemory(s) => Error::OutOfMemory(s),
            E::IexInvalidType(s) => Error::InvalidType(s),
            E::IexInvalidArgument(s) => Error::InvalidArgument(s),
            E::IexIo(s) => Error::Io(s),
            E::IexNoPreviewImage(s) => Error::NoPreviewImage(s),
        }
    }
}
