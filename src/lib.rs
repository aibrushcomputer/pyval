use pyo3::prelude::*;

mod error;
mod syntax;
mod domain;
mod validator;
mod lookup;
mod fastpath;
mod lazy;
mod simd;

#[allow(dead_code)]
mod prefetch;
#[allow(dead_code)]
mod jit;
#[allow(dead_code)]
mod vectorized;
#[allow(dead_code)]
mod approximate;

use validator::{EmailValidator as RustEmailValidator, ValidatedEmail as RustValidatedEmail};
use simd::PortableSimd;
use lazy::ZeroCopyValidator;

/// Validated email result - lazy version
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

/// Ultra-fast check using lookup tables and SIMD
#[inline(always)]
pub fn is_valid_fast(email: &str, allow_smtputf8: bool) -> bool {
    // Try SIMD path for long ASCII strings
    if email.len() >= 32 {
        if let Some(result) = PortableSimd::validate_email_fast(email) {
            return result;
        }
    }
    
    // Try zero-allocation path for medium strings
    if let Some(result) = fastpath::fast_ascii_email_check(email) {
        return result;
    }
    
    // Fallback to detailed validation
    is_valid_detailed(email, allow_smtputf8)
}

/// Detailed validation with full checks
#[inline(always)]
pub fn is_valid_detailed(email: &str, allow_smtputf8: bool) -> bool {
    let email = email.trim();
    
    if email.is_empty() {
        return false;
    }
    
    // Fast @ counting with early exit
    let bytes = email.as_bytes();
    let mut at_pos = None;
    for (i, &b) in bytes.iter().enumerate() {
        if b == b'@' {
            if at_pos.is_some() {
                return false;
            }
            at_pos = Some(i);
        }
    }
    
    let Some(at_pos) = at_pos else {
        return false;
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
    
    // Use lookup table for character validation
    let mut prev_dot = false;
    for &b in local_bytes {
        if b == b'.' {
            if prev_dot {
                return false;
            }
            prev_dot = true;
        } else {
            prev_dot = false;
            if !lookup::is_valid_local_byte_fast(b) {
                if !(allow_smtputf8 && b >= 128) {
                    return false;
                }
            }
        }
    }
    
    // Fast domain checks
    if domain.is_empty() || domain.len() > 253 {
        return false;
    }
    
    if !domain.contains('.') {
        return false;
    }
    
    // All-numeric check
    let all_numeric = domain.split('.').all(|label| label.bytes().all(|b| b.is_ascii_digit()));
    if all_numeric {
        return false;
    }
    
    // Fast domain validation
    if domain.is_ascii() {
        for label in domain.split('.') {
            if label.is_empty() || label.len() > 63 {
                return false;
            }
            let bytes = label.as_bytes();
            if bytes[0] == b'-' || bytes[bytes.len()-1] == b'-' {
                return false;
            }
            for &b in bytes {
                if !lookup::is_valid_domain_byte_fast(b) {
                    return false;
                }
            }
        }
        true
    } else {
        use crate::domain::validate_domain;
        validate_domain(domain).is_ok()
    }
}

/// Check if email is valid (returns bool, no exception)
#[pyfunction]
#[pyo3(signature = (email, *, allow_smtputf8 = true))]
#[inline(always)]
fn is_valid(email: &str, allow_smtputf8: bool) -> bool {
    is_valid_fast(email, allow_smtputf8)
}

/// Ultra-fast is_valid using zero-copy validation
#[pyfunction]
#[pyo3(signature = (email, *))]
#[inline(always)]
fn is_valid_ultra(email: &str) -> bool {
    ZeroCopyValidator::validate_no_alloc(email)
}

/// Batch validate multiple emails (for high throughput)
#[pyfunction]
#[pyo3(signature = (emails, *, allow_smtputf8 = true))]
fn batch_is_valid(emails: Vec<String>, allow_smtputf8: bool) -> Vec<bool> {
    // Use prefetching for large batches
    if emails.len() > 16 {
        let email_refs: Vec<&str> = emails.iter().map(|s| s.as_str()).collect();
        prefetch::pipelined_validation(&email_refs)
    } else {
        emails.iter()
            .map(|e| is_valid_fast(e, allow_smtputf8))
            .collect()
    }
}

/// pyval module
#[pymodule]
fn pyval(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ValidatedEmail>()?;
    m.add_class::<EmailValidator>()?;
    m.add_function(wrap_pyfunction!(validate_email, m)?)?;
    m.add_function(wrap_pyfunction!(is_valid, m)?)?;
    m.add_function(wrap_pyfunction!(is_valid_ultra, m)?)?;
    m.add_function(wrap_pyfunction!(batch_is_valid, m)?)?;
    m.add("__version__", "0.2.0")?;
    Ok(())
}
