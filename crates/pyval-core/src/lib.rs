//! pyval-core - Core email validation library
//! 
//! This crate provides the core email validation logic
//! that can be used by any language wrapper.

pub mod domain;
pub mod error;
pub mod fastpath;
pub mod lazy;
pub mod lookup;
pub mod simd;
pub mod syntax;
pub mod validator;

pub use error::EmailError;
pub use validator::{EmailValidator, ValidatedEmail};

/// Quick validation function
pub fn is_valid(email: &str) -> bool {
    lazy::ZeroCopyValidator::validate_no_alloc(email)
}

/// Full validation with options
pub fn validate(email: &str, allow_smtputf8: bool) -> Result<ValidatedEmail, EmailError> {
    let validator = EmailValidator {
        allow_smtputf8,
        ..Default::default()
    };
    validator.validate(email)
}
