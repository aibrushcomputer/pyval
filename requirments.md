# Project: pyval - Blazingly Fast Email Validator in Rust for Python

## Mission

Build a high-performance email validation library in Rust with Python bindings (pyo3). Achieve **100-1000x performance improvement** over `python-email-validator` while maintaining feature parity and RFC compliance.

Work autonomously and continuously until the goal is reached. Do not stop. Do not ask for permission. Iterate relentlessly.

---

### Project structure to create
```
/home/aibrush/pyval/
├── Cargo.toml
├── pyproject.toml
├── src/
│   ├── lib.rs              # Main pyo3 module entry
│   ├── validator.rs        # Core validation logic
│   ├── syntax.rs           # RFC 5322/6531 syntax parsing
│   ├── domain.rs           # Domain validation & IDN support
│   ├── local_part.rs       # Local part validation
│   ├── normalizer.rs       # Email normalization
│   ├── error.rs            # Error types
│   └── deliverability.rs   # DNS/MX checks (optional feature)
├── tests/
│   ├── test_api_compat.py  # Must match python-email-validator behavior
│   ├── test_rfc_compliance.py  # RFC test vectors
│   └── test_performance.py # Benchmark comparisons
├── benches/
│   └── benchmarks.rs       # Rust-native benchmarks
└── test_data/
    └── emails.txt          # Test email corpus
```

---

## Phase 1: Gather Test Data and Baseline

### Step 1.1: Create test email corpus
```bash
mkdir -p /home/aibrush/pyval/test_data
```

Create `/home/aibrush/pyval/test_data/emails.py`:
```python
#!/usr/bin/env python3
"""Test email corpus for validation."""

# Valid emails (should pass)
VALID_EMAILS = [
    "simple@example.com",
    "very.common@example.com",
    "disposable.style.email.with+symbol@example.com",
    "other.email-with-hyphen@example.com",
    "fully-qualified-domain@example.com",
    "user.name+tag+sorting@example.com",
    "x@example.com",  # one-letter local-part
    "example-indeed@strange-example.com",
    "test/test@test.com",  # slashes ok in local part
    "admin@mailserver1",  # local domain name with no TLD
    "example@s.example",  # short domain
    "mailhost!username@example.org",  # bangified host route
    "user%example.com@example.org",  # % escaped mail route
    "user-@example.org",  # hyphen at end of local part
    "postmaster@[123.123.123.123]",  # IP address literal
    "postmaster@[IPv6:2001:db8::1]",  # IPv6 literal
    # Internationalized
    "用户@例子.广告",  # Chinese
    "उपयोगकर्ता@उदाहरण.कॉम",  # Hindi
    "юзер@екзампл.ком",  # Cyrillic
    "θσερ@εχαμπλε.ψομ",  # Greek
    "Pelstransen@telefonlansen.dk",
]

# Invalid emails (should fail)
INVALID_EMAILS = [
    "",  # empty
    "plainaddress",  # missing @
    "@missinglocal.com",  # missing local part
    "missing@.com",  # missing domain name
    "missing.domain@",  # missing domain
    "two@@ats.com",  # two @ signs
    ".leadingdot@example.com",  # leading dot
    "trailingdot.@example.com",  # trailing dot in local
    "double..dot@example.com",  # consecutive dots
    "email@example..com",  # consecutive dots in domain
    "email@-example.com",  # leading hyphen in domain
    "email@example-.com",  # trailing hyphen in domain
    "email@111.222.333.44444",  # invalid IP
    "email@example.com (Joe Smith)",  # text after email
    "email@example",  # missing TLD (configurable)
    '"(),:;<>[\\]@example.com',  # special chars unquoted
    'just"not"right@example.com',  # quotes must be dot-separated
    'this is"notallowed@example.com',  # spaces unquoted
    'this still"not\\allowed@example.com',  # backslash unquoted
    # Too long
    "a" * 65 + "@example.com",  # local part > 64 chars
    "test@" + "a" * 256 + ".com",  # domain too long
]

# Edge cases
EDGE_CASES = [
    ('"john..doe"@example.com', True),  # quoted consecutive dots
    ('" "@example.com', True),  # space in quotes
    ('"very.unusual.@.unusual.com"@example.com', True),  # special chars quoted
    ('"\\"@example.com', True),  # escaped quote
]

# For bulk benchmarking
def generate_bulk_emails(n=100000):
    """Generate n email addresses for benchmarking."""
    import random
    import string
    domains = ["gmail.com", "yahoo.com", "hotmail.com", "example.com", "test.org"]
    emails = []
    for i in range(n):
        local = ''.join(random.choices(string.ascii_lowercase + string.digits, k=random.randint(5, 20)))
        domain = random.choice(domains)
        emails.append(f"{local}@{domain}")
    return emails
```

### Step 1.2: Create baseline benchmark script
Create `/home/aibrush/pyval/baseline_benchmark.py`:
```python
#!/usr/bin/env python3
"""Baseline benchmarks for python-email-validator."""
import time
import statistics
import json
from pathlib import Path

# Reference implementation
from email_validator import validate_email as py_validate_email, EmailNotValidError

# Our test data
from test_data.emails import VALID_EMAILS, INVALID_EMAILS, generate_bulk_emails

RESULTS_FILE = Path("/home/aibrush/pyval/baseline_results.json")
ITERATIONS = 1000
BULK_SIZE = 10000

def benchmark(func, iterations=ITERATIONS):
    """Run function multiple times and return stats."""
    times = []
    for _ in range(iterations):
        start = time.perf_counter_ns()
        func()
        end = time.perf_counter_ns()
        times.append(end - start)
    return {
        "mean_ns": statistics.mean(times),
        "median_ns": statistics.median(times),
        "min_ns": min(times),
        "max_ns": max(times),
        "stdev_ns": statistics.stdev(times) if len(times) > 1 else 0,
        "iterations": iterations
    }

def run_benchmarks():
    results = {}
    
    # Single email validation
    test_email = "user.name+tag@example.com"
    print(f"Benchmarking single email: {test_email}")
    results["single_valid"] = benchmark(
        lambda: py_validate_email(test_email, check_deliverability=False)
    )
    
    # Invalid email (should be fast to reject)
    invalid_email = "invalid@@email"
    print(f"Benchmarking invalid email: {invalid_email}")
    def validate_invalid():
        try:
            py_validate_email(invalid_email, check_deliverability=False)
        except EmailNotValidError:
            pass
    results["single_invalid"] = benchmark(validate_invalid)
    
    # Batch validation
    bulk_emails = generate_bulk_emails(BULK_SIZE)
    print(f"Benchmarking batch of {BULK_SIZE} emails...")
    
    def validate_batch():
        for email in bulk_emails[:100]:  # 100 per iteration
            try:
                py_validate_email(email, check_deliverability=False)
            except EmailNotValidError:
                pass
    
    results["batch_100"] = benchmark(validate_batch, iterations=100)
    
    # Internationalized email
    idn_email = "用户@例子.广告"
    print(f"Benchmarking IDN email: {idn_email}")
    results["idn_email"] = benchmark(
        lambda: py_validate_email(idn_email, check_deliverability=False)
    )
    
    # With normalization
    print("Benchmarking with normalization...")
    results["with_normalization"] = benchmark(
        lambda: py_validate_email("  User.Name@EXAMPLE.COM  ", check_deliverability=False)
    )
    
    # Save results
    with open(RESULTS_FILE, "w") as f:
        json.dump(results, f, indent=2)
    
    print(f"\nBaseline results saved to {RESULTS_FILE}")
    print("\nSummary (mean times):")
    for name, stats in results.items():
        print(f"  {name}: {stats['mean_ns']/1000:.2f} µs")
    
    return results

if __name__ == "__main__":
    run_benchmarks()
```

### Step 1.3: Run baseline and record results
```bash
cd /home/aibrush/pyval
python baseline_benchmark.py
cat baseline_results.json
```

---

## Phase 2: Implement Core Rust Library

### Cargo.toml
```toml
[package]
name = "pyval"
version = "0.1.0"
edition = "2021"
description = "Blazingly fast email validator for Python"
license = "MIT"

[lib]
name = "pyval"
crate-type = ["cdylib"]

[dependencies]
pyo3 = { version = "0.22", features = ["extension-module"] }
idna = "1.0"           # Internationalized domain names
unicode-normalization = "0.1"
once_cell = "1.19"     # Lazy statics
regex = "1.10"         # For complex patterns (use sparingly)
thiserror = "1.0"

# Optional DNS checking
trust-dns-resolver = { version = "0.23", optional = true }

[features]
default = []
deliverability = ["trust-dns-resolver"]

[profile.release]
lto = true
codegen-units = 1
panic = "abort"
strip = true
opt-level = 3
```

### Implementation priority:

1. **Syntax validation** - RFC 5322 local-part and domain parsing
2. **Domain validation** - IDNA 2008 support
3. **Normalization** - Lowercase, unicode NFC
4. **Error messages** - User-friendly, match python-email-validator
5. **Deliverability** - DNS/MX checks (optional feature)

### Core implementation

Create `/home/aibrush/pyval/src/error.rs`:
```rust
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
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
    
    #[error("The domain '{0}' is not valid: {1}")]
    InvalidDomain(String, String),
    
    #[error("Invalid character '{0}' in the email address")]
    InvalidCharacter(char),
    
    #[error("The email address is not valid: {0}")]
    Generic(String),
}

impl From<EmailError> for PyErr {
    fn from(err: EmailError) -> PyErr {
        PyValueError::new_err(err.to_string())
    }
}
```

Create `/home/aibrush/pyval/src/syntax.rs`:
```rust
//! RFC 5322 / RFC 6531 email syntax validation

use crate::error::EmailError;

/// Validates the local part (before @) of an email address
pub fn validate_local_part(local: &str, allow_smtputf8: bool) -> Result<(), EmailError> {
    if local.is_empty() {
        return Err(EmailError::Empty);
    }
    
    if local.len() > 64 {
        return Err(EmailError::LocalPartTooLong);
    }
    
    if local.starts_with('.') {
        return Err(EmailError::LeadingDot);
    }
    
    if local.ends_with('.') {
        return Err(EmailError::TrailingDot);
    }
    
    if local.contains("..") {
        return Err(EmailError::ConsecutiveDots);
    }
    
    // Check each character
    for c in local.chars() {
        if !is_valid_local_char(c, allow_smtputf8) {
            return Err(EmailError::InvalidCharacter(c));
        }
    }
    
    Ok(())
}

/// Characters allowed in local part (unquoted)
#[inline]
fn is_valid_local_char(c: char, allow_smtputf8: bool) -> bool {
    match c {
        // RFC 5322 atext
        'a'..='z' | 'A'..='Z' | '0'..='9' => true,
        '!' | '#' | '$' | '%' | '&' | '\'' | '*' | '+' | '-' | '/' => true,
        '=' | '?' | '^' | '_' | '`' | '{' | '|' | '}' | '~' | '.' => true,
        // RFC 6531 UTF8 (if allowed)
        _ if allow_smtputf8 && c as u32 > 127 => !is_unsafe_unicode(c),
        _ => false,
    }
}

/// Check for unsafe unicode characters
#[inline]
fn is_unsafe_unicode(c: char) -> bool {
    // Control characters, combining marks at start, etc.
    c.is_control() || 
    ('\u{200B}'..='\u{200D}').contains(&c) ||  // Zero-width chars
    c == '\u{FEFF}'  // BOM
}
```

Create `/home/aibrush/pyval/src/domain.rs`:
```rust
//! Domain validation with IDNA 2008 support

use crate::error::EmailError;
use idna::domain_to_ascii;

pub fn validate_domain(domain: &str) -> Result<String, EmailError> {
    if domain.is_empty() {
        return Err(EmailError::InvalidDomain(domain.to_string(), "empty".to_string()));
    }
    
    // Handle IP literals [192.168.1.1] or [IPv6:...]
    if domain.starts_with('[') && domain.ends_with(']') {
        return validate_ip_literal(domain);
    }
    
    // Check length
    if domain.len() > 253 {
        return Err(EmailError::DomainTooLong);
    }
    
    // Convert to ASCII (handles IDN)
    let ascii_domain = domain_to_ascii(domain)
        .map_err(|e| EmailError::InvalidDomain(domain.to_string(), e.to_string()))?;
    
    // Validate each label
    for label in ascii_domain.split('.') {
        validate_domain_label(label)?;
    }
    
    Ok(ascii_domain)
}

fn validate_domain_label(label: &str) -> Result<(), EmailError> {
    if label.is_empty() {
        return Err(EmailError::ConsecutiveDots);
    }
    
    if label.len() > 63 {
        return Err(EmailError::InvalidDomain(
            label.to_string(),
            "label too long".to_string()
        ));
    }
    
    if label.starts_with('-') || label.ends_with('-') {
        return Err(EmailError::InvalidDomain(
            label.to_string(),
            "cannot start or end with hyphen".to_string()
        ));
    }
    
    for c in label.chars() {
        if !c.is_ascii_alphanumeric() && c != '-' {
            return Err(EmailError::InvalidCharacter(c));
        }
    }
    
    Ok(())
}

fn validate_ip_literal(domain: &str) -> Result<String, EmailError> {
    let inner = &domain[1..domain.len()-1];
    
    if let Some(ipv6) = inner.strip_prefix("IPv6:") {
        // Validate IPv6
        ipv6.parse::<std::net::Ipv6Addr>()
            .map_err(|_| EmailError::InvalidDomain(domain.to_string(), "invalid IPv6".to_string()))?;
    } else {
        // Validate IPv4
        inner.parse::<std::net::Ipv4Addr>()
            .map_err(|_| EmailError::InvalidDomain(domain.to_string(), "invalid IP".to_string()))?;
    }
    
    Ok(domain.to_string())
}
```

Create `/home/aibrush/pyval/src/validator.rs`:
```rust
//! Main email validation logic

use crate::error::EmailError;
use crate::syntax::validate_local_part;
use crate::domain::validate_domain;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone)]
pub struct ValidatedEmail {
    pub original: String,
    pub local_part: String,
    pub domain: String,
    pub normalized: String,
    pub ascii_domain: String,
    pub smtputf8: bool,
}

#[derive(Debug, Clone, Default)]
pub struct EmailValidator {
    pub allow_smtputf8: bool,
    pub allow_quoted_local: bool,
    pub allow_domain_literal: bool,
    pub check_deliverability: bool,
}

impl EmailValidator {
    pub fn new() -> Self {
        Self {
            allow_smtputf8: true,
            allow_quoted_local: false,
            allow_domain_literal: false,
            check_deliverability: false,
        }
    }
    
    pub fn validate(&self, email: &str) -> Result<ValidatedEmail, EmailError> {
        let email = email.trim();
        
        if email.is_empty() {
            return Err(EmailError::Empty);
        }
        
        // Find @ sign
        let at_count = email.matches('@').count();
        if at_count == 0 {
            return Err(EmailError::MissingAt);
        }
        if at_count > 1 {
            return Err(EmailError::MultipleAt);
        }
        
        let at_pos = email.rfind('@').unwrap();
        let local_part = &email[..at_pos];
        let domain = &email[at_pos + 1..];
        
        // Validate parts
        validate_local_part(local_part, self.allow_smtputf8)?;
        let ascii_domain = validate_domain(domain)?;
        
        // Normalize
        let normalized_local: String = local_part.nfc().collect();
        let normalized = format!("{}@{}", normalized_local, ascii_domain.to_lowercase());
        
        // Check if SMTPUTF8 is required
        let smtputf8 = local_part.chars().any(|c| c as u32 > 127);
        
        Ok(ValidatedEmail {
            original: email.to_string(),
            local_part: local_part.to_string(),
            domain: domain.to_string(),
            normalized,
            ascii_domain,
            smtputf8,
        })
    }
}
```

Create `/home/aibrush/pyval/src/lib.rs`:
```rust
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
    let validator = RustEmailValidator {
        allow_smtputf8,
        ..Default::default()
    };
    validator.validate(email).is_ok()
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
```

---

## Phase 3: Testing Strategy

### 3.1 API Compatibility Tests

Create `/home/aibrush/pyval/tests/test_api_compat.py`:
```python
#!/usr/bin/env python3
"""API compatibility tests against python-email-validator."""
import pytest
import sys
sys.path.insert(0, '/home/aibrush/pyval/test_data')

from emails import VALID_EMAILS, INVALID_EMAILS, EDGE_CASES

def get_implementations():
    """Return both implementations for comparison."""
    from email_validator import validate_email as py_validate
    from email_validator import EmailNotValidError
    try:
        import pyval
        return [
            ("python-email-validator", py_validate, EmailNotValidError),
            ("pyval", pyval.validate_email, ValueError),
        ]
    except ImportError:
        return [("python-email-validator", py_validate, EmailNotValidError)]


class TestValidEmails:
    @pytest.mark.parametrize("email", VALID_EMAILS)
    def test_valid_emails_pyval(self, email):
        """pyval should accept all valid emails."""
        try:
            import pyval
        except ImportError:
            pytest.skip("pyval not built")
        
        result = pyval.validate_email(email, check_deliverability=False)
        assert result is not None
        assert result.normalized


class TestInvalidEmails:
    @pytest.mark.parametrize("email", INVALID_EMAILS)
    def test_invalid_emails_pyval(self, email):
        """pyval should reject all invalid emails."""
        try:
            import pyval
        except ImportError:
            pytest.skip("pyval not built")
        
        with pytest.raises(ValueError):
            pyval.validate_email(email, check_deliverability=False)


class TestNormalization:
    def test_normalization_matches(self):
        """Normalization should match python-email-validator."""
        from email_validator import validate_email as py_validate
        try:
            import pyval
        except ImportError:
            pytest.skip("pyval not built")
        
        test_cases = [
            "User.Name@EXAMPLE.COM",
            "  test@example.com  ",
            "UPPERCASE@DOMAIN.COM",
        ]
        
        for email in test_cases:
            py_result = py_validate(email, check_deliverability=False)
            rust_result = pyval.validate_email(email, check_deliverability=False)
            assert py_result.normalized == rust_result.normalized, f"Mismatch for {email}"


class TestIsValid:
    def test_is_valid_function(self):
        """is_valid() should return bool."""
        try:
            import pyval
        except ImportError:
            pytest.skip("pyval not built")
        
        assert pyval.is_valid("test@example.com") is True
        assert pyval.is_valid("invalid@@email") is False
        assert pyval.is_valid("") is False


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
```

### 3.2 Performance Benchmark Tests

Create `/home/aibrush/pyval/tests/test_performance.py`:
```python
#!/usr/bin/env python3
"""Performance benchmarks comparing pyval vs python-email-validator."""
import time
import json
import statistics
from pathlib import Path
import sys

sys.path.insert(0, '/home/aibrush/pyval/test_data')
from emails import generate_bulk_emails, VALID_EMAILS

BASELINE_FILE = Path("/home/aibrush/pyval/baseline_results.json")
ITERATIONS = 1000

def load_baseline():
    with open(BASELINE_FILE) as f:
        return json.load(f)

def benchmark(func, iterations=ITERATIONS):
    times = []
    for _ in range(iterations):
        start = time.perf_counter_ns()
        func()
        end = time.perf_counter_ns()
        times.append(end - start)
    return {
        "mean_ns": statistics.mean(times),
        "median_ns": statistics.median(times),
        "min_ns": min(times),
    }

def run_comparison():
    try:
        import pyval
    except ImportError:
        print("ERROR: pyval not built. Run: maturin develop --release")
        return
    
    baseline = load_baseline()
    results = {}
    
    # Single valid email
    test_email = "user.name+tag@example.com"
    print(f"\nBenchmarking single email: {test_email}")
    
    rust_result = benchmark(lambda: pyval.validate_email(test_email, check_deliverability=False))
    orig = baseline.get("single_valid", {})
    
    if orig:
        speedup = orig["mean_ns"] / rust_result["mean_ns"]
        results["single_valid"] = {
            "original_ns": orig["mean_ns"],
            "rust_ns": rust_result["mean_ns"],
            "speedup": speedup,
            "target_met": speedup >= 100
        }
        print(f"  Single valid: {speedup:.1f}x speedup {'✓' if speedup >= 100 else '✗'}")
    
    # Single invalid email
    invalid_email = "invalid@@email"
    def validate_invalid():
        try:
            pyval.validate_email(invalid_email, check_deliverability=False)
        except ValueError:
            pass
    
    rust_result = benchmark(validate_invalid)
    orig = baseline.get("single_invalid", {})
    
    if orig:
        speedup = orig["mean_ns"] / rust_result["mean_ns"]
        results["single_invalid"] = {
            "original_ns": orig["mean_ns"],
            "rust_ns": rust_result["mean_ns"],
            "speedup": speedup,
            "target_met": speedup >= 100
        }
        print(f"  Single invalid: {speedup:.1f}x speedup {'✓' if speedup >= 100 else '✗'}")
    
    # Batch validation
    bulk_emails = generate_bulk_emails(10000)
    
    def validate_batch():
        for email in bulk_emails[:100]:
            try:
                pyval.validate_email(email, check_deliverability=False)
            except ValueError:
                pass
    
    rust_result = benchmark(validate_batch, iterations=100)
    orig = baseline.get("batch_100", {})
    
    if orig:
        speedup = orig["mean_ns"] / rust_result["mean_ns"]
        results["batch_100"] = {
            "original_ns": orig["mean_ns"],
            "rust_ns": rust_result["mean_ns"],
            "speedup": speedup,
            "target_met": speedup >= 100
        }
        print(f"  Batch (100): {speedup:.1f}x speedup {'✓' if speedup >= 100 else '✗'}")
    
    # is_valid() function (fastest path)
    print("\nBenchmarking is_valid() function:")
    rust_result = benchmark(lambda: pyval.is_valid(test_email))
    if orig:
        speedup = baseline["single_valid"]["mean_ns"] / rust_result["mean_ns"]
        results["is_valid"] = {
            "original_ns": baseline["single_valid"]["mean_ns"],
            "rust_ns": rust_result["mean_ns"],
            "speedup": speedup,
            "target_met": speedup >= 100
        }
        print(f"  is_valid(): {speedup:.1f}x speedup {'✓' if speedup >= 100 else '✗'}")
    
    # Summary
    print("\n" + "="*50)
    all_met = all(r.get("target_met", False) for r in results.values())
    if all_met:
        print("🎉 ALL TARGETS MET! 100x+ improvement achieved!")
    else:
        not_met = [k for k, v in results.items() if not v.get("target_met")]
        print(f"Targets not yet met: {not_met}")
        print("Keep optimizing!")
    
    # Save results
    with open("/home/aibrush/pyval/performance_results.json", "w") as f:
        json.dump(results, f, indent=2)
    
    return results

if __name__ == "__main__":
    run_comparison()
```

---

## Phase 4: Optimization Loop

**Run this loop continuously until 100x is achieved:**

```bash
#!/bin/bash
cd /home/aibrush/pyval

while true; do
    echo "=========================================="
    echo "Building release..."
    maturin develop --release
    
    if [ $? -ne 0 ]; then
        echo "Build failed. Fixing..."
        continue
    fi
    
    echo "=========================================="
    echo "Running tests..."
    python -m pytest tests/ -v
    
    if [ $? -ne 0 ]; then
        echo "Tests failed. Fixing..."
        continue
    fi
    
    echo "=========================================="
    echo "Running benchmarks..."
    python tests/test_performance.py
    
    # Check if 100x achieved
    if python -c "
import json
with open('performance_results.json') as f:
    results = json.load(f)
    all_met = all(r.get('target_met', False) for r in results.values())
    exit(0 if all_met else 1)
    "; then
        echo "🎉 SUCCESS! 100x+ improvement achieved!"
        break
    fi
    
    echo "Target not met. Continuing optimization..."
done
```

---

## Phase 5: Optimization Techniques

When performance is not yet 100x, apply these optimizations:

### Level 1: Low-hanging fruit
- [ ] Enable LTO and `codegen-units = 1`
- [ ] Use `--release` builds
- [ ] Avoid allocations in hot paths
- [ ] Use `&str` instead of `String` where possible

### Level 2: Algorithm optimizations
- [ ] Single-pass parsing (don't iterate string multiple times)
- [ ] Early rejection for obvious invalid patterns
- [ ] Inline small functions with `#[inline]`
- [ ] Use byte operations instead of char when ASCII

### Level 3: Advanced
- [ ] SIMD for character scanning
- [ ] Custom fast paths for common domains (gmail.com, etc.)
- [ ] Pre-compiled patterns
- [ ] Zero-allocation validation mode

### Profiling commands:
```bash
# Rust benchmarks
cargo bench

# Flamegraph
cargo install flamegraph
cargo flamegraph --release -- <benchmark>

# Python profiling
python -m cProfile -s cumtime tests/test_performance.py
```

---

## Success Criteria

You are DONE when ALL of these are true:

1. **Performance**: ≥100x speedup on all benchmarks vs python-email-validator
2. **Correctness**: 
   - All valid emails accepted
   - All invalid emails rejected
   - Normalization matches reference
3. **API**: Drop-in replacement compatible API
4. **RFC Compliance**: RFC 5322, RFC 6531 (internationalized)
5. **Features**:
   - [x] Syntax validation
   - [x] IDN support
   - [x] Normalization
   - [x] User-friendly errors
   - [ ] Deliverability checks (optional)

---

## Progress Tracking

Create /home/aibrush/pyval/PROGRESS.md` and update after each session:

```markdown
# pyval Progress

## Current Status
- [ ] Phase 0: Setup complete
- [ ] Phase 1: Baseline recorded
- [ ] Phase 2: Core implementation
  - [ ] Syntax validation
  - [ ] Domain validation
  - [ ] Normalization
  - [ ] Error messages
- [ ] Phase 3: Python bindings
- [ ] Phase 4: Tests passing
- [ ] Phase 5: 100x achieved

## Benchmark History
| Date | Single Valid | Single Invalid | Batch 100 | Notes |
|------|-------------|----------------|-----------|-------|
| YYYY-MM-DD | Xx | Xx | Xx | Initial |

## Next Actions
1. ...
```

---

## IMPORTANT RULES

1. **Never stop** - If something fails, fix it and continue
2. **Never ask for permission** - You have full sudo access
3. **Commit often** - `git commit` after each milestone
4. **Benchmark constantly** - Run benchmarks after every change
5. **Tests must pass** - Never sacrifice correctness for speed
6. **Profile before optimizing** - Don't guess bottlenecks

---

## Quick Start Commands

```bash
# Navigate to project
cd /home/aibrush/pyval
source ~/.venv/ai3.14/bin/activate
source ~/.cargo/env

# Development loop
maturin develop --release && python -m pytest tests/ -v && python tests/test_performance.py

# When stuck, profile:
cargo flamegraph --release
```

---

## BEGIN

Start now. Phase 0 first (setup project structure). Do not stop until 100x is achieved.
