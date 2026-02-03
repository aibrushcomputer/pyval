use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use thiserror::Error;

#[derive(Error, Debug, Clone, Copy)]
pub enum EmailError {
    #[error("The email address is empty")]
    Empty,

    #[error("The email address is missing an '@' sign")]
    MissingAt,

    #[error("The email address has too many '@' signs")]
    MultipleAt,

    #[error("The email address cannot start with a period")]
    LeadingDot,

    #[error("The email address cannot end with a period")]
    TrailingDot,

    #[error("The email address cannot have two periods in a row")]
    ConsecutiveDots,

    #[error("The local part (before '@') is too long")]
    LocalPartTooLong,

    #[error("The domain is too long")]
    DomainTooLong,

    #[error("The domain is not valid")]
    InvalidDomain,

    #[error("Invalid character in the email address")]
    InvalidCharacter,

    #[error("The email address is not valid")]
    Generic,
}

impl From<EmailError> for PyErr {
    fn from(err: EmailError) -> PyErr {
        PyValueError::new_err(err.to_string())
    }
}
