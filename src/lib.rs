use pyo3::prelude::*;

mod error;
mod syntax;
mod domain;
mod validator;

use validator::{EmailValidator as RustEmailValidator, ValidatedEmail as RustValidatedEmail};

/// Validated email result
#[pyclass]
#[derive(Clone)]
struct ValidatedEmail {
    #[pyo3(get)]
    original: String,
    #[pyo3(get)]
    local_part: String,
    #[pyo3(get)]
    domain: String,
    #[pyo3(get)]
    normalized: String,
    #[pyo3(get)]
    ascii_domain: String,
    #[pyo3(get)]
    smtputf8: bool,
}

impl From<RustValidatedEmail> for ValidatedEmail {
    fn from(v: RustValidatedEmail) -> Self {
        Self {
            original: v.original,
            local_part: v.local_part,
            domain: v.domain,
            normalized: v.normalized,
            ascii_domain: v.ascii_domain,
            smtputf8: v.smtputf8,
        }
    }
}

#[pymethods]
impl ValidatedEmail {
    fn __repr__(&self) -> String {
        format!("ValidatedEmail('{}')", self.normalized)
    }
    
    fn __str__(&self) -> String {
        self.normalized.clone()
    }
}

/// Email validator with configurable options
#[pyclass]
#[derive(Clone)]
struct EmailValidator {
    inner: RustEmailValidator,
}

#[pymethods]
impl EmailValidator {
    #[new]
    #[pyo3(signature = (
        allow_smtputf8 = true,
        allow_quoted_local = false,
        allow_domain_literal = false,
        check_deliverability = false
    ))]
    fn new(
        allow_smtputf8: bool,
        allow_quoted_local: bool,
        allow_domain_literal: bool,
        check_deliverability: bool,
    ) -> Self {
        Self {
            inner: RustEmailValidator {
                allow_smtputf8,
                allow_quoted_local,
                allow_domain_literal,
                check_deliverability,
            },
        }
    }
    
    fn validate_email(&self, email: &str) -> PyResult<ValidatedEmail> {
        self.inner.validate(email)
            .map(ValidatedEmail::from)
            .map_err(|e| e.into())
    }
}

/// Validate an email address (convenience function)
#[pyfunction]
#[pyo3(signature = (email, *, check_deliverability = false, allow_smtputf8 = true))]
fn validate_email(
    email: &str,
    check_deliverability: bool,
    allow_smtputf8: bool,
) -> PyResult<ValidatedEmail> {
    let validator = RustEmailValidator {
        allow_smtputf8,
        check_deliverability,
        ..Default::default()
    };
    validator.validate(email)
        .map(ValidatedEmail::from)
        .map_err(|e| e.into())
}

/// Check if email is valid (returns bool, no exception)
#[pyfunction]
#[pyo3(signature = (email, *, allow_smtputf8 = true))]
fn is_valid(email: &str, allow_smtputf8: bool) -> bool {
    // Fast path - inline validation without full error handling
    let email = email.trim();
    
    if email.is_empty() {
        return false;
    }
    
    // Fast @ counting with early exit
    let mut at_pos = None;
    for (i, b) in email.bytes().enumerate() {
        if b == b'@' {
            if at_pos.is_some() {
                return false;  // Second @ - early exit
            }
            at_pos = Some(i);
        }
    }
    
    let Some(at_pos) = at_pos else {
        return false;  // No @ found
    };
    
    let local = &email[..at_pos];
    let domain = &email[at_pos + 1..];
    
    // Fast local checks
    if local.is_empty() || local.len() > 64 {
        return false;
    }
    
    let local_bytes = local.as_bytes();
    if local_bytes[0] == b'.' || local_bytes[local_bytes.len() - 1] == b'.' {
        return false;
    }
    
    // Check for consecutive dots and invalid chars
    let mut prev_dot = false;
    for &b in local_bytes {
        if b == b'.' {
            if prev_dot {
                return false;
            }
            prev_dot = true;
        } else {
            prev_dot = false;
            // Fast byte-level check
            match b {
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' => {}
                b'!' | b'#' | b'$' | b'%' | b'&' | b'\'' | b'*' | b'+' | b'-' | b'/' => {}
                b'=' | b'?' | b'^' | b'_' | b'`' | b'{' | b'|' | b'}' | b'~' => {}
                128..=255 if allow_smtputf8 => {}
                _ => return false,
            }
        }
    }
    
    // Fast domain checks
    if domain.is_empty() || domain.len() > 253 {
        return false;
    }
    
    // Check for at least one dot
    if !domain.contains('.') {
        return false;
    }
    
    // All-numeric check (invalid IP-like)
    let all_numeric = domain.split('.').all(|label| label.bytes().all(|b| b.is_ascii_digit()));
    if all_numeric {
        return false;
    }
    
    // Fast domain validation - skip IDNA for ASCII-only domains
    if domain.is_ascii() {
        // Fast path: validate ASCII domain without IDNA conversion
        for label in domain.split('.') {
            if label.is_empty() || label.len() > 63 {
                return false;
            }
            let bytes = label.as_bytes();
            if bytes[0] == b'-' || bytes[bytes.len()-1] == b'-' {
                return false;
            }
            for &b in bytes {
                if !b.is_ascii_alphanumeric() && b != b'-' {
                    return false;
                }
            }
        }
        true
    } else {
        // Domain validation with IDNA for non-ASCII
        use crate::domain::validate_domain;
        validate_domain(domain).is_ok()
    }
}

/// pyval module
#[pymodule]
fn pyval(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ValidatedEmail>()?;
    m.add_class::<EmailValidator>()?;
    m.add_function(wrap_pyfunction!(validate_email, m)?)?;
    m.add_function(wrap_pyfunction!(is_valid, m)?)?;
    m.add("__version__", "0.1.0")?;
    Ok(())
}
