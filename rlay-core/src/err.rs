use thiserror::Error;

#[derive(Error, Debug)]
pub enum RlayError {
    #[error("No root element")]
    NoRoot,

    #[error("Cannot take root because it is still borrowed")]
    RootBorrowed,

    #[error("Cannot take root because the mutex was corrupted")]
    RootCorrupted,

    #[error("Cannot take element because it is still borrowed")]
    ElementBorrowed,

    #[error("Cannot take element because the mutex was corrupted")]
    ElementCorrupted,
}
